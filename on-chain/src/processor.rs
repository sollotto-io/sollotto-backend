//! Program state processor
use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::{LotteryData, TicketData},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

/// Program state handler.
pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = LotteryInstruction::unpack(instruction_data)?;

        match instruction {
            LotteryInstruction::InitLottery {
                lottery_id,
                charity_1_id,
                charity_2_id,
                charity_3_id,
                charity_4_id,
            } => {
                msg!("Instruction: InitLottery");
                Self::process_init_lottery(
                    program_id,
                    accounts,
                    lottery_id,
                    charity_1_id,
                    charity_2_id,
                    charity_3_id,
                    charity_4_id,
                )
            }

            LotteryInstruction::PurchaseTicket {
                charity_id,
                user_wallet_pk,
                ticket_number_arr,
            } => {
                msg!("Instruction: PurchaseTicket");
                Self::process_ticket_purchase(
                    program_id,
                    accounts,
                    charity_id,
                    user_wallet_pk,
                    ticket_number_arr,
                )
            }
        }
    }

    pub fn process_init_lottery(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        lottery_id: u32,
        charity_1_id: u32,
        charity_2_id: u32,
        charity_3_id: u32,
        charity_4_id: u32,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // lottery data account
        let lottery_data_account = next_account_info(accounts_iter)?;

        // Check if program owns data account
        if lottery_data_account.owner != program_id {
            msg!("Ticket Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        let rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;
        if !rent.is_exempt(
            lottery_data_account.lamports(),
            lottery_data_account.data_len(),
        ) {
            return Err(LotteryError::NotRentExempt.into());
        }

        // Add data to account
        let mut lottery_data = LotteryData::unpack_unchecked(&lottery_data_account.data.borrow())?;
        if lottery_data.is_initialized {
            return Err(LotteryError::Initialized.into());
        }

        lottery_data.is_initialized = true;
        lottery_data.lottery_id = lottery_id;
        lottery_data.charity_1_id = charity_1_id;
        lottery_data.charity_2_id = charity_2_id;
        lottery_data.charity_3_id = charity_3_id;
        lottery_data.charity_4_id = charity_4_id;
        lottery_data.charity_1_vc = 0;
        lottery_data.charity_2_vc = 0;
        lottery_data.charity_3_vc = 0;
        lottery_data.charity_4_vc = 0;
        lottery_data.total_registrations = 0;
        LotteryData::pack(lottery_data, &mut lottery_data_account.data.borrow_mut())?;

        msg!("Data stored");

        Ok(())
    }

    pub fn process_ticket_purchase(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        charity_id: u32,
        user_wallet_pk: Pubkey,
        ticket_number_arr: [u8; 6],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // lottery data account
        let lottery_data_account = next_account_info(accounts_iter)?;

        // Check if program owns lottery data account
        if lottery_data_account.owner != program_id {
            msg!("Lottery Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        let mut lottery_data = LotteryData::unpack_unchecked(&lottery_data_account.data.borrow())?;
        //Check if lottery initisalised
        if !lottery_data.is_initialized {
            return Err(LotteryError::NotInitialized.into());
        }

        let ticket_data_account = next_account_info(accounts_iter)?;
        // Check if program owns ticket data account
        if ticket_data_account.owner != program_id {
            msg!("Ticket Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        let rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;
        if !rent.is_exempt(
            lottery_data_account.lamports(),
            lottery_data_account.data_len(),
        ) && !rent.is_exempt(
            ticket_data_account.lamports(),
            ticket_data_account.data_len(),
        ) {
            return Err(LotteryError::NotRentExempt.into());
        }

        let mut ticket_data = TicketData::unpack_unchecked(&ticket_data_account.data.borrow())?;
        ticket_data.charity_id = charity_id;
        ticket_data.user_wallet_pk = user_wallet_pk;
        ticket_data.ticket_number_arr = ticket_number_arr;

        TicketData::pack(ticket_data, &mut ticket_data_account.data.borrow_mut())?;

        lottery_data.total_registrations += 1;
        let charity_arr = [
            lottery_data.charity_1_id,
            lottery_data.charity_2_id,
            lottery_data.charity_3_id,
            lottery_data.charity_4_id,
        ];
        msg!("Charity Ids: {:?}", charity_arr);
        for (pos, id) in charity_arr.iter().enumerate() {
            msg!("Entered Loop");
            msg!("Current Charity: {}", *id);
            msg!("Receieved Charity: {}", charity_id);
            if *id == charity_id {
                msg!("Matched ID Loop");
                match pos {
                    0 => lottery_data.charity_1_vc += 1,
                    1 => lottery_data.charity_2_vc += 1,
                    2 => lottery_data.charity_3_vc += 1,
                    3 => lottery_data.charity_4_vc += 1,
                    _ => return Err(LotteryError::InvalidCharity.into()),
                }
                break;
            }
        }

        LotteryData::pack(lottery_data, &mut lottery_data_account.data.borrow_mut())?;

        Ok(())
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{
        instruction::Instruction, program_pack::Pack,
    };
    use solana_sdk::account::{
        create_account_for_test, create_is_signer_account_infos, Account as SolanaAccount,
    };

    fn init_acc_minimum_balance() -> u64 {
        Rent::default().minimum_balance(LotteryData::get_packed_len())
    }

    fn do_process(instruction: Instruction, accounts: Vec<&mut SolanaAccount>) -> ProgramResult {
        let mut meta = instruction
            .accounts
            .iter()
            .zip(accounts)
            .map(|(account_meta, account)| (&account_meta.pubkey, account_meta.is_signer, account))
            .collect::<Vec<_>>();

        let account_infos = create_is_signer_account_infos(&mut meta);
        Processor::process(&instruction.program_id, &account_infos, &instruction.data)
    }

    #[test]
    fn test_init_lottery() {
        let program_id = id();
        let lottery_key = Pubkey::new_unique();
        let mut lottery_account = SolanaAccount::new(
            init_acc_minimum_balance(),
            LotteryData::get_packed_len(),
            &program_id,
        );
        let mut rent_sysvar_account = create_account_for_test(&Rent::default());
        let collateral_key = Pubkey::new_unique();
        let oracles = vec![Pubkey::new_unique(), Pubkey::new_unique()];

        let mut bad_sync_acc = SolanaAccount::new(
            init_acc_minimum_balance() - 100,
            LotteryData::get_packed_len(),
            &program_id,
        );
        assert_eq!(
            Err(LotteryError::NotRentExempt.into()),
            do_process(
                crate::instruction::initialize_lottery(&id(), 1, 1, 2, 3, 4, &lottery_key,)
                    .unwrap(),
                vec![&mut bad_sync_acc, &mut rent_sysvar_account]
            )
        );
    }
}
