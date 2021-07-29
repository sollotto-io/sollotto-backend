//! Program state processor
use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    native_token::{lamports_to_sol, sol_to_lamports},
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
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
            LotteryInstruction::PurchaseTicket { amount } => {
                msg!("Instruction: PurchaseTicket");
                Self::process_ticket_purchase(
                    program_id,
                    accounts,
                    amount
                )
            },
            LotteryInstruction::RewardWinners { idx, prize_pool } => {
                msg!("Instruction: RewardWinners");
                Self::process_reward_winners(
                    program_id, accounts,
                    idx, prize_pool
                )
            }
        }
    }

    pub fn process_ticket_purchase(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u32
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_fqticket_account   = next_account_info(accounts_iter)?;
        let user_sol_account        = next_account_info(accounts_iter)?;
        let user_slot_account       = next_account_info(accounts_iter)?;
        let fqticket_mint           = next_account_info(accounts_iter)?;
        let fqticket_mint_authority = next_account_info(accounts_iter)?;
        let sollotto_sol_account    = next_account_info(accounts_iter)?;
        let system_program_account  = next_account_info(accounts_iter)?;
        let spl_token_account       = next_account_info(accounts_iter)?;

        let ticket_price = sol_to_lamports(0.1);
        let total_price = ticket_price * (amount as u64);

        if user_sol_account.lamports() < total_price {
            msg!("User cannot pay for ticket");
            return Err(ProgramError::InsufficientFunds);
        }

        // TODO: Check for signed/writable attributes on fields that require them,
        // Identify and decline transactions that try to spoof user SLOT balance.
        // This can be achieved by ensuring that user's SLOT account has corrent mint

        // Take buyers SOL tokens.
        invoke(
            &system_instruction::transfer(
                user_sol_account.key,
                sollotto_sol_account.key,
                total_price
            ),
            &[
                user_sol_account.clone(),
                sollotto_sol_account.clone(),
                system_program_account.clone()
            ]
        )?;

        // Mint the corresponding Fixed-Quantity Tokens for taken SOL.
        invoke(
            &spl_token::instruction::mint_to(
                spl_token_account.key,
                fqticket_mint.key,
                user_slot_account.key,
                spl_token_account.owner,
                &[],
                amount as u64
            )?,
            &[
                fqticket_mint.clone(),
                user_fqticket_account.clone(),
                fqticket_mint_authority.clone()
            ]
        )?;

        Ok(())
    }

    pub fn process_reward_winners(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        idx: u64,
        prize_pool: u64
    ) -> ProgramResult {
        Ok(())
    }

    pub fn write_winning_address(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        winning_addr: Pubkey,
    ) -> ProgramResult {
        // FIXME
        Ok(())
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{instruction::Instruction, program_pack::Pack};
    use solana_sdk::account::{
        create_account_for_test, create_is_signer_account_infos, Account as SolanaAccount,
        ReadableAccount,
    };

    fn lottery_minimum_balance() -> u64 {
        Rent::default().minimum_balance(LotteryData::get_packed_len())
    }

    fn ticket_minimum_balance() -> u64 {
        Rent::default().minimum_balance(TicketData::get_packed_len())
    }

    fn lottery_result_minimum_balance() -> u64 {
        Rent::default().minimum_balance(LotteryResultData::get_packed_len())
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
}
