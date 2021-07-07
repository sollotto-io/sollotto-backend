//! Program state processor
use crate::{error::LotteryError, instruction::LotteryInstruction, state::LotteryData};
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
                staking_pool_wallet,
                staking_pool_token_mint,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
            } => {
                msg!("Instruction: InitLottery");
                Self::process_init_lottery(
                    program_id,
                    accounts,
                    staking_pool_wallet,
                    staking_pool_token_mint,
                    rewards_wallet,
                    slot_holders_rewards_wallet,
                    sollotto_labs_wallet,
                )
            }

            LotteryInstruction::Deposit { amount } => {
                msg!("Instruction: Deposit");
                Self::process_deposit(program_id, accounts, amount)
            }

            LotteryInstruction::Undeposit { amount } => {
                msg!("Instruction: Undeposit");
                Self::process_undeposit(program_id, accounts, amount)
            }

            LotteryInstruction::RewardWinner {} => {
                msg!("Instruction: reward winners");
                Self::process_reward_winner(program_id, accounts)
            }

            LotteryInstruction::UpdateLotteryWallets {
                staking_pool_wallet,
                staking_pool_token_mint,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
            } => {
                msg!("Instruction: update lottery wallets");
                Self::process_update_lottery_wallets(
                    program_id,
                    accounts,
                    staking_pool_wallet,
                    staking_pool_token_mint,
                    rewards_wallet,
                    slot_holders_rewards_wallet,
                    sollotto_labs_wallet,
                )
            }
        }
    }

    pub fn process_init_lottery(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        staking_pool_wallet: Pubkey,
        staking_pool_token_mint: Pubkey,
        rewards_wallet: Pubkey,
        slot_holders_rewards_wallet: Pubkey,
        sollotto_labs_wallet: Pubkey,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let lottery_account = next_account_info(accounts_iter)?;
        let rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;

        if lottery_account.owner != program_id {
            msg!("Lottery Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        if !lottery_account.is_signer {
            msg!("Missing lottery data account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !rent.is_exempt(lottery_account.lamports(), lottery_account.data_len()) {
            return Err(LotteryError::NotRentExempt.into());
        }

        let mut lottery_data = LotteryData::unpack_unchecked(&lottery_account.data.borrow())?;
        if lottery_data.is_initialized {
            msg!("Lottery data account already initialized");
            return Err(LotteryError::Initialized.into());
        }

        lottery_data.is_initialized = true;
        lottery_data.staking_pool_amount = 0;
        lottery_data.staking_pool_wallet = staking_pool_wallet;
        lottery_data.staking_pool_token_mint = staking_pool_token_mint;
        lottery_data.rewards_wallet = rewards_wallet;
        lottery_data.slot_holders_rewards_wallet = slot_holders_rewards_wallet;
        lottery_data.sollotto_labs_wallet = sollotto_labs_wallet;

        LotteryData::pack(lottery_data, &mut lottery_account.data.borrow_mut())?;

        Ok(())
    }

    pub fn process_deposit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // TODO
        // Check access
        // Check user funds
        // Transfer amount SOL from user to staking_pool_wallet
        // Mint amount staking_pool_token_mint to user associated account

        Ok(())
    }

    pub fn process_undeposit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // TODO

        Ok(())
    }

    pub fn process_reward_winner(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // TODO

        Ok(())
    }

    pub fn process_update_lottery_wallets(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        staking_pool_wallet: Pubkey,
        staking_pool_token_mint: Pubkey,
        rewards_wallet: Pubkey,
        slot_holders_rewards_wallet: Pubkey,
        sollotto_labs_wallet: Pubkey,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let lottery_account = next_account_info(accounts_iter)?;

        if lottery_account.owner != program_id {
            msg!("Lottery Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        if !lottery_account.is_signer {
            msg!("Missing lottery data account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut lottery_data = LotteryData::unpack_unchecked(&lottery_account.data.borrow())?;
        if !lottery_data.is_initialized {
            msg!("Lottery data account is not initialized");
            return Err(LotteryError::NotInitialized.into());
        }

        lottery_data.staking_pool_wallet = staking_pool_wallet;
        lottery_data.staking_pool_token_mint = staking_pool_token_mint;
        lottery_data.rewards_wallet = rewards_wallet;
        lottery_data.slot_holders_rewards_wallet = slot_holders_rewards_wallet;
        lottery_data.sollotto_labs_wallet = sollotto_labs_wallet;

        LotteryData::pack(lottery_data, &mut lottery_account.data.borrow_mut())?;

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

    // fn lottery_minimum_balance() -> u64 {
    //     Rent::default().minimum_balance(LotteryData::get_packed_len())
    // }

    // fn lottery_result_minimum_balance() -> u64 {
    //     Rent::default().minimum_balance(LotteryResultData::get_packed_len())
    // }

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

    // TODO: unit tests
}
