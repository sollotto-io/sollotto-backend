//! Instruction types
use crate::error::LotteryError::InvalidInstruction;
use crate::processor::check_program_account;
use solana_program::{
    instruction::{AccountMeta, Instruction},
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
    sysvar,
};
use std::{convert::TryInto, mem::size_of};

/// Instructions supported by the Lottery program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum LotteryInstruction {
    /// Initialize lottery data with basic information.
    /// Creates staking pool token Mint and
    /// associated token account for staking pool token.
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    /// 1. `[writable]` Staking pool token Mint
    /// 2. `[writable]` Staking pool token associated account
    /// 3. `[]` Rent sysvar
    /// 4. `[]` SPL Token program
    InitLottery {
        staking_pool_wallet: Pubkey,
        rewards_wallet: Pubkey,
        slot_holders_rewards_wallet: Pubkey,
        sollotto_labs_wallet: Pubkey,
    },

    /// User deposits amount in lamports and gets equivalent of
    /// Sollotto SOL Staking pool token
    ///
    /// Accounts expected by this instruction:
    // TODO: Fix it with liquidity stake pool information
    /// 0. `[writable, signer]` Lottery data account (also onwer for staking pool token mint)
    /// 1. `[writable]` Staking pool token mint
    /// 2. `[writable, signer]` User funding account (must be a system account)
    /// 3. `[writable]` User staking pool token associated account
    /// 4. `[writable]` Sollotto staking pool wallet (TODO: liquidity pool here)
    /// 5. `[]` SPL Token program
    /// 6. `[]` System program account
    Deposit { amount: u64 },

    /// User undeposits amount of Sollotto SOL Staking pool token
    /// and gets equivalent of SOL
    ///
    /// Accounts expected by this instruction:
    // TODO: Fix it with liquidity stake pool information
    /// 0. `[writable, signer]` Lottery data account (also onwer for staking pool token mint)
    /// 1. `[writable]` Staking pool token mint
    /// 2. `[writable, signer]` User funding account (must be a system account)
    /// 3. `[writable]` User staking pool token associated account
    /// 4. `[writable, signer]` Sollotto staking pool wallet (TODO: liquidity pool here)
    /// 5. `[]` SPL Token program
    /// 6. `[]` System program account
    Undeposit { amount: u64 },

    /// Get the winner`s wallet and pay reward from prize pool
    ///
    /// Accounts expected by this instruction:
    // TODO: Fix it with liquidity stake pool information
    /// 0. `[signer]` Lottery data account (also onwer for staking pool token mint)
    /// 1. `[writable]` Lottery result data account
    /// 2. `[writable]` Winner wallet (must be a system account)
    /// 3. `[writable, signer]` Sollotto staking pool wallet (TODO: liquidity pool here)
    /// 4. `[writable]` Sollotto Foundation Rewards wallet (must be a system account)
    /// 5. `[writable]` SLOT Holders rewards wallet (must be a system account)
    /// 6. `[writable]` Sollotto labs wallet (must be a system account)
    /// 7. `[]` System program account
    RewardWinner { lottery_id: u32 },

    /// Update wallets pubkeys in lottery data account
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    UpdateLotteryWallets {
        staking_pool_wallet: Pubkey,
        staking_pool_token_mint: Pubkey,
        rewards_wallet: Pubkey,
        slot_holders_rewards_wallet: Pubkey,
        sollotto_labs_wallet: Pubkey,
    },
}

impl LotteryInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (staking_pool_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (slot_holders_rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (sollotto_labs_wallet, _) = Self::unpack_pubkey(rest).unwrap();

                Self::InitLottery {
                    staking_pool_wallet,
                    rewards_wallet,
                    slot_holders_rewards_wallet,
                    sollotto_labs_wallet,
                }
            }

            1 | 2 => {
                let (amount, _) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                match tag {
                    1 => Self::Deposit { amount: amount },
                    2 => Self::Undeposit { amount: amount },
                    _ => unreachable!(),
                }
            }

            3 => {
                let (lottery_id, _) = rest.split_at(4);
                let lottery_id = lottery_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                Self::RewardWinner { lottery_id }
            }

            4 => {
                let (staking_pool_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (staking_pool_token_mint, rest) = Self::unpack_pubkey(rest).unwrap();
                let (rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (slot_holders_rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (sollotto_labs_wallet, _) = Self::unpack_pubkey(rest).unwrap();

                Self::UpdateLotteryWallets {
                    staking_pool_wallet,
                    staking_pool_token_mint,
                    rewards_wallet,
                    slot_holders_rewards_wallet,
                    sollotto_labs_wallet,
                }
            }

            _ => return Err(InvalidInstruction.into()),
        })
    }

    /// Packs a LotteryInstruction into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::InitLottery {
                staking_pool_wallet,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
            } => {
                buf.push(0);
                buf.extend_from_slice(staking_pool_wallet.as_ref());
                buf.extend_from_slice(rewards_wallet.as_ref());
                buf.extend_from_slice(slot_holders_rewards_wallet.as_ref());
                buf.extend_from_slice(sollotto_labs_wallet.as_ref());
            }

            Self::Deposit { amount } => {
                buf.push(1);
                buf.extend_from_slice(&amount.to_le_bytes());
            }

            Self::Undeposit { amount } => {
                buf.push(2);
                buf.extend_from_slice(&amount.to_le_bytes());
            }

            Self::RewardWinner { lottery_id } => {
                buf.push(3);
                buf.extend_from_slice(&lottery_id.to_le_bytes());
            }

            Self::UpdateLotteryWallets {
                staking_pool_wallet,
                staking_pool_token_mint,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
            } => {
                buf.push(4);
                buf.extend_from_slice(staking_pool_wallet.as_ref());
                buf.extend_from_slice(staking_pool_token_mint.as_ref());
                buf.extend_from_slice(rewards_wallet.as_ref());
                buf.extend_from_slice(slot_holders_rewards_wallet.as_ref());
                buf.extend_from_slice(sollotto_labs_wallet.as_ref());
            }
        };
        buf
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() < 32 {
            msg!("Pubkey cannot be unpacked");
            return Err(InvalidInstruction.into());
        }
        let (key, rest) = input.split_at(32);
        let pk = Pubkey::new(key);
        Ok((pk, rest))
    }
}

/// Creates a `InitLottery` instruction
pub fn initialize_lottery(
    program_id: &Pubkey,
    staking_pool_wallet: &Pubkey,
    staking_pool_token_mint: &Pubkey,
    staking_pool_token_account: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_rewards_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::InitLottery {
        staking_pool_wallet: *staking_pool_wallet,
        rewards_wallet: *rewards_wallet,
        slot_holders_rewards_wallet: *slot_holders_rewards_wallet,
        sollotto_labs_wallet: *sollotto_labs_wallet,
    }
    .pack();

    let mut accounts = Vec::with_capacity(4);
    accounts.push(AccountMeta::new(*lottery_authority, true));
    accounts.push(AccountMeta::new(*staking_pool_token_mint, false));
    accounts.push(AccountMeta::new(*staking_pool_token_account, false));
    accounts.push(AccountMeta::new_readonly(sysvar::rent::id(), false));
    accounts.push(AccountMeta::new_readonly(spl_token::id(), false));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a `Deposit` instruction
pub fn deposit(
    program_id: &Pubkey,
    amount: u64,
    staking_pool_token_mint: &Pubkey,
    user_staking_pool_token_account: &Pubkey,
    staking_pool_wallet: &Pubkey,
    user_authority: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::Deposit { amount }.pack();

    let mut accounts = Vec::with_capacity(7);
    accounts.push(AccountMeta::new(*lottery_authority, true));
    accounts.push(AccountMeta::new(*staking_pool_token_mint, false));
    accounts.push(AccountMeta::new(*user_authority, true));
    accounts.push(AccountMeta::new(*user_staking_pool_token_account, false));
    accounts.push(AccountMeta::new(*staking_pool_wallet, false));
    accounts.push(AccountMeta::new_readonly(spl_token::id(), false));
    accounts.push(AccountMeta::new_readonly(
        solana_program::system_program::id(),
        false,
    ));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a `Undeposit` instruction
pub fn undeposit(
    program_id: &Pubkey,
    amount: u64,
    staking_pool_token_mint: &Pubkey,
    user_staking_pool_token_account: &Pubkey,
    staking_pool_wallet: &Pubkey,
    user_authority: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::Undeposit { amount }.pack();

    let mut accounts = Vec::with_capacity(7);
    accounts.push(AccountMeta::new(*lottery_authority, true));
    accounts.push(AccountMeta::new(*staking_pool_token_mint, false));
    accounts.push(AccountMeta::new(*user_authority, true));
    accounts.push(AccountMeta::new(*user_staking_pool_token_account, false));
    accounts.push(AccountMeta::new(*staking_pool_wallet, true));
    accounts.push(AccountMeta::new_readonly(spl_token::id(), false));
    accounts.push(AccountMeta::new_readonly(
        solana_program::system_program::id(),
        false,
    ));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a `RewardWinner` instruction
pub fn reward_winner(
    program_id: &Pubkey,
    lottery_id: u32,
    lottery_result: &Pubkey,
    winner_wallet: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    staking_pool_wallet: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::RewardWinner { lottery_id }.pack();

    let mut accounts = Vec::with_capacity(8);
    accounts.push(AccountMeta::new_readonly(*lottery_authority, true));
    accounts.push(AccountMeta::new(*lottery_result, false));
    accounts.push(AccountMeta::new(*winner_wallet, false));
    accounts.push(AccountMeta::new(*staking_pool_wallet, true));
    accounts.push(AccountMeta::new(*rewards_wallet, false));
    accounts.push(AccountMeta::new(*slot_holders_wallet, false));
    accounts.push(AccountMeta::new(*sollotto_labs_wallet, false));
    accounts.push(AccountMeta::new_readonly(
        solana_program::system_program::id(),
        false,
    ));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a `UpdateLotteryWallets` instruction
pub fn update_lottery_wallets(
    program_id: &Pubkey,
    staking_pool_wallet: &Pubkey,
    staking_pool_token_mint: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_rewards_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::UpdateLotteryWallets {
        staking_pool_wallet: *staking_pool_wallet,
        staking_pool_token_mint: *staking_pool_token_mint,
        rewards_wallet: *rewards_wallet,
        slot_holders_rewards_wallet: *slot_holders_rewards_wallet,
        sollotto_labs_wallet: *sollotto_labs_wallet,
    }
    .pack();

    let mut accounts = Vec::with_capacity(1);
    accounts.push(AccountMeta::new(*lottery_authority, true));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
