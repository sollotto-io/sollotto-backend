//! Program state processor
use crate::{
    check_program_account, error::LotteryError, instruction::LotteryInstruction,
    state::LotteryResultData,
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};
use spl_token::{
    amount_to_ui_amount,
    state::{Account, Mint},
    ui_amount_to_amount,
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
        let token_prize_pool_owner = next_account_info(accounts_iter)?;
        let token_prize_pool_account = next_account_info(accounts_iter)?;
        let charity_token_account = next_account_info(accounts_iter)?;
        let token_mint = next_account_info(accounts_iter)?;
        let lottery_result_account = next_account_info(accounts_iter)?;
        let staking_pool_token_mint = next_account_info(accounts_iter)?;
        let spl_token_info = next_account_info(accounts_iter)?;
        let rent_info = next_account_info(accounts_iter)?;
        let participants_accounts = accounts_iter.as_slice();

        if !token_prize_pool_owner.is_signer {
            msg!("Missing Custom SPL Token prize pool owner signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        if lottery_result_account.owner != program_id {
            msg!("Invalid owner for LotteryResult data account");
            return Err(ProgramError::IncorrectProgramId);
        }

        let rent = Rent::from_account_info(rent_info)?;
        if !rent.is_exempt(lottery_result_account.lamports(), LotteryResultData::LEN) {
            msg!("Rent exempt error for LotteryDataAccount");
            return Err(ProgramError::AccountNotRentExempt);
        }

        let token_mint_data = Mint::unpack(&token_mint.data.borrow())?;
        let token_prize_pool_account_data =
            Account::unpack(&token_prize_pool_account.data.borrow())?;
        if token_prize_pool_account_data.amount == 0 {
            msg!("Custom SPL Token Prize pool is empty");
            return Err(LotteryError::EmptyPrizePool.into());
        }

        let prize_pool = amount_to_ui_amount(
            token_prize_pool_account_data.amount,
            token_mint_data.decimals,
        );

        if participants_accounts.len() % 2 != 0 {
            msg!(
                "Invalid participants accounts size: {}",
                participants_accounts.len()
            );
            return Err(LotteryError::InvalidParticipantsAccounts.into());
        }
        if random_number > participants_accounts.len() as u32 / 2 {
            msg!("Invalid random number: {}", random_number);
            return Err(LotteryError::InvalidRandomNumber.into());
        }

        // Check all participants validness (mint and amount)
        for i in (0..participants_accounts.len()).step_by(2) {
            let participant_token_account =
                Account::unpack(&participants_accounts[i].data.borrow())?;
            if participant_token_account.mint != *token_mint.key {
                msg!("Invalid Custom SPL Token participant mint");
                return Err(LotteryError::InvalidParticipantsAccounts.into());
            }

            let participant_staking_pool_token_account =
                Account::unpack(&participants_accounts[i + 1].data.borrow())?;
            if participant_staking_pool_token_account.amount == 0 {
                msg!("Participant Staking pool token amount 0");
                return Err(LotteryError::InvalidParticipantsAccounts.into());
            }
            if participant_staking_pool_token_account.mint != *staking_pool_token_mint.key {
                msg!("Invalid Staking Pool Mint");
                return Err(LotteryError::InvalidParticipantsAccounts.into());
            }
        }

        // Find the winner's Custom SPL Token account in participant list
        let winner_account = participants_accounts[(random_number * 2) as usize].clone();
        // Find the reward shares
        // 70% of the prize pool is transferred to the winner
        let winner_share = prize_pool * 0.7;
        // 30% to a charity provided by the partner project
        let charity_share = prize_pool * 0.3;

        // Transfer winner share
        invoke(
            &spl_token::instruction::transfer(
                &spl_token::id(),
                token_prize_pool_account.key,
                winner_account.key,
                token_prize_pool_owner.key,
                &[],
                ui_amount_to_amount(winner_share, token_mint_data.decimals),
            )
            .unwrap(),
            &[
                spl_token_info.clone(),
                token_prize_pool_account.clone(),
                winner_account.clone(),
                token_prize_pool_owner.clone(),
            ],
        )?;

        // Transfer charity share
        invoke(
            &spl_token::instruction::transfer(
                &spl_token::id(),
                token_prize_pool_account.key,
                charity_token_account.key,
                token_prize_pool_owner.key,
                &[],
                ui_amount_to_amount(charity_share, token_mint_data.decimals),
            )
            .unwrap(),
            &[
                spl_token_info.clone(),
                token_prize_pool_account.clone(),
                charity_token_account.clone(),
                token_prize_pool_owner.clone(),
            ],
        )?;

        // Save LotteryResult on-chain
        LotteryResultData::pack(
            LotteryResultData {
                lottery_id: lottery_id,
                winner: *winner_account.key,
            },
            &mut lottery_result_account.data.borrow_mut(),
        )?;

        Ok(())
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::instruction::Instruction;
    use solana_sdk::account::{
        create_account_for_test, create_is_signer_account_infos, Account as SolanaAccount,
    };
    use spl_token::ui_amount_to_amount;

    fn mint_minimum_balance() -> u64 {
        Rent::default().minimum_balance(spl_token::state::Mint::get_packed_len())
    }

    fn account_minimum_balance() -> u64 {
        Rent::default().minimum_balance(spl_token::state::Account::get_packed_len())
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
        let program_id = crate::id();
        let mut spl_token_acc = SolanaAccount::default();
        let mut rent_acc = create_account_for_test(&Rent::default());

        let prize_pool_owner_key = Pubkey::new_unique();
        let mut prize_pool_owner_acc = SolanaAccount::default();

        let token_mint_key = Pubkey::new_unique();
        let mut token_mint_acc = SolanaAccount::new(
            mint_minimum_balance(),
            spl_token::state::Mint::get_packed_len(),
            &spl_token::id(),
        );
        Mint::pack(
            Mint {
                is_initialized: true,
                decimals: 9,
                ..Default::default()
            },
            &mut token_mint_acc.data,
        )
        .unwrap();

        let staking_pool_token_mint_key = Pubkey::new_unique();
        let mut staking_pool_token_mint_acc = SolanaAccount::new(
            mint_minimum_balance(),
            spl_token::state::Mint::get_packed_len(),
            &spl_token::id(),
        );
        Mint::pack(
            Mint {
                is_initialized: true,
                decimals: 9,
                ..Default::default()
            },
            &mut staking_pool_token_mint_acc.data,
        )
        .unwrap();

        let prize_pool_token_account_key = Pubkey::new_unique();
        let mut prize_pool_token_account_acc = SolanaAccount::new(
            account_minimum_balance(),
            spl_token::state::Account::get_packed_len(),
            &spl_token::id()
        );
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                amount: 0,
                mint: token_mint_key,
                owner: prize_pool_owner_key,
                ..Default::default()
            },
            &mut prize_pool_token_account_acc.data,
        )
        .unwrap();

        let charity_token_account_key = Pubkey::new_unique();
        let mut charity_token_account_acc = SolanaAccount::new(
            account_minimum_balance(),
            spl_token::state::Account::get_packed_len(),
            &spl_token::id()
        );
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                mint: token_mint_key,
                owner: Pubkey::new_unique(),
                ..Default::default()
            },
            &mut prize_pool_token_account_acc.data,
        )
        .unwrap();

        let user_1_auth = Pubkey::new_unique();
        let user_1_token_key = Pubkey::new_unique();
        let mut user_1_token_acc = SolanaAccount::new(
            account_minimum_balance(),
            spl_token::state::Account::get_packed_len(),
            &spl_token::id()
        );
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                mint: token_mint_key,
                owner: user_1_auth,
                ..Default::default()
            },
            &mut user_1_token_acc.data,
        )
        .unwrap();
        let user_1_staking_pool_token_key = Pubkey::new_unique();
        let mut user_1_staking_pool_token_acc = SolanaAccount::new(
            account_minimum_balance(),
            spl_token::state::Account::get_packed_len(),
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                mint: staking_pool_token_mint_key,
                owner: user_1_auth,
                ..Default::default()
            },
            &mut user_1_staking_pool_token_acc.data,
        )
        .unwrap();

        let user_2_auth = Pubkey::new_unique();
        let user_2_token_key = Pubkey::new_unique();
        let mut user_2_token_acc = SolanaAccount::new(
            account_minimum_balance(),
            spl_token::state::Account::get_packed_len(),
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                mint: token_mint_key,
                owner: user_2_auth,
                ..Default::default()
            },
            &mut user_2_token_acc.data,
        )
        .unwrap();
        let user_2_staking_pool_token_key = Pubkey::new_unique();
        let mut user_2_staking_pool_token_acc = SolanaAccount::new(
            account_minimum_balance(),
            spl_token::state::Account::get_packed_len(),
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                mint: staking_pool_token_mint_key,
                owner: user_2_auth,
                ..Default::default()
            },
            &mut user_2_staking_pool_token_acc.data,
        )
        .unwrap();

        let lottery_id = 1;
        let random_number = 1;

        let lottery_result_key = Pubkey::new_unique();
        let mut lottery_result_acc = SolanaAccount::new(
            lottery_result_minimum_balance() - 1,
            LotteryResultData::LEN,
            &program_id,
        );

        // BadCase: LotteryResult account rent exempt
        assert_eq!(
            Err(ProgramError::AccountNotRentExempt),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    random_number,
                    &prize_pool_owner_key,
                    &prize_pool_token_account_key,
                    &charity_token_account_key,
                    &token_mint_key,
                    &lottery_result_key,
                    &staking_pool_token_mint_key,
                    &vec![
                        (user_1_token_key, user_1_staking_pool_token_key),
                        (user_2_token_key, user_2_staking_pool_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_owner_acc,
                    &mut prize_pool_token_account_acc,
                    &mut charity_token_account_acc,
                    &mut token_mint_acc,
                    &mut lottery_result_acc,
                    &mut staking_pool_token_mint_acc,
                    &mut spl_token_acc,
                    &mut rent_acc,
                    &mut user_1_token_acc,
                    &mut user_1_staking_pool_token_acc,
                    &mut user_2_token_acc,
                    &mut user_2_staking_pool_token_acc,
                ],
            )
        );

        let mut lottery_result_acc = SolanaAccount::new(
            lottery_result_minimum_balance(),
            LotteryResultData::LEN,
            &program_id,
        );

        // BadCase EmptyPrizePool
        assert_eq!(
            Err(LotteryError::EmptyPrizePool.into()),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    random_number,
                    &prize_pool_owner_key,
                    &prize_pool_token_account_key,
                    &charity_token_account_key,
                    &token_mint_key,
                    &lottery_result_key,
                    &staking_pool_token_mint_key,
                    &vec![
                        (user_1_token_key, user_1_staking_pool_token_key),
                        (user_2_token_key, user_2_staking_pool_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_owner_acc,
                    &mut prize_pool_token_account_acc,
                    &mut charity_token_account_acc,
                    &mut token_mint_acc,
                    &mut lottery_result_acc,
                    &mut staking_pool_token_mint_acc,
                    &mut spl_token_acc,
                    &mut rent_acc,
                    &mut user_1_token_acc,
                    &mut user_1_staking_pool_token_acc,
                    &mut user_2_token_acc,
                    &mut user_2_staking_pool_token_acc,
                ],
            )
        );

        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                amount: ui_amount_to_amount(10.0, 9),
                mint: token_mint_key,
                owner: prize_pool_owner_key,
                ..Default::default()
            },
            &mut prize_pool_token_account_acc.data,
        )
        .unwrap();

        // BadCase: InvalidParticipantsAccounts size
        assert_eq!(
            Err(LotteryError::InvalidParticipantsAccounts.into()),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    random_number,
                    &prize_pool_owner_key,
                    &prize_pool_token_account_key,
                    &charity_token_account_key,
                    &token_mint_key,
                    &lottery_result_key,
                    &staking_pool_token_mint_key,
                    &vec![
                        (user_1_token_key, user_1_staking_pool_token_key),
                        (user_2_token_key, user_2_staking_pool_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_owner_acc,
                    &mut prize_pool_token_account_acc,
                    &mut charity_token_account_acc,
                    &mut token_mint_acc,
                    &mut lottery_result_acc,
                    &mut staking_pool_token_mint_acc,
                    &mut spl_token_acc,
                    &mut rent_acc,
                    &mut user_1_token_acc,
                    &mut user_1_staking_pool_token_acc,
                    &mut user_2_token_acc,
                ],
            )
        );

        // BadCase: InvalidParticipantsAccounts bad random number
        let bad_random_number = 10;
        assert_eq!(
            Err(LotteryError::InvalidRandomNumber.into()),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    bad_random_number,
                    &prize_pool_owner_key,
                    &prize_pool_token_account_key,
                    &charity_token_account_key,
                    &token_mint_key,
                    &lottery_result_key,
                    &staking_pool_token_mint_key,
                    &vec![
                        (user_1_token_key, user_1_staking_pool_token_key),
                        (user_2_token_key, user_2_staking_pool_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_owner_acc,
                    &mut prize_pool_token_account_acc,
                    &mut charity_token_account_acc,
                    &mut token_mint_acc,
                    &mut lottery_result_acc,
                    &mut staking_pool_token_mint_acc,
                    &mut spl_token_acc,
                    &mut rent_acc,
                    &mut user_1_token_acc,
                    &mut user_1_staking_pool_token_acc,
                    &mut user_2_token_acc,
                    &mut user_2_staking_pool_token_acc,
                ],
            )
        );

        // BadCase: InvalidParticipantsAccounts invalid staking pool token amount
        assert_eq!(
            Err(LotteryError::InvalidParticipantsAccounts.into()),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    random_number,
                    &prize_pool_owner_key,
                    &prize_pool_token_account_key,
                    &charity_token_account_key,
                    &token_mint_key,
                    &lottery_result_key,
                    &staking_pool_token_mint_key,
                    &vec![
                        (user_1_token_key, user_1_staking_pool_token_key),
                        (user_2_token_key, user_2_staking_pool_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_owner_acc,
                    &mut prize_pool_token_account_acc,
                    &mut charity_token_account_acc,
                    &mut token_mint_acc,
                    &mut lottery_result_acc,
                    &mut staking_pool_token_mint_acc,
                    &mut spl_token_acc,
                    &mut rent_acc,
                    &mut user_1_token_acc,
                    &mut user_1_staking_pool_token_acc,
                    &mut user_2_token_acc,
                    &mut user_2_staking_pool_token_acc,
                ],
            )
        );

        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                amount: ui_amount_to_amount(1.0, 9),
                mint: staking_pool_token_mint_key,
                owner: user_1_auth,
                ..Default::default()
            },
            &mut user_1_staking_pool_token_acc.data,
        )
        .unwrap();
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                amount: ui_amount_to_amount(1.0, 9),
                mint: staking_pool_token_mint_key,
                owner: user_2_auth,
                ..Default::default()
            },
            &mut user_2_staking_pool_token_acc.data,
        )
        .unwrap();

        do_process(
            crate::instruction::reward_winner(
                &program_id,
                lottery_id,
                random_number,
                &prize_pool_owner_key,
                &prize_pool_token_account_key,
                &charity_token_account_key,
                &token_mint_key,
                &lottery_result_key,
                &staking_pool_token_mint_key,
                &vec![
                    (user_1_token_key, user_1_staking_pool_token_key),
                    (user_2_token_key, user_2_staking_pool_token_key),
                ],
            )
            .unwrap(),
            vec![
                &mut prize_pool_owner_acc,
                &mut prize_pool_token_account_acc,
                &mut charity_token_account_acc,
                &mut token_mint_acc,
                &mut lottery_result_acc,
                &mut staking_pool_token_mint_acc,
                &mut spl_token_acc,
                &mut rent_acc,
                &mut user_1_token_acc,
                &mut user_1_staking_pool_token_acc,
                &mut user_2_token_acc,
                &mut user_2_staking_pool_token_acc,
            ],
        )
        .unwrap();

        // Check LotteryResult account
        let lottery_result_data =
            LotteryResultData::unpack_unchecked(&lottery_result_acc.data).unwrap();
        assert_eq!(lottery_result_data.lottery_id, lottery_id);
        assert_eq!(lottery_result_data.winner, user_2_token_key);
    }
}
