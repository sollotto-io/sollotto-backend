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
        let sollotto_token_acc = next_account_info(accounts_iter)?;
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
                sollotto_token_acc.key,
                user_wallet.key,
                &[],
                amount,
            )
            .unwrap(),
            &[
                spl_token_info.clone(),
                user_token_acc.clone(),
                sollotto_token_acc.clone(),
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
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let user_wallet = next_account_info(accounts_iter)?;
        let sollotto_staking_pool_authority = next_account_info(accounts_iter)?;
        let user_token_acc = next_account_info(accounts_iter)?;
        let user_staking_pool_token_acc = next_account_info(accounts_iter)?;
        let sollotto_token_acc = next_account_info(accounts_iter)?;
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

        // Burn SPL Token Staking pool token from user
        invoke(
            &spl_token::instruction::burn(
                &spl_token::id(),
                user_staking_pool_token_acc.key,
                staking_pool_token_mint.key,
                user_wallet.key,
                &[],
                amount,
            )
            .unwrap(),
            &[
                spl_token_info.clone(),
                user_staking_pool_token_acc.clone(),
                staking_pool_token_mint.clone(),
                user_wallet.clone(),
            ],
        )?;

        // Transfer Custom SPL Token from staking pool to user
        invoke(
            &spl_token::instruction::transfer(
                &spl_token::id(),
                sollotto_token_acc.key,
                user_token_acc.key,
                sollotto_staking_pool_authority.key,
                &[],
                amount,
            )
            .unwrap(),
            &[
                spl_token_info.clone(),
                sollotto_token_acc.clone(),
                user_token_acc.clone(),
                sollotto_staking_pool_authority.clone(),
            ],
        )?;

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
        // TODO: rent check
        // TODO: check all participants (mint + amount > 0)
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
    use solana_program::instruction::Instruction;
    use solana_sdk::account::{create_is_signer_account_infos, Account as SolanaAccount};
    use spl_token::ui_amount_to_amount;

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
        let program_id = crate::id();
        let mut spl_token_acc = SolanaAccount::default();
        let user_wallet_key = Pubkey::new_unique();
        let mut user_wallet_acc = SolanaAccount::default();
        let staking_pool_key = Pubkey::new_unique();
        let mut staking_pool_acc = SolanaAccount::default();
        let staking_pool_token_key = Pubkey::new_unique();
        let mut staking_pool_token_acc = SolanaAccount::default();

        let user_token_account_key = Pubkey::new_unique();
        let mut user_token_account_acc = SolanaAccount::default();
        let user_staking_pool_token_key = Pubkey::new_unique();
        let mut user_staking_pool_token_acc = SolanaAccount::default();

        let staking_pool_token_mint_key = Pubkey::new_unique();
        let mut staking_pool_token_mint_acc = SolanaAccount::default();
        let decimals = 9;

        let amount = ui_amount_to_amount(1.0, decimals);

        do_process(
            crate::instruction::deposit(
                &program_id,
                amount,
                &user_wallet_key,
                &staking_pool_key,
                &user_token_account_key,
                &user_staking_pool_token_key,
                &staking_pool_token_key,
                &staking_pool_token_mint_key,
            )
            .unwrap(),
            vec![
                &mut user_wallet_acc,
                &mut staking_pool_acc,
                &mut user_token_account_acc,
                &mut user_staking_pool_token_acc,
                &mut staking_pool_token_acc,
                &mut staking_pool_token_mint_acc,
                &mut spl_token_acc,
            ],
        )
        .unwrap();
    }

    #[test]
    fn test_unpool() {
        let program_id = crate::id();
        let mut spl_token_acc = SolanaAccount::default();
        let user_wallet_key = Pubkey::new_unique();
        let mut user_wallet_acc = SolanaAccount::default();
        let staking_pool_key = Pubkey::new_unique();
        let mut staking_pool_acc = SolanaAccount::default();
        let staking_pool_token_key = Pubkey::new_unique();
        let mut staking_pool_token_acc = SolanaAccount::default();

        let user_token_account_key = Pubkey::new_unique();
        let mut user_token_account_acc = SolanaAccount::default();
        let user_staking_pool_token_key = Pubkey::new_unique();
        let mut user_staking_pool_token_acc = SolanaAccount::default();

        let staking_pool_token_mint_key = Pubkey::new_unique();
        let mut staking_pool_token_mint_acc = SolanaAccount::default();
        let decimals = 9;

        let amount = ui_amount_to_amount(1.0, decimals);

        do_process(
            crate::instruction::unpool(
                &program_id,
                amount,
                &user_wallet_key,
                &staking_pool_key,
                &user_token_account_key,
                &user_staking_pool_token_key,
                &staking_pool_token_key,
                &staking_pool_token_mint_key,
            )
            .unwrap(),
            vec![
                &mut user_wallet_acc,
                &mut staking_pool_acc,
                &mut user_token_account_acc,
                &mut user_staking_pool_token_acc,
                &mut staking_pool_token_acc,
                &mut staking_pool_token_mint_acc,
                &mut spl_token_acc,
            ],
        )
        .unwrap();
    }

    #[test]
    fn test_reward_winner() {
        // TODO
    }
}
