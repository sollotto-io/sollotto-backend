//! Program state processor
use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::{LotteryData, LotteryResultData},
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
            LotteryInstruction::InitLottery {
                staking_pool_wallet,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
            } => {
                msg!("Instruction: InitLottery");
                Self::process_init_lottery(
                    program_id,
                    accounts,
                    staking_pool_wallet,
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

            LotteryInstruction::RewardWinner { lottery_id } => {
                msg!("Instruction: reward winners");
                Self::process_reward_winner(program_id, accounts, lottery_id)
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
        rewards_wallet: Pubkey,
        slot_holders_rewards_wallet: Pubkey,
        sollotto_labs_wallet: Pubkey,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let lottery_account = next_account_info(accounts_iter)?;
        let staking_pool_token_mint = next_account_info(accounts_iter)?;
        let staking_pool_token_account = next_account_info(accounts_iter)?;
        let rent_account = next_account_info(accounts_iter)?;
        let spl_token_account = next_account_info(accounts_iter)?;

        if lottery_account.owner != program_id {
            msg!("Lottery Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        if !lottery_account.is_signer {
            msg!("Missing lottery data account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let rent = &Rent::from_account_info(rent_account)?;
        if !rent.is_exempt(lottery_account.lamports(), lottery_account.data_len()) {
            return Err(LotteryError::NotRentExempt.into());
        }

        let mut lottery_data = LotteryData::unpack_unchecked(&lottery_account.data.borrow())?;
        if lottery_data.is_initialized {
            msg!("Lottery data account already initialized");
            return Err(LotteryError::Initialized.into());
        }

        // Initialize staking pool token Mint
        invoke(
            &spl_token::instruction::initialize_mint(
                &spl_token::id(),
                &staking_pool_token_mint.key,
                &lottery_account.key,
                None,
                9,
            )
            .unwrap(),
            &[
                spl_token_account.clone(),
                staking_pool_token_mint.clone(),
                rent_account.clone(),
            ],
        )?;

        // Initialize token associated account
        invoke(
            &spl_token::instruction::initialize_account(
                &spl_token::id(),
                &staking_pool_token_account.key,
                &staking_pool_token_mint.key,
                &lottery_account.key,
            )
            .unwrap(),
            &[
                spl_token_account.clone(),
                staking_pool_token_account.clone(),
                staking_pool_token_mint.clone(),
                lottery_account.clone(),
                rent_account.clone(),
            ],
        )?;

        lottery_data.is_initialized = true;
        lottery_data.staking_pool_amount = 0;
        lottery_data.staking_pool_wallet = staking_pool_wallet;
        lottery_data.staking_pool_token_mint = *staking_pool_token_mint.key;
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
        let lottery_account = next_account_info(accounts_iter)?;
        let staking_pool_token_mint = next_account_info(accounts_iter)?;
        let user_funding_account = next_account_info(accounts_iter)?;
        let user_staking_pool_token_account = next_account_info(accounts_iter)?;
        let sollotto_staking_pool_wallet = next_account_info(accounts_iter)?;
        let spl_token_account = next_account_info(accounts_iter)?;
        let system_program_account = next_account_info(accounts_iter)?;

        if lottery_account.owner != program_id {
            msg!("Lottery Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        if !lottery_account.is_signer {
            msg!("Missing lottery data account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !user_funding_account.is_signer {
            msg!("Missing user account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut lottery_data = LotteryData::unpack_unchecked(&lottery_account.data.borrow())?;
        if !lottery_data.is_initialized {
            msg!("Lottery data account is not initialized");
            return Err(LotteryError::NotInitialized.into());
        }

        if lottery_data.staking_pool_token_mint != *staking_pool_token_mint.key {
            msg!("Invalid staking pool token mint");
            return Err(LotteryError::InvalidSollottoAccount.into());
        }
        if lottery_data.staking_pool_wallet != *sollotto_staking_pool_wallet.key {
            msg!("Invalid staking pool wallet");
            return Err(LotteryError::InvalidSollottoAccount.into());
        }

        // TODO: transfer to liquidity pool?
        // Transfer SOL from user to staking_pool_wallet
        invoke(
            &system_instruction::transfer(
                &user_funding_account.key,
                &sollotto_staking_pool_wallet.key,
                amount,
            ),
            &[
                user_funding_account.clone(),
                sollotto_staking_pool_wallet.clone(),
                system_program_account.clone(),
            ],
        )?;

        // Mint amount staking_pool_token_mint to user associated account
        invoke(
            &spl_token::instruction::mint_to(
                &spl_token::id(),
                &staking_pool_token_mint.key,
                &user_staking_pool_token_account.key,
                &lottery_account.key,
                &[],
                amount,
            )?,
            &[
                spl_token_account.clone(),
                staking_pool_token_mint.clone(),
                user_staking_pool_token_account.clone(),
                lottery_account.clone(),
            ],
        )?;

        // Save information in lottery data account
        lottery_data.staking_pool_amount += amount;

        LotteryData::pack(lottery_data, &mut lottery_account.data.borrow_mut())?;

        Ok(())
    }

    pub fn process_undeposit(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let lottery_account = next_account_info(accounts_iter)?;
        let staking_pool_token_mint = next_account_info(accounts_iter)?;
        let user_funding_account = next_account_info(accounts_iter)?;
        let user_staking_pool_token_account = next_account_info(accounts_iter)?;
        let sollotto_staking_pool_wallet = next_account_info(accounts_iter)?;
        let spl_token_account = next_account_info(accounts_iter)?;
        let system_program_account = next_account_info(accounts_iter)?;

        if lottery_account.owner != program_id {
            msg!("Lottery Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        if !lottery_account.is_signer {
            msg!("Missing lottery data account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !user_funding_account.is_signer {
            msg!("Missing user account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !sollotto_staking_pool_wallet.is_signer {
            msg!("Missing staking pool wallet signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut lottery_data = LotteryData::unpack_unchecked(&lottery_account.data.borrow())?;
        if !lottery_data.is_initialized {
            msg!("Lottery data account is not initialized");
            return Err(LotteryError::NotInitialized.into());
        }

        if lottery_data.staking_pool_token_mint != *staking_pool_token_mint.key {
            msg!("Invalid staking pool token mint");
            return Err(LotteryError::InvalidSollottoAccount.into());
        }
        if lottery_data.staking_pool_wallet != *sollotto_staking_pool_wallet.key {
            msg!("Invalid staking pool wallet");
            return Err(LotteryError::InvalidSollottoAccount.into());
        }

        if amount > lottery_data.staking_pool_amount {
            msg!("Lottery staking pool insufficient funds");
            return Err(ProgramError::InsufficientFunds);
        }

        // Burn amout of staking pool tokens from user associated account
        invoke(
            &spl_token::instruction::burn(
                &spl_token::id(),
                &user_staking_pool_token_account.key,
                &staking_pool_token_mint.key,
                &user_funding_account.key,
                &[],
                amount,
            )?,
            &[
                spl_token_account.clone(),
                staking_pool_token_mint.clone(),
                user_staking_pool_token_account.clone(),
                user_funding_account.clone(),
            ],
        )?;

        // TODO: work with liquidity pool here
        // Transfer amount of SOL from staking pool wallet to user wallet
        invoke(
            &system_instruction::transfer(
                &sollotto_staking_pool_wallet.key,
                &user_funding_account.key,
                amount,
            ),
            &[
                sollotto_staking_pool_wallet.clone(),
                user_funding_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        // Update information in lottery data account
        lottery_data.staking_pool_amount -= amount;

        LotteryData::pack(lottery_data, &mut lottery_account.data.borrow_mut())?;

        Ok(())
    }

    pub fn process_reward_winner(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        lottery_id: u32,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let lottery_account = next_account_info(accounts_iter)?;
        let lottery_result_account = next_account_info(accounts_iter)?;
        let winner_account = next_account_info(accounts_iter)?;
        let sollotto_staking_pool_wallet = next_account_info(accounts_iter)?;
        let sollotto_reward_wallet = next_account_info(accounts_iter)?;
        let slot_holders_wallet = next_account_info(accounts_iter)?;
        let sollotto_labs_wallet = next_account_info(accounts_iter)?;
        let system_program_account = next_account_info(accounts_iter)?;

        if lottery_account.owner != program_id {
            msg!("Lottery Data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }
        if lottery_result_account.owner != program_id {
            msg!("Lottery result data account does not have the correct program id");
            return Err(ProgramError::IncorrectProgramId);
        }

        if !lottery_account.is_signer {
            msg!("Missing lottery data account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !sollotto_staking_pool_wallet.is_signer {
            msg!("Missing user account signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let lottery_data = LotteryData::unpack_unchecked(&lottery_account.data.borrow())?;
        if !lottery_data.is_initialized {
            msg!("Lottery data account is not initialized");
            return Err(LotteryError::NotInitialized.into());
        }

        if lottery_data.staking_pool_wallet != *sollotto_staking_pool_wallet.key {
            msg!("Invalid staking pool wallet");
            return Err(LotteryError::InvalidSollottoAccount.into());
        }
        if lottery_data.rewards_wallet != *sollotto_reward_wallet.key {
            msg!("Invalid sollotto foundation rewards wallet");
            return Err(LotteryError::InvalidSollottoAccount.into());
        }
        if lottery_data.slot_holders_rewards_wallet != *slot_holders_wallet.key {
            msg!("Invalid SLOT holders wallet");
            return Err(LotteryError::InvalidSollottoAccount.into());
        }
        if lottery_data.sollotto_labs_wallet != *sollotto_labs_wallet.key {
            msg!("Invalid sollotto labs wallet");
            return Err(LotteryError::InvalidSollottoAccount.into());
        }

        if sollotto_staking_pool_wallet.lamports() < lottery_data.staking_pool_amount {
            msg!("Sollotto staking pool wallet insufficient funds");
            return Err(ProgramError::InsufficientFunds);
        }

        // Calculate current prize pool from staking pool rewards
        // TODO: for now is just difference between sollotto_staking_pool_wallet balance and lottery.staking_pool_amount
        // fix it later to difference between liquidity staking pool balance and lottery.staking_pool_amount
        let prize_pool_lamports =
            sollotto_staking_pool_wallet.lamports() - lottery_data.staking_pool_amount;
        if prize_pool_lamports == 0 {
            msg!("Prize pool is empty");
            return Err(LotteryError::EmptyPrizePool.into());
        }

        let prize_pool_sol = lamports_to_sol(prize_pool_lamports);
        let winner_share = prize_pool_sol * 0.95;
        let sollotto_rewards_share = prize_pool_sol * 0.04;
        let slot_holders_share = prize_pool_sol * 0.0006;
        let sollotto_labs_share = prize_pool_sol * 0.0004;

        // Pay 95% of prize pool to the user
        invoke(
            &system_instruction::transfer(
                &sollotto_staking_pool_wallet.key,
                &winner_account.key,
                sol_to_lamports(winner_share),
            ),
            &[
                sollotto_staking_pool_wallet.clone(),
                winner_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        // Pay 4% of prize pool to Sollotto Foundation Rewards wallet
        invoke(
            &system_instruction::transfer(
                &sollotto_staking_pool_wallet.key,
                &sollotto_reward_wallet.key,
                sol_to_lamports(sollotto_rewards_share),
            ),
            &[
                sollotto_staking_pool_wallet.clone(),
                sollotto_reward_wallet.clone(),
                system_program_account.clone(),
            ],
        )?;

        // Pay 0.06% to SLOT Holders rewards wallet
        invoke(
            &system_instruction::transfer(
                &sollotto_staking_pool_wallet.key,
                &slot_holders_wallet.key,
                sol_to_lamports(slot_holders_share),
            ),
            &[
                sollotto_staking_pool_wallet.clone(),
                slot_holders_wallet.clone(),
                system_program_account.clone(),
            ],
        )?;

        // Pay 0.04% to Sollotto Labs wallet
        invoke(
            &system_instruction::transfer(
                &sollotto_staking_pool_wallet.key,
                &sollotto_labs_wallet.key,
                sol_to_lamports(sollotto_labs_share),
            ),
            &[
                sollotto_staking_pool_wallet.clone(),
                sollotto_labs_wallet.clone(),
                system_program_account.clone(),
            ],
        )?;

        // Save lottery result in account
        LotteryResultData::pack(
            LotteryResultData {
                lottery_id: lottery_id,
                winner: *winner_account.key,
            },
            &mut lottery_result_account.data.borrow_mut(),
        )?;

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
    };
    use spl_token::state::{Account, Mint};

    fn lottery_minimum_balance() -> u64 {
        Rent::default().minimum_balance(LotteryData::get_packed_len())
    }

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
    fn test_init_lottery() {
        let program_id = id();
        let mut rent_sysvar_acc = create_account_for_test(&Rent::default());
        let mut spl_token_acc = SolanaAccount::default();
        let lottery_key = Pubkey::new_unique();
        let mut lottery_acc = SolanaAccount::new(
            lottery_minimum_balance(),
            LotteryData::get_packed_len(),
            &program_id,
        );
        let staking_pool_mint_key = Pubkey::new_unique();
        let mut staking_pool_mint =
            SolanaAccount::new(mint_minimum_balance(), Mint::LEN, &spl_token::id());
        let staking_pool_token_account_key = Pubkey::new_unique();
        let mut staking_pool_token_account =
            SolanaAccount::new(account_minimum_balance(), Account::LEN, &spl_token::id());
        let staking_pool_wallet = Pubkey::new_unique();
        let rewards_wallet = Pubkey::new_unique();
        let slot_holders_rewards_wallet = Pubkey::new_unique();
        let sollotto_labs_wallet = Pubkey::new_unique();

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
                    &staking_pool_wallet,
                    &staking_pool_mint_key,
                    &staking_pool_token_account_key,
                    &rewards_wallet,
                    &slot_holders_rewards_wallet,
                    &sollotto_labs_wallet,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut bad_lottery_acc,
                    &mut staking_pool_mint,
                    &mut staking_pool_token_account,
                    &mut rent_sysvar_acc,
                    &mut spl_token_acc,
                ],
            )
        );

        do_process(
            crate::instruction::initialize_lottery(
                &program_id,
                &staking_pool_wallet,
                &staking_pool_mint_key,
                &staking_pool_token_account_key,
                &rewards_wallet,
                &slot_holders_rewards_wallet,
                &sollotto_labs_wallet,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut staking_pool_token_account,
                &mut rent_sysvar_acc,
                &mut spl_token_acc,
            ],
        )
        .unwrap();

        assert_eq!(
            Err(LotteryError::Initialized.into()),
            do_process(
                crate::instruction::initialize_lottery(
                    &program_id,
                    &staking_pool_wallet,
                    &staking_pool_mint_key,
                    &staking_pool_token_account_key,
                    &rewards_wallet,
                    &slot_holders_rewards_wallet,
                    &sollotto_labs_wallet,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut staking_pool_mint,
                    &mut staking_pool_token_account,
                    &mut rent_sysvar_acc,
                    &mut spl_token_acc,
                ],
            )
        );

        let lottery_data = LotteryData::unpack(&lottery_acc.data).unwrap();
        assert_eq!(lottery_data.is_initialized, true);
        assert_eq!(lottery_data.staking_pool_amount, 0);
        assert_eq!(lottery_data.staking_pool_token_mint, staking_pool_mint_key);
        assert_eq!(lottery_data.staking_pool_wallet, staking_pool_wallet);
        assert_eq!(lottery_data.sollotto_labs_wallet, sollotto_labs_wallet);
        assert_eq!(
            lottery_data.slot_holders_rewards_wallet,
            slot_holders_rewards_wallet
        );
        assert_eq!(lottery_data.rewards_wallet, rewards_wallet);
    }

    #[test]
    fn test_deposit() {
        let program_id = id();
        let mut rent_sysvar_acc = create_account_for_test(&Rent::default());
        let mut spl_token_acc = SolanaAccount::default();
        let mut system_acc = SolanaAccount::default();
        let lottery_key = Pubkey::new_unique();
        let mut lottery_acc = SolanaAccount::new(
            lottery_minimum_balance(),
            LotteryData::get_packed_len(),
            &program_id,
        );
        let staking_pool_mint_key = Pubkey::new_unique();
        let mut staking_pool_mint =
            SolanaAccount::new(mint_minimum_balance(), Mint::LEN, &spl_token::id());
        let staking_pool_token_account_key = Pubkey::new_unique();
        let mut staking_pool_token_account =
            SolanaAccount::new(account_minimum_balance(), Account::LEN, &spl_token::id());
        let staking_pool_wallet = Pubkey::new_unique();
        let rewards_wallet = Pubkey::new_unique();
        let slot_holders_rewards_wallet = Pubkey::new_unique();
        let sollotto_labs_wallet = Pubkey::new_unique();

        let user_key = Pubkey::new_unique();
        let mut user_wallet = SolanaAccount::default();
        let user_staking_pool_token_acc_key = Pubkey::new_unique();
        let mut user_staking_pool_token_acc =
            SolanaAccount::new(account_minimum_balance(), Account::LEN, &spl_token::id());

        // BadCase: Lottery is not initialized
        assert_eq!(
            Err(LotteryError::NotInitialized.into()),
            do_process(
                crate::instruction::deposit(
                    &program_id,
                    sol_to_lamports(1.0),
                    &staking_pool_mint_key,
                    &user_staking_pool_token_acc_key,
                    &staking_pool_wallet,
                    &user_key,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut staking_pool_mint,
                    &mut user_wallet,
                    &mut user_staking_pool_token_acc,
                    &mut staking_pool_token_account,
                    &mut spl_token_acc,
                    &mut system_acc,
                ],
            )
        );

        // Initialize lottery
        do_process(
            crate::instruction::initialize_lottery(
                &program_id,
                &staking_pool_wallet,
                &staking_pool_mint_key,
                &staking_pool_token_account_key,
                &rewards_wallet,
                &slot_holders_rewards_wallet,
                &sollotto_labs_wallet,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut staking_pool_token_account,
                &mut rent_sysvar_acc,
                &mut spl_token_acc,
            ],
        )
        .unwrap();

        // BadCase: Invalid token mint
        let bad_mint_key = Pubkey::new_unique();
        let mut bad_mint = SolanaAccount::default();
        assert_eq!(
            Err(LotteryError::InvalidSollottoAccount.into()),
            do_process(
                crate::instruction::deposit(
                    &program_id,
                    sol_to_lamports(1.0),
                    &bad_mint_key,
                    &user_staking_pool_token_acc_key,
                    &staking_pool_wallet,
                    &user_key,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut bad_mint,
                    &mut user_wallet,
                    &mut user_staking_pool_token_acc,
                    &mut staking_pool_token_account,
                    &mut spl_token_acc,
                    &mut system_acc,
                ],
            )
        );

        // BadCase: Invalid lottery staking pool wallet
        let bad_wallet_key = Pubkey::new_unique();
        assert_eq!(
            Err(LotteryError::InvalidSollottoAccount.into()),
            do_process(
                crate::instruction::deposit(
                    &program_id,
                    sol_to_lamports(1.0),
                    &staking_pool_mint_key,
                    &staking_pool_token_account_key,
                    &bad_wallet_key,
                    &user_key,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut staking_pool_mint,
                    &mut user_wallet,
                    &mut user_staking_pool_token_acc,
                    &mut staking_pool_token_account,
                    &mut spl_token_acc,
                    &mut system_acc,
                ],
            )
        );

        do_process(
            crate::instruction::deposit(
                &program_id,
                sol_to_lamports(1.0),
                &staking_pool_mint_key,
                &user_staking_pool_token_acc_key,
                &staking_pool_wallet,
                &user_key,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut user_wallet,
                &mut user_staking_pool_token_acc,
                &mut staking_pool_token_account,
                &mut spl_token_acc,
                &mut system_acc,
            ],
        )
        .unwrap();

        // Check lottery staking pool amount
        let lottery_data = LotteryData::unpack(&lottery_acc.data).unwrap();
        assert_eq!(lottery_data.staking_pool_amount, sol_to_lamports(1.0));
    }

    #[test]
    fn test_undeposit() {
        let program_id = id();
        let mut rent_sysvar_acc = create_account_for_test(&Rent::default());
        let mut spl_token_acc = SolanaAccount::default();
        let mut system_acc = SolanaAccount::default();
        let lottery_key = Pubkey::new_unique();
        let mut lottery_acc = SolanaAccount::new(
            lottery_minimum_balance(),
            LotteryData::get_packed_len(),
            &program_id,
        );
        let staking_pool_mint_key = Pubkey::new_unique();
        let mut staking_pool_mint =
            SolanaAccount::new(mint_minimum_balance(), Mint::LEN, &spl_token::id());
        let staking_pool_token_account_key = Pubkey::new_unique();
        let mut staking_pool_token_account =
            SolanaAccount::new(account_minimum_balance(), Account::LEN, &spl_token::id());
        let staking_pool_wallet = Pubkey::new_unique();
        let rewards_wallet = Pubkey::new_unique();
        let slot_holders_rewards_wallet = Pubkey::new_unique();
        let sollotto_labs_wallet = Pubkey::new_unique();

        let user_key = Pubkey::new_unique();
        let mut user_wallet = SolanaAccount::default();
        let user_staking_pool_token_acc_key = Pubkey::new_unique();
        let mut user_staking_pool_token_acc =
            SolanaAccount::new(account_minimum_balance(), Account::LEN, &spl_token::id());

        // BadCase: Lottery is not initialized
        assert_eq!(
            Err(LotteryError::NotInitialized.into()),
            do_process(
                crate::instruction::undeposit(
                    &program_id,
                    sol_to_lamports(1.0),
                    &staking_pool_mint_key,
                    &user_staking_pool_token_acc_key,
                    &staking_pool_wallet,
                    &user_key,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut staking_pool_mint,
                    &mut user_wallet,
                    &mut user_staking_pool_token_acc,
                    &mut staking_pool_token_account,
                    &mut spl_token_acc,
                    &mut system_acc,
                ],
            )
        );

        // Initialize lottery
        do_process(
            crate::instruction::initialize_lottery(
                &program_id,
                &staking_pool_wallet,
                &staking_pool_mint_key,
                &staking_pool_token_account_key,
                &rewards_wallet,
                &slot_holders_rewards_wallet,
                &sollotto_labs_wallet,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut staking_pool_token_account,
                &mut rent_sysvar_acc,
                &mut spl_token_acc,
            ],
        )
        .unwrap();

        // BadCase: Invalid token mint
        let bad_mint_key = Pubkey::new_unique();
        let mut bad_mint = SolanaAccount::default();
        assert_eq!(
            Err(LotteryError::InvalidSollottoAccount.into()),
            do_process(
                crate::instruction::undeposit(
                    &program_id,
                    sol_to_lamports(1.0),
                    &bad_mint_key,
                    &user_staking_pool_token_acc_key,
                    &staking_pool_wallet,
                    &user_key,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut bad_mint,
                    &mut user_wallet,
                    &mut user_staking_pool_token_acc,
                    &mut staking_pool_token_account,
                    &mut spl_token_acc,
                    &mut system_acc,
                ],
            )
        );

        // BadCase: Invalid lottery staking pool wallet
        let bad_wallet_key = Pubkey::new_unique();
        assert_eq!(
            Err(LotteryError::InvalidSollottoAccount.into()),
            do_process(
                crate::instruction::undeposit(
                    &program_id,
                    sol_to_lamports(1.0),
                    &staking_pool_mint_key,
                    &staking_pool_token_account_key,
                    &bad_wallet_key,
                    &user_key,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut staking_pool_mint,
                    &mut user_wallet,
                    &mut user_staking_pool_token_acc,
                    &mut staking_pool_token_account,
                    &mut spl_token_acc,
                    &mut system_acc,
                ],
            )
        );

        // BadCase: Staking pool amount insufficient funds
        assert_eq!(
            Err(ProgramError::InsufficientFunds),
            do_process(
                crate::instruction::undeposit(
                    &program_id,
                    sol_to_lamports(1.0),
                    &staking_pool_mint_key,
                    &staking_pool_token_account_key,
                    &staking_pool_wallet,
                    &user_key,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut staking_pool_mint,
                    &mut user_wallet,
                    &mut user_staking_pool_token_acc,
                    &mut staking_pool_token_account,
                    &mut spl_token_acc,
                    &mut system_acc,
                ],
            )
        );

        // Deposit 2 SOL
        do_process(
            crate::instruction::deposit(
                &program_id,
                sol_to_lamports(2.0),
                &staking_pool_mint_key,
                &user_staking_pool_token_acc_key,
                &staking_pool_wallet,
                &user_key,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut user_wallet,
                &mut user_staking_pool_token_acc,
                &mut staking_pool_token_account,
                &mut spl_token_acc,
                &mut system_acc,
            ],
        )
        .unwrap();

        // Undeposit 1 SOL
        do_process(
            crate::instruction::undeposit(
                &program_id,
                sol_to_lamports(1.0),
                &staking_pool_mint_key,
                &user_staking_pool_token_acc_key,
                &staking_pool_wallet,
                &user_key,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut user_wallet,
                &mut user_staking_pool_token_acc,
                &mut staking_pool_token_account,
                &mut spl_token_acc,
                &mut system_acc,
            ],
        )
        .unwrap();

        // Check lottery staking pool amount
        let lottery_data = LotteryData::unpack(&lottery_acc.data).unwrap();
        assert_eq!(lottery_data.staking_pool_amount, sol_to_lamports(1.0));
    }

    #[test]
    fn test_reward_winner() {
        let program_id = id();
        let mut rent_sysvar_acc = create_account_for_test(&Rent::default());
        let mut spl_token_acc = SolanaAccount::default();
        let mut system_acc = SolanaAccount::default();
        let lottery_key = Pubkey::new_unique();
        let mut lottery_acc = SolanaAccount::new(
            lottery_minimum_balance(),
            LotteryData::get_packed_len(),
            &program_id,
        );
        let staking_pool_mint_key = Pubkey::new_unique();
        let mut staking_pool_mint =
            SolanaAccount::new(mint_minimum_balance(), Mint::LEN, &spl_token::id());
        let staking_pool_token_account_key = Pubkey::new_unique();
        let mut staking_pool_token_account =
            SolanaAccount::new(account_minimum_balance(), Account::LEN, &spl_token::id());
        let staking_pool_wallet = Pubkey::new_unique();
        let mut staking_pool_wallet_account = SolanaAccount::default();
        let rewards_wallet = Pubkey::new_unique();
        let mut rewards_wallet_account = SolanaAccount::default();
        let slot_holders_rewards_wallet = Pubkey::new_unique();
        let mut slot_holders_rewards_wallet_account = SolanaAccount::default();
        let sollotto_labs_wallet = Pubkey::new_unique();
        let mut sollotto_labs_wallet_account = SolanaAccount::default();

        let user_key = Pubkey::new_unique();
        let mut user_wallet = SolanaAccount::default();
        let user_staking_pool_token_acc_key = Pubkey::new_unique();
        let mut user_staking_pool_token_acc =
            SolanaAccount::new(account_minimum_balance(), Account::LEN, &spl_token::id());

        let lottery_id = 112233;
        let lottery_result_account_key = Pubkey::new_unique();
        let mut lottery_result_account = SolanaAccount::new(
            lottery_result_minimum_balance(),
            LotteryResultData::LEN,
            &program_id,
        );

        // BadCase: Lottery is not initialized
        assert_eq!(
            Err(LotteryError::NotInitialized.into()),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    &lottery_result_account_key,
                    &user_key,
                    &rewards_wallet,
                    &slot_holders_rewards_wallet,
                    &sollotto_labs_wallet,
                    &staking_pool_wallet,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut lottery_result_account,
                    &mut user_wallet,
                    &mut staking_pool_wallet_account,
                    &mut rewards_wallet_account,
                    &mut slot_holders_rewards_wallet_account,
                    &mut sollotto_labs_wallet_account,
                    &mut system_acc,
                ],
            )
        );

        // Initialize lottery
        do_process(
            crate::instruction::initialize_lottery(
                &program_id,
                &staking_pool_wallet,
                &staking_pool_mint_key,
                &staking_pool_token_account_key,
                &rewards_wallet,
                &slot_holders_rewards_wallet,
                &sollotto_labs_wallet,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut staking_pool_token_account,
                &mut rent_sysvar_acc,
                &mut spl_token_acc,
            ],
        )
        .unwrap();

        // User deposit 1 SOL
        do_process(
            crate::instruction::deposit(
                &program_id,
                sol_to_lamports(1.0),
                &staking_pool_mint_key,
                &user_staking_pool_token_acc_key,
                &staking_pool_wallet,
                &user_key,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut user_wallet,
                &mut user_staking_pool_token_acc,
                &mut staking_pool_token_account,
                &mut spl_token_acc,
                &mut system_acc,
            ],
        )
        .unwrap();
        staking_pool_wallet_account.lamports = sol_to_lamports(1.0);

        // BadCase: prize pool is empty (there is no staking rewards for now)
        assert_eq!(
            Err(LotteryError::EmptyPrizePool.into()),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    &lottery_result_account_key,
                    &user_key,
                    &rewards_wallet,
                    &slot_holders_rewards_wallet,
                    &sollotto_labs_wallet,
                    &staking_pool_wallet,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut lottery_result_account,
                    &mut user_wallet,
                    &mut staking_pool_wallet_account,
                    &mut rewards_wallet_account,
                    &mut slot_holders_rewards_wallet_account,
                    &mut sollotto_labs_wallet_account,
                    &mut system_acc,
                ],
            )
        );

        // BadCase: staking pool wallet insufficient funds
        // Staking pool wallet spends 0.5 SOL
        staking_pool_wallet_account.lamports -= sol_to_lamports(0.5);
        assert_eq!(
            Err(ProgramError::InsufficientFunds),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    &lottery_result_account_key,
                    &user_key,
                    &rewards_wallet,
                    &slot_holders_rewards_wallet,
                    &sollotto_labs_wallet,
                    &staking_pool_wallet,
                    &lottery_key,
                )
                .unwrap(),
                vec![
                    &mut lottery_acc,
                    &mut lottery_result_account,
                    &mut user_wallet,
                    &mut staking_pool_wallet_account,
                    &mut rewards_wallet_account,
                    &mut slot_holders_rewards_wallet_account,
                    &mut sollotto_labs_wallet_account,
                    &mut system_acc,
                ],
            )
        );

        // Get the staking reward for prize pool
        staking_pool_wallet_account.lamports += sol_to_lamports(1.0);

        // User wins lottery
        do_process(
            crate::instruction::reward_winner(
                &program_id,
                lottery_id,
                &lottery_result_account_key,
                &user_key,
                &rewards_wallet,
                &slot_holders_rewards_wallet,
                &sollotto_labs_wallet,
                &staking_pool_wallet,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut lottery_result_account,
                &mut user_wallet,
                &mut staking_pool_wallet_account,
                &mut rewards_wallet_account,
                &mut slot_holders_rewards_wallet_account,
                &mut sollotto_labs_wallet_account,
                &mut system_acc,
            ],
        )
        .unwrap();

        // Check staking pool amount
        let lottery_data = LotteryData::unpack(&lottery_acc.data).unwrap();
        assert_eq!(lottery_data.staking_pool_amount, sol_to_lamports(1.0));

        // Check lottery result data
        let lottery_result_data =
            LotteryResultData::unpack_unchecked(&lottery_result_account.data).unwrap();
        assert_eq!(lottery_result_data.lottery_id, lottery_id);
        assert_eq!(lottery_result_data.winner, user_key);
    }

    #[test]
    fn test_update_wallets() {
        let program_id = id();
        let mut rent_sysvar_acc = create_account_for_test(&Rent::default());
        let mut spl_token_acc = SolanaAccount::default();
        let lottery_key = Pubkey::new_unique();
        let mut lottery_acc = SolanaAccount::new(
            lottery_minimum_balance(),
            LotteryData::get_packed_len(),
            &program_id,
        );
        let staking_pool_mint_key = Pubkey::new_unique();
        let mut staking_pool_mint =
            SolanaAccount::new(mint_minimum_balance(), Mint::LEN, &spl_token::id());
        let staking_pool_token_account_key = Pubkey::new_unique();
        let mut staking_pool_token_account =
            SolanaAccount::new(account_minimum_balance(), Account::LEN, &spl_token::id());
        let staking_pool_wallet = Pubkey::new_unique();
        let rewards_wallet = Pubkey::new_unique();
        let slot_holders_rewards_wallet = Pubkey::new_unique();
        let sollotto_labs_wallet = Pubkey::new_unique();

        // BadCase: Lottery is not initialized
        assert_eq!(
            Err(LotteryError::NotInitialized.into()),
            do_process(
                crate::instruction::update_lottery_wallets(
                    &program_id,
                    &staking_pool_wallet,
                    &staking_pool_mint_key,
                    &rewards_wallet,
                    &slot_holders_rewards_wallet,
                    &sollotto_labs_wallet,
                    &lottery_key,
                )
                .unwrap(),
                vec![&mut lottery_acc,],
            )
        );

        // Initialize lottery data
        do_process(
            crate::instruction::initialize_lottery(
                &program_id,
                &staking_pool_wallet,
                &staking_pool_mint_key,
                &staking_pool_token_account_key,
                &rewards_wallet,
                &slot_holders_rewards_wallet,
                &sollotto_labs_wallet,
                &lottery_key,
            )
            .unwrap(),
            vec![
                &mut lottery_acc,
                &mut staking_pool_mint,
                &mut staking_pool_token_account,
                &mut rent_sysvar_acc,
                &mut spl_token_acc,
            ],
        )
        .unwrap();

        // Update lottery wallets
        let new_rewards_wallet = Pubkey::new_unique();
        let new_slot_holders_rewards_wallet = Pubkey::new_unique();
        let new_sollotto_labs_wallet = Pubkey::new_unique();
        do_process(
            crate::instruction::update_lottery_wallets(
                &program_id,
                &staking_pool_wallet,
                &staking_pool_mint_key,
                &new_rewards_wallet,
                &new_slot_holders_rewards_wallet,
                &new_sollotto_labs_wallet,
                &lottery_key,
            )
            .unwrap(),
            vec![&mut lottery_acc],
        )
        .unwrap();

        let lottery_data = LotteryData::unpack(&lottery_acc.data).unwrap();
        assert_eq!(lottery_data.is_initialized, true);
        assert_eq!(lottery_data.staking_pool_amount, 0);
        assert_eq!(lottery_data.staking_pool_token_mint, staking_pool_mint_key);
        assert_eq!(lottery_data.staking_pool_wallet, staking_pool_wallet);
        assert_eq!(lottery_data.sollotto_labs_wallet, new_sollotto_labs_wallet);
        assert_eq!(
            lottery_data.slot_holders_rewards_wallet,
            new_slot_holders_rewards_wallet
        );
        assert_eq!(lottery_data.rewards_wallet, new_rewards_wallet);
    }
}
