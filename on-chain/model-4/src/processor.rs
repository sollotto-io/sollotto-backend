//! Program state processor
use crate::{
    check_program_account, error::LotteryError, instruction::LotteryInstruction,
    state::LotteryResultData,
};
use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, native_token::{lamports_to_sol, sol_to_lamports}, program::invoke, program_error::ProgramError, program_option::COption, program_pack::Pack, pubkey::Pubkey, rent::Rent, system_instruction, sysvar::Sysvar};
use spl_token::state::{Account, Mint};

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
            LotteryInstruction::RewardWinner {
                lottery_id,
                random_number,
            } => {
                msg!("Instruction: reward winner");
                Self::process_reward_winner(program_id, accounts, lottery_id, random_number)
            }
        }
    }

    pub fn process_reward_winner(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        lottery_id: u32,
        random_number: u32,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let prize_pool_wallet = next_account_info(accounts_iter)?;
        let sollotto_rewards_wallet = next_account_info(accounts_iter)?;
        let slot_holders_wallet = next_account_info(accounts_iter)?;
        let sollotto_labs_wallet = next_account_info(accounts_iter)?;
        let lottery_result_account = next_account_info(accounts_iter)?;
        let lifetime_ticket_token_owner = next_account_info(accounts_iter)?;
        let lifetime_ticket_token_mint = next_account_info(accounts_iter)?;
        let system_program_info = next_account_info(accounts_iter)?;
        let rent_info = next_account_info(accounts_iter)?;
        let participants_accounts = accounts_iter.as_slice();

        if !prize_pool_wallet.is_signer {
            msg!("Missing Prize pool wallet signature");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if !lifetime_ticket_token_owner.is_signer {
            msg!("Missint Lifetime Ticket Token mint authority signature")
        }

        if lottery_result_account.owner != program_id {
            msg!("Invalid owner for LotteryResult data account");
            return Err(ProgramError::IncorrectProgramId);
        }

        let lifetime_ticket_mint = Mint::unpack(&lifetime_ticket_token_mint.data.borrow())?;
        if lifetime_ticket_mint.mint_authority != COption::Some(*lifetime_ticket_token_owner.key) {
            msg!("Invalid Lifetime Ticket token owner!");
            return Err(LotteryError::InvalidLifetimeTicketOwner.into());
        }

        let rent = Rent::from_account_info(rent_info)?;
        if !rent.is_exempt(lottery_result_account.lamports(), LotteryResultData::LEN) {
            msg!("Rent exempt error for LotteryDataAccount");
            return Err(ProgramError::AccountNotRentExempt);
        }

        if prize_pool_wallet.lamports() == 0 {
            msg!("Prize pool is empty");
            return Err(LotteryError::EmptyPrizePool.into());
        }

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

        // Check all participants validness (mint, owner and amount)
        for i in (0..participants_accounts.len()).step_by(2) {
            let participant_lifetime_ticket_token_account =
                Account::unpack(&participants_accounts[i + 1].data.borrow())?;
            if participant_lifetime_ticket_token_account.amount == 0 {
                msg!("Participant Lifetime Ticket token amount 0");
                return Err(LotteryError::InvalidParticipantsAccounts.into());
            }
            if participant_lifetime_ticket_token_account.owner != *participants_accounts[i].key {
                msg!("Invalid Lifetime Ticket token account owner");
                return Err(LotteryError::InvalidParticipantsAccounts.into());
            }
            if participant_lifetime_ticket_token_account.mint != *lifetime_ticket_token_mint.key {
                msg!("Invalid Lifetime Ticket token Mint");
                return Err(LotteryError::InvalidParticipantsAccounts.into());
            }
        }

        // Find the winner's wallet in participant list
        let winner_wallet = participants_accounts[(random_number * 2) as usize].clone();
        // Find and pay the reward shares
        let prize_pool = lamports_to_sol(prize_pool_wallet.lamports());
        // Send the winner 95% of the prize pool
        let winner_share = prize_pool * 0.95;
        invoke(
            &system_instruction::transfer(
                prize_pool_wallet.key,
                winner_wallet.key,
                sol_to_lamports(winner_share),
            ),
            &[
                system_program_info.clone(),
                prize_pool_wallet.clone(),
                winner_wallet.clone(),
            ],
        )?;

        // Send 4% to the SolLotto Foundation Rewards Pool Wallet
        let sollotto_rewards_share = prize_pool * 0.04;
        invoke(
            &system_instruction::transfer(
                prize_pool_wallet.key,
                sollotto_rewards_wallet.key,
                sol_to_lamports(sollotto_rewards_share),
            ),
            &[
                system_program_info.clone(),
                prize_pool_wallet.clone(),
                sollotto_rewards_wallet.clone(),
            ],
        )?;

        // Send .6% to the “SLOT Holder Rewards” Wallet Address
        let slot_holders_share = prize_pool * 0.006;
        invoke(
            &system_instruction::transfer(
                prize_pool_wallet.key,
                slot_holders_wallet.key,
                sol_to_lamports(slot_holders_share),
            ),
            &[
                system_program_info.clone(),
                prize_pool_wallet.clone(),
                slot_holders_wallet.clone(),
            ],
        )?;

        // Send .4% to the SolLotto Labs Wallet Address
        let sollotto_labs_share = prize_pool * 0.004;
        invoke(
            &system_instruction::transfer(
                prize_pool_wallet.key,
                sollotto_labs_wallet.key,
                sol_to_lamports(sollotto_labs_share),
            ),
            &[
                system_program_info.clone(),
                prize_pool_wallet.clone(),
                sollotto_labs_wallet.clone(),
            ],
        )?;

        // Save LotteryResult on-chain
        LotteryResultData::pack(
            LotteryResultData {
                lottery_id: lottery_id,
                winner: *winner_wallet.key,
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
    use solana_program::{instruction::Instruction, program_option::COption};
    use solana_sdk::account::{
        create_account_for_test, create_is_signer_account_infos, Account as SolanaAccount,
    };
    use spl_token::{state::Mint, ui_amount_to_amount};

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
    fn test_reward_winner() {
        let program_id = crate::id();
        let mut system_account = SolanaAccount::default();
        let mut rent_acc = create_account_for_test(&Rent::default());

        let lifetime_token_owner_key = Pubkey::new_unique();
        let mut lifetime_token_owner_acc = SolanaAccount::default();
        let lifetime_token_mint_key = Pubkey::new_unique();
        let mut lifetime_token_mint_acc = SolanaAccount::new(
            mint_minimum_balance(),
            spl_token::state::Mint::get_packed_len(),
            &spl_token::id(),
        );
        Mint::pack(
            Mint {
                is_initialized: true,
                mint_authority: COption::Some(lifetime_token_owner_key),
                decimals: 9,
                ..Default::default()
            },
            &mut lifetime_token_mint_acc.data,
        )
        .unwrap();

        let prize_pool_key = Pubkey::new_unique();
        let mut prize_pool_acc = SolanaAccount::default();

        let sollotto_rewards_key = Pubkey::new_unique();
        let mut sollotto_rewards_acc = SolanaAccount::default();
        let slot_holders_key = Pubkey::new_unique();
        let mut slot_holders_acc = SolanaAccount::default();
        let sollotto_labs_key = Pubkey::new_unique();
        let mut sollotto_labs_acc = SolanaAccount::default();

        let user_1_wallet_key = Pubkey::new_unique();
        let mut user_1_wallet_acc = SolanaAccount::default();
        let user_1_lifetime_token_key = Pubkey::new_unique();
        let mut user_1_lifetime_token_acc = SolanaAccount::new(
            account_minimum_balance(),
            spl_token::state::Account::get_packed_len(),
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                mint: lifetime_token_mint_key,
                owner: user_1_wallet_key,
                ..Default::default()
            },
            &mut user_1_lifetime_token_acc.data,
        )
        .unwrap();

        let user_2_wallet_key = Pubkey::new_unique();
        let mut user_2_wallet_acc = SolanaAccount::default();
        let user_2_lifetime_token_key = Pubkey::new_unique();
        let mut user_2_lifetime_token_acc = SolanaAccount::new(
            account_minimum_balance(),
            spl_token::state::Account::get_packed_len(),
            &spl_token::id(),
        );
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                mint: lifetime_token_mint_key,
                owner: user_2_wallet_key,
                ..Default::default()
            },
            &mut user_2_lifetime_token_acc.data,
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

        assert_eq!(
            Err(ProgramError::AccountNotRentExempt),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    random_number,
                    &prize_pool_key,
                    &sollotto_rewards_key,
                    &slot_holders_key,
                    &sollotto_labs_key,
                    &lottery_result_key,
                    &lifetime_token_owner_key,
                    &lifetime_token_mint_key,
                    &vec![
                        (user_1_wallet_key, user_1_lifetime_token_key),
                        (user_2_wallet_key, user_2_lifetime_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_acc,
                    &mut sollotto_rewards_acc,
                    &mut slot_holders_acc,
                    &mut sollotto_labs_acc,
                    &mut lottery_result_acc,
                    &mut lifetime_token_owner_acc,
                    &mut lifetime_token_mint_acc,
                    &mut system_account,
                    &mut rent_acc,
                    &mut user_1_wallet_acc,
                    &mut user_1_lifetime_token_acc,
                    &mut user_2_wallet_acc,
                    &mut user_2_lifetime_token_acc,
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
                    &prize_pool_key,
                    &sollotto_rewards_key,
                    &slot_holders_key,
                    &sollotto_labs_key,
                    &lottery_result_key,
                    &lifetime_token_owner_key,
                    &lifetime_token_mint_key,
                    &vec![
                        (user_1_wallet_key, user_1_lifetime_token_key),
                        (user_2_wallet_key, user_2_lifetime_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_acc,
                    &mut sollotto_rewards_acc,
                    &mut slot_holders_acc,
                    &mut sollotto_labs_acc,
                    &mut lottery_result_acc,
                    &mut lifetime_token_owner_acc,
                    &mut lifetime_token_mint_acc,
                    &mut system_account,
                    &mut rent_acc,
                    &mut user_1_wallet_acc,
                    &mut user_1_lifetime_token_acc,
                    &mut user_2_wallet_acc,
                    &mut user_2_lifetime_token_acc,
                ],
            )
        );

        prize_pool_acc.lamports = sol_to_lamports(10.0);

        // BadCase: InvalidParticipantsAccounts size
        assert_eq!(
            Err(LotteryError::InvalidParticipantsAccounts.into()),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    random_number,
                    &prize_pool_key,
                    &sollotto_rewards_key,
                    &slot_holders_key,
                    &sollotto_labs_key,
                    &lottery_result_key,
                    &lifetime_token_owner_key,
                    &lifetime_token_mint_key,
                    &vec![
                        (user_1_wallet_key, user_1_lifetime_token_key),
                        (user_2_wallet_key, user_2_lifetime_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_acc,
                    &mut sollotto_rewards_acc,
                    &mut slot_holders_acc,
                    &mut sollotto_labs_acc,
                    &mut lottery_result_acc,
                    &mut lifetime_token_owner_acc,
                    &mut lifetime_token_mint_acc,
                    &mut system_account,
                    &mut rent_acc,
                    &mut user_1_wallet_acc,
                    &mut user_1_lifetime_token_acc,
                    &mut user_2_wallet_acc,
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
                    &prize_pool_key,
                    &sollotto_rewards_key,
                    &slot_holders_key,
                    &sollotto_labs_key,
                    &lottery_result_key,
                    &lifetime_token_owner_key,
                    &lifetime_token_mint_key,
                    &vec![
                        (user_1_wallet_key, user_1_lifetime_token_key),
                        (user_2_wallet_key, user_2_lifetime_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_acc,
                    &mut sollotto_rewards_acc,
                    &mut slot_holders_acc,
                    &mut sollotto_labs_acc,
                    &mut lottery_result_acc,
                    &mut lifetime_token_owner_acc,
                    &mut lifetime_token_mint_acc,
                    &mut system_account,
                    &mut rent_acc,
                    &mut user_1_wallet_acc,
                    &mut user_1_lifetime_token_acc,
                    &mut user_2_wallet_acc,
                    &mut user_2_lifetime_token_acc,
                ],
            )
        );

        // BadCase: InvalidParticipantsAccounts invalid lifetime ticket token amount
        assert_eq!(
            Err(LotteryError::InvalidParticipantsAccounts.into()),
            do_process(
                crate::instruction::reward_winner(
                    &program_id,
                    lottery_id,
                    random_number,
                    &prize_pool_key,
                    &sollotto_rewards_key,
                    &slot_holders_key,
                    &sollotto_labs_key,
                    &lottery_result_key,
                    &lifetime_token_owner_key,
                    &lifetime_token_mint_key,
                    &vec![
                        (user_1_wallet_key, user_1_lifetime_token_key),
                        (user_2_wallet_key, user_2_lifetime_token_key),
                    ],
                )
                .unwrap(),
                vec![
                    &mut prize_pool_acc,
                    &mut sollotto_rewards_acc,
                    &mut slot_holders_acc,
                    &mut sollotto_labs_acc,
                    &mut lottery_result_acc,
                    &mut lifetime_token_owner_acc,
                    &mut lifetime_token_mint_acc,
                    &mut system_account,
                    &mut rent_acc,
                    &mut user_1_wallet_acc,
                    &mut user_1_lifetime_token_acc,
                    &mut user_2_wallet_acc,
                    &mut user_2_lifetime_token_acc,
                ],
            )
        );

        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                amount: ui_amount_to_amount(1.0, 9),
                mint: lifetime_token_mint_key,
                owner: user_1_wallet_key,
                ..Default::default()
            },
            &mut user_1_lifetime_token_acc.data,
        )
        .unwrap();
        spl_token::state::Account::pack(
            Account {
                state: spl_token::state::AccountState::Initialized,
                amount: ui_amount_to_amount(1.0, 9),
                mint: lifetime_token_mint_key,
                owner: user_2_wallet_key,
                ..Default::default()
            },
            &mut user_2_lifetime_token_acc.data,
        )
        .unwrap();

        do_process(
            crate::instruction::reward_winner(
                &program_id,
                lottery_id,
                random_number,
                &prize_pool_key,
                &sollotto_rewards_key,
                &slot_holders_key,
                &sollotto_labs_key,
                &lottery_result_key,
                &lifetime_token_owner_key,
                &lifetime_token_mint_key,
                &vec![
                    (user_1_wallet_key, user_1_lifetime_token_key),
                    (user_2_wallet_key, user_2_lifetime_token_key),
                ],
            )
            .unwrap(),
            vec![
                &mut prize_pool_acc,
                &mut sollotto_rewards_acc,
                &mut slot_holders_acc,
                &mut sollotto_labs_acc,
                &mut lottery_result_acc,
                &mut lifetime_token_owner_acc,
                &mut lifetime_token_mint_acc,
                &mut system_account,
                &mut rent_acc,
                &mut user_1_wallet_acc,
                &mut user_1_lifetime_token_acc,
                &mut user_2_wallet_acc,
                &mut user_2_lifetime_token_acc,
            ],
        )
        .unwrap();

        // Check LotteryResult account
        let lottery_result_data =
            LotteryResultData::unpack_unchecked(&lottery_result_acc.data).unwrap();
        assert_eq!(lottery_result_data.lottery_id, lottery_id);
        assert_eq!(lottery_result_data.winner, user_2_wallet_key);
    }
}
