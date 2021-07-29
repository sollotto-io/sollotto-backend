//! Program state processor
use crate::{
    check_program_account, error::LotteryError, instruction::LotteryInstruction,
    state::LotteryResultData,
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
            LotteryInstruction::Deposit { amount } => {
                msg!("Instruction: Deposit");
                Self::process_deposit(program_id, accounts, amount)
            }

            LotteryInstruction::Unpool { amount } => {
                msg!("Instruction: Unpool");
                Self::process_unpool(program_id, accounts, amount)
            }

            LotteryInstruction::RewardWinner {
                lottery_id,
                random_number,
            } => {
                msg!("Instruction: reward winner");
                Self::process_reward_winner(program_id, accounts, lottery_id, random_number)
            }
        }
    }

    pub fn process_deposit(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user_wallet = next_account_info(accounts_iter)?;
        let sollotto_staking_pool_authority = next_account_info(accounts_iter)?;
        let user_token_acc = next_account_info(accounts_iter)?;
        let user_staking_pool_token_acc = next_account_info(accounts_iter)?;
        let staking_pool_token_acc = next_account_info(accounts_iter)?;
        let staking_pool_token_mint = next_account_info(accounts_iter)?;
        let spl_token_info = next_account_info(accounts_iter)?;

        if !user_wallet.is_signer {
            msg!("Missing user wallet signature");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !sollotto_staking_pool_authority.is_signer {
            msg!("Missing user wallet signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        // Transfer Custom SPL Token from user to staking pool
        invoke(
            &spl_token::instruction::transfer(
                &spl_token::id(),
                user_token_acc.key,
                staking_pool_token_acc.key,
                user_wallet.key,
                &[],
                amount,
            )
            .unwrap(),
            &[
                spl_token_info.clone(),
                user_token_acc.clone(),
                staking_pool_token_acc.clone(),
                user_wallet.clone(),
            ],
        )?;

        // Mint SPL Token Staking pool token to user
        invoke(
            &spl_token::instruction::mint_to(
                &spl_token::id(),
                staking_pool_token_mint.key,
                user_staking_pool_token_acc.key,
                sollotto_staking_pool_authority.key,
                &[],
                amount,
            )
            .unwrap(),
            &[
                spl_token_info.clone(),
                staking_pool_token_mint.clone(),
                user_staking_pool_token_acc.clone(),
                sollotto_staking_pool_authority.clone(),
            ],
        )?;

        Ok(())
    }

    pub fn process_unpool(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // TODO: Check access
        // TODO: Burn SPL Token Staking pool token from user
        // TODO: transfer SPL Token from staking pool to user

        Ok(())
    }

    pub fn process_reward_winner(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        lottery_id: u32,
        random_number: u32,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // TODO: check access
        // TODO: Find winner in participant list
        // TODO: Get the reward shares
        // TODO: Transfer reward
        // TODO: Save result on-chain

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
    };
    use spl_token::state::{Account, Mint};

    fn lottery_result_minimum_balance() -> u64 {
        Rent::default().minimum_balance(LotteryResultData::get_packed_len())
    }

    fn mint_minimum_balance() -> u64 {
        Rent::default().minimum_balance(Mint::get_packed_len())
    }

    fn account_minimum_balance() -> u64 {
        Rent::default().minimum_balance(Account::get_packed_len())
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
    fn test_deposit() {
        // TODO
    }

    #[test]
    fn test_unpool() {
        // TODO
    }

    #[test]
    fn test_reward_winner() {
        // TODO
    }
}
