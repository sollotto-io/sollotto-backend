//! Instruction types
use crate::check_program_account;
use crate::error::LotteryError::InvalidInstruction;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    pubkey::Pubkey,
};
use std::{convert::TryInto, mem::size_of};

/// Instructions supported by the Lottery program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum LotteryInstruction {
    /// User deposits amount of custom SPL Token into staking pool and
    /// gets equivalent of SPL Token Staking pool token
    ///
    /// Accounts expected by this instruction:
    /// 0. `[writable, signer]` User wallet
    /// 1. `[writable, signer]` Sollotto Staking Pool authority
    /// 2. `[writable]` User Custom token Account
    /// 3. `[writable]` User Sollotto Staking pool token Account
    /// 4. `[writable]` Sollotto Staking Pool Custom token Account
    /// 5. `[writable]` SPL Token Staking pool token mint
    /// 6. `[]` SPL Token program
    Deposit { amount: u64 },

    /// User unpools amount of SPL Token Staking pool token
    /// and gets equivalent of Custom SPL token back
    ///
    /// Accounts expected by this instruction:
    // TODO: accs
    /// 5. `[]` SPL Token program
    Unpool { amount: u64 },

    /// Get the random number, find winner and pay reward from prize pool.
    /// 30% of the prize pool pays to the charity.
    /// Lottery id, winner's wallet are recorded into chain.
    ///
    /// Accounts expected by this instruction:
    /// TODO: accs
    /// 7. `[]` SPL Token program
    RewardWinner { lottery_id: u32, random_number: u32 },
}

impl LotteryInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 | 1 => {
                let (amount, _) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                match tag {
                    0 => Self::Deposit { amount: amount },
                    1 => Self::Unpool { amount: amount },
                    _ => unreachable!(),
                }
            }

            2 => {
                let (lottery_id, rest) = rest.split_at(4);
                let lottery_id = lottery_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (random_number, _) = rest.split_at(4);
                let random_number = random_number
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                Self::RewardWinner {
                    lottery_id,
                    random_number,
                }
            }

            _ => return Err(InvalidInstruction.into()),
        })
    }

    /// Packs a LotteryInstruction into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::Deposit { amount } => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
            }

            Self::Unpool { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }

            Self::RewardWinner {
                lottery_id,
                random_number,
            } => {
                buf.push(2);
                buf.extend_from_slice(&lottery_id.to_le_bytes());
                buf.extend_from_slice(&random_number.to_le_bytes());
            }
        };
        buf
    }
}

/// Creates a `Deposit` instruction
pub fn deposit(
    program_id: &Pubkey,
    amount: u64,
    user_authority: &Pubkey,
    staking_pool_authoruty: &Pubkey,
    user_token_account: &Pubkey,
    user_staking_pool_account: &Pubkey,
    staking_pool_token_account: &Pubkey,
    staking_pool_token_mint: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::Deposit { amount }.pack();

    let mut accounts = Vec::with_capacity(7);
    accounts.push(AccountMeta::new(*user_authority, true));
    accounts.push(AccountMeta::new(*staking_pool_authoruty, true));
    accounts.push(AccountMeta::new(*user_token_account, false));
    accounts.push(AccountMeta::new(*user_staking_pool_account, false));
    accounts.push(AccountMeta::new(*staking_pool_token_account, false));
    accounts.push(AccountMeta::new(*staking_pool_token_mint, false));
    accounts.push(AccountMeta::new_readonly(spl_token::id(), false));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

// /// Creates a `Undeposit` instruction
// TODO:
// pub fn undeposit(
//     program_id: &Pubkey,
//     amount: u64,
//     staking_pool_token_mint: &Pubkey,
//     user_staking_pool_token_account: &Pubkey,
//     staking_pool_wallet: &Pubkey,
//     user_authority: &Pubkey,
//     lottery_authority: &Pubkey,
// ) -> Result<Instruction, ProgramError> {
//     check_program_account(program_id)?;
//     let data = LotteryInstruction::Undeposit { amount }.pack();

//     let mut accounts = Vec::with_capacity(7);
//     accounts.push(AccountMeta::new(*lottery_authority, true));
//     accounts.push(AccountMeta::new(*staking_pool_token_mint, false));
//     accounts.push(AccountMeta::new(*user_authority, true));
//     accounts.push(AccountMeta::new(*user_staking_pool_token_account, false));
//     accounts.push(AccountMeta::new(*staking_pool_wallet, true));
//     accounts.push(AccountMeta::new_readonly(spl_token::id(), false));
//     accounts.push(AccountMeta::new_readonly(
//         solana_program::system_program::id(),
//         false,
//     ));

//     Ok(Instruction {
//         program_id: *program_id,
//         accounts,
//         data,
//     })
// }

// /// Creates a `RewardWinner` instruction
// TODO:
// pub fn reward_winner(
//     program_id: &Pubkey,
//     lottery_id: u32,
//     lottery_result: &Pubkey,
//     winner_wallet: &Pubkey,
//     rewards_wallet: &Pubkey,
//     slot_holders_wallet: &Pubkey,
//     sollotto_labs_wallet: &Pubkey,
//     staking_pool_wallet: &Pubkey,
//     lottery_authority: &Pubkey,
// ) -> Result<Instruction, ProgramError> {
//     check_program_account(program_id)?;
//     let data = LotteryInstruction::RewardWinner { lottery_id }.pack();

//     let mut accounts = Vec::with_capacity(8);
//     accounts.push(AccountMeta::new_readonly(*lottery_authority, true));
//     accounts.push(AccountMeta::new(*lottery_result, false));
//     accounts.push(AccountMeta::new(*winner_wallet, false));
//     accounts.push(AccountMeta::new(*staking_pool_wallet, true));
//     accounts.push(AccountMeta::new(*rewards_wallet, false));
//     accounts.push(AccountMeta::new(*slot_holders_wallet, false));
//     accounts.push(AccountMeta::new(*sollotto_labs_wallet, false));
//     accounts.push(AccountMeta::new_readonly(
//         solana_program::system_program::id(),
//         false,
//     ));

//     Ok(Instruction {
//         program_id: *program_id,
//         accounts,
//         data,
//     })
// }
