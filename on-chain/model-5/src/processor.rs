//! Program state processor
use crate::{error::LotteryError, instruction::LotteryInstruction, state::LotteryResultData};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    native_token::{lamports_to_sol, sol_to_lamports},
    program::invoke,
    program_error::ProgramError,
    program_option::COption,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction,
};
use spl_token::{
    amount_to_ui_amount,
    state::{Account as SPLAccount, Mint},
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
                Self::process_ticket_purchase(program_id, accounts, amount)
            }
            LotteryInstruction::RewardWinners {
                lottery_id,
                idx,
                prize_pool,
            } => {
                msg!("Instruction: RewardWinners");
                Self::process_reward_winners(program_id, accounts, lottery_id, idx, prize_pool)
            }
        }
    }

    pub fn process_ticket_purchase(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        amount: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let user_fqticket_account = next_account_info(accounts_iter)?;
        let user_sol_account = next_account_info(accounts_iter)?;
        let user_slot_account = next_account_info(accounts_iter)?;
        let fqticket_mint = next_account_info(accounts_iter)?;
        let fqticket_mint_authority = next_account_info(accounts_iter)?;
        let slot_mint_account = next_account_info(accounts_iter)?;
        let slot_mint_authority = next_account_info(accounts_iter)?;
        let sollotto_sol_account = next_account_info(accounts_iter)?;
        let system_program_account = next_account_info(accounts_iter)?;
        let spl_token_account = next_account_info(accounts_iter)?;

        if !user_sol_account.is_signer {
            msg!("Missing user wallet signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !fqticket_mint_authority.is_signer {
            msg!("Missing FQTicket Mint Authority signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        if !slot_mint_authority.is_signer {
            msg!("Missing SLOT Mint Authority signature");
            return Err(ProgramError::MissingRequiredSignature);
        }

        let user_slot_account = SPLAccount::unpack(&user_slot_account.data.borrow())?;
        let slot_mint = Mint::unpack(&slot_mint_account.data.borrow())?;
        let fqticket_mint_data = Mint::unpack(&fqticket_mint.data.borrow())?;

        // Checks to determine if user tries to spoof their SLOT account
        //
        // Assert that provided SLOT account belongs to user
        if user_slot_account.owner != *user_sol_account.key {
            msg!("User provided incorrect SLOT account");
            return Err(LotteryError::InvalidSLOTAccount.into());
        }
        // Assert that provided user's SLOT account has the corrent Mint
        if user_slot_account.mint != *slot_mint_account.key {
            msg!("User provided incorrect SLOT account");
            return Err(LotteryError::InvalidSLOTAccount.into());
        }
        // If provided SLOT Mint account has no mint_authority, return Err
        if let COption::Some(mint_authority) = slot_mint.mint_authority {
            if mint_authority != *slot_mint_authority.key {
                msg!("SLOT Mint has incorrect mint_authority");
                return Err(LotteryError::InvalidSLOTAccount.into());
            }
        } else {
            msg!("SLOT Mint has incorrect mint_authority");
            return Err(LotteryError::InvalidSLOTAccount.into());
        }

        // Temporary account variable to check user's fqticket balance
        let _user_fqticket_account = SPLAccount::unpack(&user_fqticket_account.data.borrow())?;
        let fqtickets_owned =
            amount_to_ui_amount(_user_fqticket_account.amount, fqticket_mint_data.decimals);
        let fqtickets_cap = amount_to_ui_amount(user_slot_account.amount, slot_mint.decimals);

        let amount_float = amount_to_ui_amount(amount, fqticket_mint_data.decimals);
        let ticket_price = sol_to_lamports(0.1);
        let total_price = (ticket_price as f64) * amount_float;

        if (fqtickets_owned + amount_float) > fqtickets_cap {
            msg!("Requested FQTickets exceeds SLOT amount for the user");
            return Err(LotteryError::SLOTCapExceeded.into());
        }

        if (user_sol_account.lamports() as f64) < total_price {
            msg!("User cannot pay for the ticket");
            return Err(ProgramError::InsufficientFunds);
        }

        // Transfer SOL tokens from the buyer's wallet.
        Self::transfer_sol(
            user_sol_account.key,
            sollotto_sol_account.key,
            total_price as u64,
            &[
                user_sol_account.clone(),
                sollotto_sol_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        // Mint the corresponding Fixed-Quantity Tokens for taken SOL.
        invoke(
            &spl_token::instruction::mint_to(
                &spl_token::id(),
                fqticket_mint.key,
                user_fqticket_account.key,
                fqticket_mint_authority.key,
                &[fqticket_mint_authority.key],
                amount as u64,
            )?,
            &[
                spl_token_account.clone(),
                fqticket_mint.clone(),
                user_fqticket_account.clone(),
                fqticket_mint_authority.clone(),
            ],
        )?;

        Ok(())
    }

    pub fn process_reward_winners(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        lottery_id: u32,
        idx: u64,
        prize_pool: u64,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let idx = idx as usize;

        let sollotto_sol_account = next_account_info(accounts_iter)?;
        let sollotto_rewards_account = next_account_info(accounts_iter)?;
        let holder_rewards_account = next_account_info(accounts_iter)?;
        let sollotto_labs_account = next_account_info(accounts_iter)?;
        let sollotto_results_account = next_account_info(accounts_iter)?;
        let system_program_account = next_account_info(accounts_iter)?;
        let participants = accounts_iter.as_slice();

        if (participants.len() / 2) <= idx {
            msg!("Winner's index exceedes the number of participants");
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        if prize_pool == 0 {
            msg!("Empty prize pool");
            return Err(LotteryError::EmptyPrizePool.into());
        }

        let winner = participants.get(idx * 2).unwrap();

        let winner_fq_acc =
            SPLAccount::unpack(&participants.get(idx * 2 + 1).unwrap().data.borrow())?;

        if winner_fq_acc.amount < 1 {
            return Err(LotteryError::NotEnoughFQTokens.into());
        }

        let sol_prize_pool = lamports_to_sol(prize_pool);
        let winners_cut = sol_to_lamports(sol_prize_pool * 0.95);
        let sollotto_rewards_cut = sol_to_lamports(sol_prize_pool * 0.04);
        let holder_rewards_cut = sol_to_lamports(sol_prize_pool * 0.006);
        let sollotto_labs_cut = sol_to_lamports(sol_prize_pool * 0.004);

        Self::transfer_sol(
            sollotto_sol_account.key,
            winner.key,
            winners_cut,
            &[
                sollotto_sol_account.clone(),
                winner.clone(),
                system_program_account.clone(),
            ],
        )?;

        Self::transfer_sol(
            sollotto_sol_account.key,
            sollotto_rewards_account.key,
            sollotto_rewards_cut,
            &[
                sollotto_sol_account.clone(),
                sollotto_rewards_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        Self::transfer_sol(
            sollotto_sol_account.key,
            holder_rewards_account.key,
            holder_rewards_cut,
            &[
                sollotto_sol_account.clone(),
                holder_rewards_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        Self::transfer_sol(
            sollotto_sol_account.key,
            sollotto_labs_account.key,
            sollotto_labs_cut,
            &[
                sollotto_sol_account.clone(),
                sollotto_labs_account.clone(),
                system_program_account.clone(),
            ],
        )?;

        LotteryResultData::pack(
            LotteryResultData {
                lottery_id: lottery_id,
                winner: *winner.key,
            },
            &mut sollotto_results_account.data.borrow_mut(),
        )?;

        Ok(())
    }

    #[inline]
    fn transfer_sol(
        dest: &Pubkey,
        src: &Pubkey,
        lamports: u64,
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        invoke(&system_instruction::transfer(dest, src, lamports), accounts)
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{instruction::Instruction, program_pack::Pack, rent::Rent};
    use solana_sdk::account::{create_is_signer_account_infos, Account as SolanaAccount};
    use spl_token::ui_amount_to_amount;

    fn mint_minimum_balance() -> u64 {
        Rent::default().minimum_balance(spl_token::state::Mint::get_packed_len())
    }

    fn account_minimum_balance() -> u64 {
        Rent::default().minimum_balance(spl_token::state::Account::get_packed_len())
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
    fn test_ticket_purchase() -> Result<(), Box<dyn std::error::Error>> {
        let program_id = super::id();
        let user_sol_key = Pubkey::new_unique();
        let mut user_sol_acc = SolanaAccount::new(
            account_minimum_balance(),
            SPLAccount::get_packed_len(),
            &user_sol_key,
        );
        let user_fqticket_key = Pubkey::new_unique();
        let mut user_fqticket_acc = SolanaAccount::new(
            account_minimum_balance(),
            SPLAccount::get_packed_len(),
            &user_sol_key,
        );
        let user_slot_key = Pubkey::new_unique();
        let mut user_slot_acc = SolanaAccount::new(
            account_minimum_balance(),
            SPLAccount::get_packed_len(),
            &user_sol_key,
        );
        let fqticket_mint_key = Pubkey::new_unique();
        let mut fqticket_mint = SolanaAccount::new(
            mint_minimum_balance(),
            Mint::get_packed_len(),
            &spl_token::id(),
        );
        let fqticket_mint_authority_key = Pubkey::new_unique();
        let mut fqticket_mint_authority = SolanaAccount::default();
        let slot_mint_key = Pubkey::new_unique();
        let mut slot_mint = SolanaAccount::new(
            mint_minimum_balance(),
            Mint::get_packed_len(),
            &spl_token::id(),
        );
        let slot_mint_authority_key = Pubkey::new_unique();
        let mut slot_mint_authority = SolanaAccount::default();
        let sollotto_sol_key = Pubkey::new_unique();
        let mut sollotto_sol_acc = SolanaAccount::default();
        let mut system_program_acc = SolanaAccount::default();
        let mut spl_token_acc = SolanaAccount::default();

        Mint::pack(
            Mint {
                mint_authority: COption::Some(fqticket_mint_authority_key),
                supply: ui_amount_to_amount(10., 9),
                decimals: 9,
                is_initialized: true,
                ..Default::default()
            },
            &mut fqticket_mint.data,
        )?;

        Mint::pack(
            Mint {
                mint_authority: COption::Some(slot_mint_authority_key),
                supply: ui_amount_to_amount(10., 9),
                decimals: 9,
                is_initialized: true,
                ..Default::default()
            },
            &mut slot_mint.data,
        )?;

        SPLAccount::pack(
            SPLAccount {
                mint: spl_token::id(),
                owner: user_sol_key,
                amount: 0,
                state: spl_token::state::AccountState::Initialized,
                ..Default::default()
            },
            &mut user_fqticket_acc.data,
        )?;

        SPLAccount::pack(
            SPLAccount {
                mint: slot_mint_key,
                owner: user_sol_key,
                amount: 0,
                state: spl_token::state::AccountState::Initialized,
                ..Default::default()
            },
            &mut user_slot_acc.data,
        )?;

        assert_eq!(
            Err(LotteryError::SLOTCapExceeded.into()),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    ui_amount_to_amount(5., 9),
                    &user_fqticket_key,
                    &user_sol_key,
                    &user_slot_key,
                    &fqticket_mint_key,
                    &fqticket_mint_authority_key,
                    &slot_mint_key,
                    &slot_mint_authority_key,
                    &sollotto_sol_key
                )
                .unwrap(),
                vec![
                    &mut user_fqticket_acc,
                    &mut user_sol_acc,
                    &mut user_slot_acc,
                    &mut fqticket_mint,
                    &mut fqticket_mint_authority,
                    &mut slot_mint,
                    &mut slot_mint_authority,
                    &mut sollotto_sol_acc,
                    &mut system_program_acc,
                    &mut spl_token_acc,
                ]
            )
        );
        msg!("SLOTCapExceeded test passed...");

        SPLAccount::pack(
            SPLAccount {
                mint: slot_mint_key,
                owner: user_sol_key,
                amount: ui_amount_to_amount(10., 9),
                state: spl_token::state::AccountState::Initialized,
                ..Default::default()
            },
            &mut user_slot_acc.data,
        )?;

        assert_eq!(
            Err(ProgramError::InsufficientFunds),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    ui_amount_to_amount(5., 9),
                    &user_fqticket_key,
                    &user_sol_key,
                    &user_slot_key,
                    &fqticket_mint_key,
                    &fqticket_mint_authority_key,
                    &slot_mint_key,
                    &slot_mint_authority_key,
                    &sollotto_sol_key
                )
                .unwrap(),
                vec![
                    &mut user_fqticket_acc,
                    &mut user_sol_acc,
                    &mut user_slot_acc,
                    &mut fqticket_mint,
                    &mut fqticket_mint_authority,
                    &mut slot_mint,
                    &mut slot_mint_authority,
                    &mut sollotto_sol_acc,
                    &mut system_program_acc,
                    &mut spl_token_acc,
                ]
            )
        );
        msg!("InsufficientFunds test passed...");

        let mut user_sol_acc = SolanaAccount::new(
            sol_to_lamports(2.),
            SPLAccount::get_packed_len(),
            &user_sol_key,
        );

        assert_eq!(
            Ok(()),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    ui_amount_to_amount(5., 9),
                    &user_fqticket_key,
                    &user_sol_key,
                    &user_slot_key,
                    &fqticket_mint_key,
                    &fqticket_mint_authority_key,
                    &slot_mint_key,
                    &slot_mint_authority_key,
                    &sollotto_sol_key,
                )
                .unwrap(),
                vec![
                    &mut user_fqticket_acc,
                    &mut user_sol_acc,
                    &mut user_slot_acc,
                    &mut fqticket_mint,
                    &mut fqticket_mint_authority,
                    &mut slot_mint,
                    &mut slot_mint_authority,
                    &mut sollotto_sol_acc,
                    &mut system_program_acc,
                    &mut spl_token_acc,
                ],
            )
        );

        Ok(())
    }

    #[test]
    fn test_reward_winners() -> Result<(), Box<dyn std::error::Error>> {
        let program_id = super::id();
        let lottery_id = 0xff;
        let mut winning_idx = 5;
        let prize_pool = sol_to_lamports(5.);
        let sollotto_key = Pubkey::new_unique();
        let sollotto_sol_key = Pubkey::new_unique();
        let mut sollotto_sol_acc =
            SolanaAccount::new(prize_pool, SPLAccount::get_packed_len(), &sollotto_key);
        let sollotto_rewards_key = Pubkey::new_unique();
        let mut sollotto_rewards_acc = SolanaAccount::default();
        let slot_holder_rewards_key = Pubkey::new_unique();
        let mut slot_holder_rewards_acc = SolanaAccount::default();
        let sollotto_labs_key = Pubkey::new_unique();
        let mut sollotto_labs_acc = SolanaAccount::default();
        let sollotto_result_key = Pubkey::new_unique();
        let mut sollotto_result_acc = SolanaAccount::new(
            account_minimum_balance(),
            LotteryResultData::get_packed_len(),
            &sollotto_key,
        );
        LotteryResultData::pack(LotteryResultData::default(), &mut sollotto_result_acc.data)?;
        let mut system_program_acc = SolanaAccount::default();

        let participant_key0 = Pubkey::new_unique();
        let participant_key1 = Pubkey::new_unique();
        let participant_key2 = Pubkey::new_unique();

        let participant_fq_key0 = Pubkey::new_unique();
        let participant_fq_key1 = Pubkey::new_unique();
        let participant_fq_key2 = Pubkey::new_unique();

        let mut participant_acc0 = SolanaAccount::default();
        let mut participant_acc1 = SolanaAccount::default();
        let mut participant_acc2 = SolanaAccount::new(
            account_minimum_balance(),
            SPLAccount::get_packed_len(),
            &participant_key2,
        );

        let mut participant_fq_acc0 = SolanaAccount::default();
        let mut participant_fq_acc1 = SolanaAccount::default();
        let mut participant_fq_acc2 = SolanaAccount::new(
            account_minimum_balance(),
            SPLAccount::get_packed_len(),
            &participant_key2,
        );

        SPLAccount::pack(
            SPLAccount {
                mint: spl_token::id(),
                owner: participant_key2,
                amount: 3,
                state: spl_token::state::AccountState::Initialized,
                ..Default::default()
            },
            &mut participant_fq_acc2.data,
        )?;

        assert_eq!(
            Err(ProgramError::NotEnoughAccountKeys),
            do_process(
                crate::instruction::reward_winners(
                    &program_id,
                    lottery_id,
                    winning_idx,
                    prize_pool,
                    &sollotto_sol_key,
                    &sollotto_rewards_key,
                    &slot_holder_rewards_key,
                    &sollotto_labs_key,
                    &sollotto_result_key,
                    &vec![
                        (participant_key0, participant_fq_key0),
                        (participant_key1, participant_fq_key1),
                        (participant_key2, participant_fq_key2)
                    ]
                )
                .unwrap(),
                vec![
                    &mut sollotto_sol_acc,
                    &mut sollotto_rewards_acc,
                    &mut slot_holder_rewards_acc,
                    &mut sollotto_labs_acc,
                    &mut sollotto_result_acc,
                    &mut system_program_acc,
                    &mut participant_acc0,
                    &mut participant_fq_acc0,
                    &mut participant_acc1,
                    &mut participant_fq_acc1,
                    &mut participant_acc2,
                    &mut participant_fq_acc2
                ]
            )
        );
        msg!("NotEnoughAccountKeys test passed...");

        winning_idx = 2;

        assert_eq!(
            Ok(()),
            do_process(
                crate::instruction::reward_winners(
                    &program_id,
                    lottery_id,
                    winning_idx,
                    prize_pool,
                    &sollotto_sol_key,
                    &sollotto_rewards_key,
                    &slot_holder_rewards_key,
                    &sollotto_labs_key,
                    &sollotto_result_key,
                    &vec![
                        (participant_key0, participant_fq_key0),
                        (participant_key1, participant_fq_key1),
                        (participant_key2, participant_fq_key2)
                    ]
                )
                .unwrap(),
                vec![
                    &mut sollotto_sol_acc,
                    &mut sollotto_rewards_acc,
                    &mut slot_holder_rewards_acc,
                    &mut sollotto_labs_acc,
                    &mut sollotto_result_acc,
                    &mut system_program_acc,
                    &mut participant_acc0,
                    &mut participant_fq_acc0,
                    &mut participant_acc1,
                    &mut participant_fq_acc1,
                    &mut participant_acc2,
                    &mut participant_fq_acc2
                ]
            )
        );

        let _lottery_res = LotteryResultData::unpack(&sollotto_result_acc.data.as_slice())?;

        assert!(
            (_lottery_res.lottery_id == lottery_id) && (_lottery_res.winner == participant_key2)
        );

        Ok(())
    }
}
