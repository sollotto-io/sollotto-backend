//! Program state processor
use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::LotteryResultData
};
use spl_token::{
    state::{Mint, Account as SPLAccount},
    ui_amount_to_amount,
    amount_to_ui_amount
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    native_token::{lamports_to_sol, sol_to_lamports},
    program::invoke,
    program_option::COption,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar
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
            LotteryInstruction::RewardWinners { lottery_id, idx, prize_pool } => {
                msg!("Instruction: RewardWinners");
                Self::process_reward_winners(
                    program_id, accounts, lottery_id,
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
        let slot_mint_account       = next_account_info(accounts_iter)?;
        let slot_mint_authority     = next_account_info(accounts_iter)?;
        let sollotto_sol_account    = next_account_info(accounts_iter)?;
        let system_program_account  = next_account_info(accounts_iter)?;
        let spl_token_account       = next_account_info(accounts_iter)?;

        // TODO: Check for signed/writable attributes on fields that require them,

        let user_slot_account = SPLAccount::unpack(
            &user_slot_account.data.borrow()
        )?;
        let slot_mint = Mint::unpack(
            &slot_mint_account.data.borrow()
        )?;

        // Checks to determine if user tries to spoof their SLOT account
        //
        // Assert that provided SLOT account belongs to user
        if user_slot_account.owner != *user_sol_account.owner {
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
        let _user_fqticket_account = SPLAccount::unpack(
            &user_fqticket_account.data.borrow()
        )?;
        let fqtickets_owned = _user_fqticket_account.amount;
        let fqtickets_cap   = user_slot_account.amount;

        if (fqtickets_owned + (amount as u64)) > fqtickets_cap {
            msg!("Requested FQTickets exceeds SLOT amount for the user");
            return Err(LotteryError::SLOTCapExceeded.into());
        }

        let ticket_price = sol_to_lamports(0.1);
        let total_price  = ticket_price * (amount as u64);

        if user_sol_account.lamports() < total_price {
            msg!("User cannot pay for the ticket");
            return Err(ProgramError::InsufficientFunds.into());
        }

        // Take buyers SOL tokens.
        Self::transfer_sol(
            user_sol_account.key,
            sollotto_sol_account.key,
            total_price,
            &[ user_sol_account.clone(), sollotto_sol_account.clone(),
               system_program_account.clone() ]
        )?;

        // Mint the corresponding Fixed-Quantity Tokens for taken SOL.
        invoke(
            &spl_token::instruction::mint_to(
                &spl_token::id(),
                fqticket_mint.key,
                user_fqticket_account.key,
                user_fqticket_account.owner,
                &[],
                amount as u64
            )?,
            &[
                spl_token_account.clone(),
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
        lottery_id: u32,
        idx: u64,
        prize_pool: u64
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();
        let idx = idx as usize;

        let sollotto_sol_account     = next_account_info(accounts_iter)?;
        let sollotto_rewards_account = next_account_info(accounts_iter)?;
        let holder_rewards_account   = next_account_info(accounts_iter)?;
        let sollotto_labs_account    = next_account_info(accounts_iter)?;
        let sollotto_results_account = next_account_info(accounts_iter)?;
        let system_program_account   = next_account_info(accounts_iter)?;
        let participants             = accounts_iter.as_slice();

        if participants.len() < idx {
            msg!("Winner's index exceedes the number of participants");
            return Err(LotteryError::InvalidParticipantsAccounts.into())
        }

        let winner = participants.get(idx).unwrap();

        let sol_prize_pool       = lamports_to_sol(prize_pool);
        let winners_cut          = sol_to_lamports(sol_prize_pool * 0.95);
        let sollotto_rewards_cut = sol_to_lamports(sol_prize_pool * 0.04);
        let holder_rewards_cut   = sol_to_lamports(sol_prize_pool * 0.006);
        let sollotto_labs_cut    = sol_to_lamports(sol_prize_pool * 0.004);

        for ( dest, cut ) in [
            ( winner, winners_cut ),
            ( sollotto_rewards_account, sollotto_rewards_cut ),
            ( holder_rewards_account, holder_rewards_cut ),
            ( sollotto_labs_account, sollotto_labs_cut )
        ] {
            Self::transfer_sol(
                sollotto_sol_account.key,
                dest.key,
                cut,
                &[
                    sollotto_sol_account.clone(),
                    dest.clone(),
                    system_program_account.clone()
                ]
            )?;
        }

        LotteryResultData::pack(
            LotteryResultData {
                lottery_id: lottery_id,
                winner: *winner.key,
            },
            &mut sollotto_results_account.data.borrow_mut()
        );

        Ok(())
    }

    #[inline]
    fn transfer_sol(
        dest: &Pubkey,
        src: &Pubkey,
        lamports: u64,
        accounts: &[AccountInfo]
    ) -> ProgramResult {
        invoke(
            &system_instruction::transfer(dest, src, lamports),
            accounts
        )
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{instruction::Instruction, program_pack::Pack, rent::Rent};
    use solana_sdk::account::{
        create_account_for_test, create_is_signer_account_infos, Account as SolanaAccount,
        ReadableAccount,
    };

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
        let user_key = Pubkey::new_unique();
        let user_fqticket_key = Pubkey::new_unique();
        let mut user_fqticket_acc = SolanaAccount::new(
            account_minimum_balance(),
            SPLAccount::get_packed_len(),
            &user_key
        );
        let user_sol_key = Pubkey::new_unique();
        let mut user_sol_acc = SolanaAccount::new(
            account_minimum_balance(),
            SPLAccount::get_packed_len(),
            &user_key
        );
        let user_slot_key = Pubkey::new_unique();
        let mut user_slot_acc = SolanaAccount::new(
            account_minimum_balance(),
            SPLAccount::get_packed_len(),
            &user_key
        );
        let fqticket_mint_key = Pubkey::new_unique();
        let mut fqticket_mint = SolanaAccount::new(
            mint_minimum_balance(),
            Mint::get_packed_len(),
            &spl_token::id()
        );
        let fqticket_mint_authority_key = Pubkey::new_unique();
        let mut fqticket_mint_authority = SolanaAccount::default();
        let slot_mint_key = Pubkey::new_unique();
        let mut slot_mint = SolanaAccount::new(
            mint_minimum_balance(),
            Mint::get_packed_len(),
            &spl_token::id()
        );
        let slot_mint_authority_key = Pubkey::new_unique();
        let mut slot_mint_authority = SolanaAccount::default();
        let sollotto_sol_key = Pubkey::new_unique();
        let mut sollotto_sol_acc = SolanaAccount::default();
        let mut system_program_acc  = SolanaAccount::default();
        let mut spl_token_acc = SolanaAccount::default();

        Mint::pack(
            Mint {
                mint_authority: COption::Some(fqticket_mint_authority_key),
                supply: ui_amount_to_amount(10., 9),
                decimals: 9,
                is_initialized: true,
                ..Default::default()
            },
            &mut fqticket_mint.data
        );

        Mint::pack(
            Mint {
                mint_authority: COption::Some(slot_mint_authority_key),
                supply: ui_amount_to_amount(10., 9),
                decimals: 9,
                is_initialized: true,
                ..Default::default()
            },
            &mut slot_mint.data
        );

        SPLAccount::pack(
            SPLAccount {
                mint: spl_token::id(),
                owner: user_key,
                amount: 0,
                state: spl_token::state::AccountState::Initialized,
                ..Default::default()
            },
            &mut user_fqticket_acc.data
        );

        SPLAccount::pack(
            SPLAccount {
                mint: slot_mint_key,
                owner: user_key,
                amount: 0,
                state: spl_token::state::AccountState::Initialized,
                ..Default::default()
            },
            &mut user_slot_acc.data
        )?;

        assert_eq!(
            Err(LotteryError::SLOTCapExceeded.into()),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    5,
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
                owner: user_key,
                amount: ui_amount_to_amount(10., 9),
                state: spl_token::state::AccountState::Initialized,
                ..Default::default()
            },
            &mut user_slot_acc.data
        )?;

        assert_eq!(
            Err(ProgramError::InsufficientFunds),
            do_process(
                crate::instruction::purchase_ticket(
                    &program_id,
                    5,
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
            &user_key
        );

        let fqticket_account_data = SPLAccount::unpack(
            &user_fqticket_acc.data.as_slice()
        )?;
        msg!("{:?}", user_fqticket_acc);
        msg!("{:?}", fqticket_account_data);

        /*
        do_process(
            spl_token::instruction::mint_to(
                &program_id,
                &fqticket_mint_key,
                &user_fqticket_key,
                &user_key,
                // fqticket_mint_authority.key,
                &[],
                10000 as u64
            )?,
            vec![
                &mut spl_token_acc.clone(),
                &mut fqticket_mint.clone(),
                &mut user_fqticket_acc.clone(),
                &mut fqticket_mint_authority.clone()
            ]
        )?;
        */


        do_process(
            crate::instruction::purchase_ticket(
                &program_id,
                5,
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
        )?;

        let fqticket_account_data = SPLAccount::unpack(
            &user_fqticket_acc.data.as_slice()
        )?;
        msg!("{:?}", user_fqticket_acc);
        msg!("{:?}", fqticket_account_data);
        panic!("");

        Ok(())
    }
}
