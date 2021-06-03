use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::{LotteryData, TicketData},
};

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
                is_lottery_initialised,
                lottery_id,
                charity_1_id,
                charity_2_id,
                charity_3_id,
                charity_4_id,
                charity_1_vc,
                charity_2_vc,
                charity_3_vc,
                charity_4_vc,
                winner_user_wallet_pk,
                total_pool_value,
                total_registrations,
                ticket_price,
            } => {
                msg!("Instruction: InitLottery");
                Self::process_init_lottery(
                    program_id,
                    accounts,
                    is_lottery_initialised,
                    lottery_id,
                    charity_1_id,
                    charity_2_id,
                    charity_3_id,
                    charity_4_id,
                    charity_1_vc,
                    charity_2_vc,
                    charity_3_vc,
                    charity_4_vc,
                    winner_user_wallet_pk,
                    total_pool_value,
                    total_registrations,
                    ticket_price,
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
    fn process_init_lottery(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        is_lottery_initialised: bool,
        lottery_id: u32,
        charity_1_id: u32,
        charity_2_id: u32,
        charity_3_id: u32,
        charity_4_id: u32,
        charity_1_vc: u32,
        charity_2_vc: u32,
        charity_3_vc: u32,
        charity_4_vc: u32,
        winner_user_wallet_pk: [u8; 32],
        total_pool_value: u32,
        total_registrations: u32,
        ticket_price: u32,
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
        let mut lottery_data = LotteryData::try_from_slice(&lottery_data_account.data.borrow())?;
        lottery_data.is_lottery_initialised = is_lottery_initialised;
        lottery_data.lottery_id = lottery_id;
        lottery_data.charity_1_id = charity_1_id;
        lottery_data.charity_2_id = charity_2_id;
        lottery_data.charity_3_id = charity_3_id;
        lottery_data.charity_4_id = charity_4_id;
        lottery_data.charity_1_vc = charity_1_vc;
        lottery_data.charity_2_vc = charity_2_vc;
        lottery_data.charity_3_vc = charity_3_vc;
        lottery_data.charity_4_vc = charity_4_vc;
        lottery_data.winner_user_wallet_pk = winner_user_wallet_pk;
        lottery_data.total_pool_value = total_pool_value;
        lottery_data.total_registrations = total_registrations;
        lottery_data.ticket_price = ticket_price;
        lottery_data.serialize(&mut &mut lottery_data_account.data.borrow_mut()[..])?;
        msg!("data stored");
        Ok(())
    }
    fn process_ticket_purchase(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        charity_id: u32,
        user_wallet_pk: [u8; 32],
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
        let mut lottery_data = LotteryData::try_from_slice(&lottery_data_account.data.borrow())?;
        //Check if lottery initisalised
        if lottery_data.is_lottery_initialised == true {
            // Ticket data account
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
            let mut ticket_data = TicketData::try_from_slice(&ticket_data_account.data.borrow())?;
            ticket_data.charity_id = charity_id;
            ticket_data.user_wallet_pk = user_wallet_pk;
            ticket_data.ticket_number_arr = ticket_number_arr;
            ticket_data.serialize(&mut &mut ticket_data_account.data.borrow_mut()[..])?;
            lottery_data.total_pool_value =
                lottery_data.total_pool_value + lottery_data.ticket_price;
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
                } else {
                    return Err(LotteryError::InvalidCharity.into());
                }
            }
            lottery_data.serialize(&mut &mut lottery_data_account.data.borrow_mut()[..])?;
        } else {
            msg!("Lottery Not yet started, please wait!");
            return Err(ProgramError::IncorrectProgramId);
        }
        Ok(())
    }
}
