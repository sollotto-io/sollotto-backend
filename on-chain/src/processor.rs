use bs58;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::{
    instruction::LotteryInstruction,
    state::{LotteryData, TicketData},
};
use borsh::{BorshDeserialize, BorshSerialize};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_id: u8,
        lottery_id: u32,
        charity_ids: &[u32; 4],
        ticket_price: f64,
        charity_id: u32,
        user_wallet_pk: [u8; 32],
        ticket_number_arr: [u8; 6],
    ) -> ProgramResult {
        let instruction = LotteryInstruction::unpack(
            instruction_id,
            lottery_id,
            charity_ids,
            ticket_price,
            charity_id,
            user_wallet_pk,
            ticket_number_arr,
        )?;

        match instruction {
            LotteryInstruction::InitLottery {
                lottery_id,
                charity_ids,
                ticket_price,
                winner_user_wallet_pk,
                total_pool_value,
                total_registrations,
            } => {
                msg!("Instruction: InitLottery");
                Self::process_init_lottery(
                    program_id,
                    accounts,
                    lottery_id,
                    charity_ids,
                    ticket_price,
                )
            }
            LotteryInstruction::PurchaseTicket {
                charity_id: charity_id,
                user_wallet_pk: user_wallet_pk,
                ticket_number_arr: ticket_number_arr,
            } => {
                msg!("Instruction: InitLottery");
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
        lottery_id: u32,
        charity_ids: &[u32; 4],
        ticket_price: f64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        // lottery data account
        let lottery_data_account = next_account_info(accounts_iter)?;
        // Check if program owns data account
        if lottery_data_account.owner != program_id {
            msg!("Ticket Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }
        // Add data to account
        let mut lottery_data = LotteryData::try_from_slice(&lottery_data_account.data.borrow())?;
        lottery_data.is_lottery_initialised = true;
        lottery_data.lottery_id = lottery_id;
        for (pos, id) in charity_ids.iter().enumerate() {
            lottery_data.charity_ids[pos] = id;
            lottery_data.charity_vote_counts[pos] = 0;
        }
        lottery_data.ticket_price = ticket_price;
        lottery_data.serialize(&mut &mut lottery_data_account.data.borrow_mut()[..])?;
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
        // Ticket data account
        let ticket_data_account = next_account_info(accounts_iter)?;
        // lottery data account
        let lottery_data_account = next_account_info(accounts_iter)?;
        // Check if program owns data account
        if ticket_data_account.owner != program_id {
            msg!("Ticket Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }
        if lottery_data_account.owner != program_id {
            msg!("Ticket Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }
        let mut lottery_data = LotteryData::try_from_slice(&lottery_data_account.data.borrow())?;
        if lottery_data.is_lottery_initialised == true {
            // Add data to ticket account
            let mut ticket_data = TicketData::try_from_slice(&ticket_data_account.data.borrow())?;
            ticket_data.charity_id = charity_id;
            ticket_data.user_wallet_pk = user_wallet_pk;
            ticket_data.ticket_number_arr = ticket_number_arr;
            ticket_data.serialize(&mut &mut ticket_data_account.data.borrow_mut()[..])?;
            let str_user_walet_pk = bs58::encode(user_wallet_pk).into_string();
            lottery_data.total_pool_value =
                lottery_data.total_pool_value + lottery_data.ticket_price;
            lottery_data.total_registrations += 1;
            for (pos, id) in lottery_data.charity_ids.iter().enumerate() {
                if id == charity_id {
                    lottery_data.charity_vote_counts[pos] += 1;
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
