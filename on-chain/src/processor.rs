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

// Sollotto program_id
solana_program::declare_id!("urNhxed8ocNiFApoooLSAJ1xnWSMUiC9S6fKcRon1rk");

/// Checks that the supplied program ID is the correct
pub fn check_program_account(program_id: &Pubkey) -> ProgramResult {
    if program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

/// Program state handler.
pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        check_program_account(program_id)?;

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

            LotteryInstruction::StoreWinningNumbers {
                winning_numbers_arr,
            } => {
                msg!("Instruction: store winning numbers");
                Self::process_store_winning_numbers(program_id, accounts, winning_numbers_arr)
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

        if !lottery_data_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
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

        if !lottery_data_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
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
        ) {
            return Err(LotteryError::NotRentExempt.into());
        }

        if !rent.is_exempt(
            ticket_data_account.lamports(),
            ticket_data_account.data_len(),
        ) {
            return Err(LotteryError::NotRentExempt.into());
        }

        for i in 0..5 {
            if ticket_number_arr[i] < 1 || ticket_number_arr[i] > 69 {
                return Err(LotteryError::InvalidNumber.into());
            }
        }
        if ticket_number_arr[5] < 1 || ticket_number_arr[5] > 29 {
            return Err(LotteryError::InvalidNumber.into());
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

    pub fn process_store_winning_numbers(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        winning_numbers_arr: [u8; 6],
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let lottery_data_account = next_account_info(accounts_iter)?;

        if lottery_data_account.owner != program_id {
            msg!("Lottery Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        if !lottery_data_account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut lottery_data = LotteryData::unpack(&lottery_data_account.data.borrow())?;
        if !lottery_data.is_initialized {
            msg!("Lottery Data account is not initialized");
            return Err(LotteryError::NotInitialized.into());
        }

        for i in 0..5 {
            if winning_numbers_arr[i] < 1 || winning_numbers_arr[i] > 69 {
                return Err(LotteryError::InvalidNumber.into());
            }
        }
        if winning_numbers_arr[5] < 1 || winning_numbers_arr[5] > 29 {
            return Err(LotteryError::InvalidNumber.into());
        }

        lottery_data.winning_numbers = winning_numbers_arr;

        LotteryData::pack(lottery_data, &mut lottery_data_account.data.borrow_mut())?;

        Ok(())
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{instruction::Instruction, program_pack::Pack};
    use solana_sdk::account::{Account as SolanaAccount, ReadableAccount, create_account_for_test, create_is_signer_account_infos};

    fn lottery_minimum_balance() -> u64 {
        Rent::default().minimum_balance(LotteryData::get_packed_len())
    }

    fn ticket_minimum_balance() -> u64 {
        Rent::default().minimum_balance(TicketData::get_packed_len())
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
        let lottery_id = 112233;
        let lottery_key = Pubkey::new_unique();
        let mut lottery_acc = SolanaAccount::new(
            lottery_minimum_balance(),
            LotteryData::get_packed_len(),
            &program_id,
        );
        let mut rent_sysvar_acc = create_account_for_test(&Rent::default());

        // BadCase: rent NotRentExempt
        let mut bad_lottery_acc = SolanaAccount::new(
            lottery_minimum_balance() - 100,
            LotteryData::get_packed_len(),
            &program_id,
        );
        assert_eq!(
            Err(LotteryError::NotRentExempt.into()),
            do_process(
                crate::instruction::initialize_lottery(
                    &program_id,
                    lottery_id,
                    1,
                    2,
                    3,
                    4,
                    &lottery_key
                )
                .unwrap(),
                vec![&mut bad_lottery_acc, &mut rent_sysvar_acc]
            )
        );

        do_process(
            crate::instruction::initialize_lottery(
                &program_id,
                lottery_id,
                1,
                2,
                3,
                4,
                &lottery_key,
            )
            .unwrap(),
            vec![&mut lottery_acc, &mut rent_sysvar_acc],
        )
        .unwrap();

        // BadCase: Lottery Already initialized
        assert_eq!(
            Err(LotteryError::Initialized.into()),
            do_process(
                crate::instruction::initialize_lottery(
                    &program_id,
                    lottery_id,
                    1,
                    2,
                    3,
                    4,
                    &lottery_key,
                )
                .unwrap(),
                vec![&mut lottery_acc, &mut rent_sysvar_acc]
            )
        );

        let lottery = LotteryData::unpack(&lottery_acc.data).unwrap();
        assert_eq!(lottery.is_initialized, true);
        assert_eq!(lottery.lottery_id, lottery_id);
        assert_eq!(lottery.charity_1_id, 1);
        assert_eq!(lottery.charity_2_id, 2);
        assert_eq!(lottery.charity_3_id, 3);
        assert_eq!(lottery.charity_4_id, 4);
        assert_eq!(lottery.charity_1_vc, 0);
        assert_eq!(lottery.charity_2_vc, 0);
        assert_eq!(lottery.charity_3_vc, 0);
        assert_eq!(lottery.charity_4_vc, 0);
        assert_eq!(lottery.total_registrations, 0);
        for number in &lottery.winning_numbers {
            assert_eq!(*number, 0);
        }
    }

    #[test]
    fn test_ticket_purchase() {
        let program_id = id();
        let lottery_id = 112233;
        let lottery_key = Pubkey::new_unique();
        let mut lottery_acc = SolanaAccount::new(
            lottery_minimum_balance(),
            LotteryData::get_packed_len(),
            &program_id,
        );
        let user_key = Pubkey::new_unique();
        let mut user_ticket_acc = SolanaAccount::new(
            ticket_minimum_balance(),
            TicketData::get_packed_len(),
            &program_id,
        );
        let mut rent_sysvar_acc = create_account_for_test(&Rent::default());
        let charity_id = 1;

        // BadCase: Lottery is not initialized
        assert_eq!(
            Err(LotteryError::NotInitialized.into()),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    charity_id,
                    &user_key,
                    &[10, 20, 30, 40, 50, 15],
                    &lottery_key,
                )
                .unwrap(),
                vec![&mut lottery_acc, &mut user_ticket_acc, &mut rent_sysvar_acc]
            )
        );

        do_process(
            crate::instruction::initialize_lottery(
                &program_id,
                lottery_id,
                1,
                2,
                3,
                4,
                &lottery_key,
            )
            .unwrap(),
            vec![&mut lottery_acc, &mut rent_sysvar_acc],
        )
        .unwrap();

        // BadCase: rent NotRentExempt
        let mut bad_ticket_acc = SolanaAccount::new(
            ticket_minimum_balance() - 100,
            TicketData::get_packed_len(),
            &program_id,
        );
        assert_eq!(
            Err(LotteryError::NotRentExempt.into()),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    charity_id,
                    &user_key,
                    &[10, 20, 30, 40, 50, 15],
                    &lottery_key,
                )
                .unwrap(),
                vec![&mut lottery_acc, &mut bad_ticket_acc, &mut rent_sysvar_acc]
            )
        );

        // BadCase: bad numbers
        assert_eq!(
            Err(LotteryError::InvalidNumber.into()),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    charity_id,
                    &user_key,
                    &[70, 20, 30, 40, 50, 15],
                    &lottery_key,
                )
                .unwrap(),
                vec![&mut lottery_acc, &mut user_ticket_acc, &mut rent_sysvar_acc]
            )
        );

        assert_eq!(
            Err(LotteryError::InvalidNumber.into()),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    charity_id,
                    &user_key,
                    &[10, 20, 30, 40, 0, 15],
                    &lottery_key,
                )
                .unwrap(),
                vec![&mut lottery_acc, &mut user_ticket_acc, &mut rent_sysvar_acc]
            )
        );

        assert_eq!(
            Err(LotteryError::InvalidNumber.into()),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    charity_id,
                    &user_key,
                    &[10, 20, 30, 40, 50, 30],
                    &lottery_key,
                )
                .unwrap(),
                vec![&mut lottery_acc, &mut user_ticket_acc, &mut rent_sysvar_acc]
            )
        );

        do_process(
            crate::instruction::purchase_ticket(
                &program_id,
                charity_id,
                &user_key,
                &[10, 20, 30, 40, 50, 29],
                &lottery_key,
            )
            .unwrap(),
            vec![&mut lottery_acc, &mut user_ticket_acc, &mut rent_sysvar_acc],
        )
        .unwrap();

        let lottery = LotteryData::unpack(&lottery_acc.data()).unwrap();
        assert_eq!(lottery.charity_1_vc, 1);
        assert_eq!(lottery.total_registrations, 1);
    }
}
