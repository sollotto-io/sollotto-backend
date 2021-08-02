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
    /// User purchases new ticket.
    /// Accounts expected by this instruction:
    ///
    /// 0.         `[writable]` User FQTicket account
    /// 1. `[writable, signer]` User SOL account
    /// 2.                 `[]` User SLOT account
    /// 3.         `[writable]` FQTicket Mint
    /// 4.           `[signer]` FQTicket mint_authority
    /// 5.                 `[]` SLOT Mint
    /// 6.           `[signer]` SLOT mint_authority
    /// 7.         `[writable]` Sollotto SOL account (must be a system account)
    /// 8.                 `[]` System program account
    /// 9.                 `[]` SPL Token account
    PurchaseTicket {
        amount: u32
    },

    /// Rewarding the winners determined by indexing accounts with `idx`.
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Sollotto SOL account (must be system account)
    /// 1.         `[writable]` Sollotto Rewards account (must be system account)
    /// 2.         `[writable]` SLOT Holder Rewards account (must be system account)
    /// 3.         `[writable]` Sollotto labs account (must be system account)
    /// 4.         `[writable]` Sollotto Result account
    /// 5.                 `[]` System program account
    /// 5+N                `[]` N lottery participants (sol_acc, fqticket_acc)
    RewardWinners { lottery_id: u32, idx: u64, prize_pool: u64 },
}

impl LotteryInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (amount, _) = rest.split_at(4);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                Self::PurchaseTicket {
                    amount
                }
            },
            1 => {
                let (lottery_id, rest) = rest.split_at(4);
                let lottery_id = lottery_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (idx, rest) = rest.split_at(8);
                let idx = idx
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (prize_pool, _) = rest.split_at(8);
                let prize_pool = prize_pool
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                Self::RewardWinners {
                    lottery_id, idx, prize_pool
                }
            },
            _ => return Err(InvalidInstruction.into())
        })
    }

    /// Packs a LotteryInstruction into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::PurchaseTicket { amount } => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
            },
            Self::RewardWinners { lottery_id, idx, prize_pool } => {
                buf.push(1);
                buf.extend_from_slice(&lottery_id.to_le_bytes());
                buf.extend_from_slice(&idx.to_le_bytes());
                buf.extend_from_slice(&prize_pool.to_le_bytes());
            },
            _ => unreachable!()
        }
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

    fn unpack_ticket_number_arr(input: &[u8]) -> Result<(&[u8; 6], &[u8]), ProgramError> {
        if input.len() < 6 {
            msg!("Cannot be unpacked");
            return Err(InvalidInstruction.into());
        }
        let (bytes, rest) = input.split_at(6);
        Ok((bytes.try_into().map_err(|_| InvalidInstruction)?, rest))
    }
}

/// Creates a `PurchaseTicket` instruction
pub fn purchase_ticket(
    program_id: &Pubkey,
    amount: u32,
    user_fqticket_acc: &Pubkey,
    user_sol_acc: &Pubkey,
    user_slot_acc: &Pubkey,
    fqticket_mint: &Pubkey,
    fqticket_mint_authority: &Pubkey,
    slot_mint: &Pubkey,
    slot_mint_authority: &Pubkey,
    sollotto_sol_acc: &Pubkey
) -> Result<Instruction, ProgramError> {
    let data = LotteryInstruction::PurchaseTicket {
        amount
    }.pack();

    let mut accounts = Vec::with_capacity(10);
    accounts.push(AccountMeta::new(*user_fqticket_acc, false));
    accounts.push(AccountMeta::new(*user_sol_acc, false));
    accounts.push(AccountMeta::new(*user_slot_acc, false));
    accounts.push(AccountMeta::new(*fqticket_mint, false));
    accounts.push(AccountMeta::new(*fqticket_mint_authority, false));
    accounts.push(AccountMeta::new(*slot_mint, false));
    accounts.push(AccountMeta::new(*slot_mint_authority, false));
    accounts.push(AccountMeta::new(*sollotto_sol_acc, false));
    accounts.push(AccountMeta::new_readonly(solana_program::system_program::id(), false));
    accounts.push(AccountMeta::new_readonly(spl_token::id(), false));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data
    })
}

/// Creates a `RewardWinners` instruction
pub fn reward_winners(
    program_id: &Pubkey,
    lottery_id: u32,
    idx: u64,
    prize_pool: u64,
    sollotto_sol: &Pubkey,
    sollotto_rewards: &Pubkey,
    slot_holder_rewards: &Pubkey,
    sollotto_labs: &Pubkey,
    sollotto_result: &Pubkey,
    participants: &Vec<(Pubkey, Pubkey)>
) -> Result<Instruction, ProgramError> {
    let data = LotteryInstruction::RewardWinners {
        lottery_id,
        idx,
        prize_pool
    }.pack();

    let mut accounts = Vec::with_capacity(6 + participants.len());
    accounts.push(AccountMeta::new(*sollotto_sol, false));
    accounts.push(AccountMeta::new(*sollotto_rewards, false));
    accounts.push(AccountMeta::new(*slot_holder_rewards, false));
    accounts.push(AccountMeta::new(*sollotto_labs, false));
    accounts.push(AccountMeta::new(*sollotto_result, false));
    accounts.push(AccountMeta::new_readonly(solana_program::system_program::id(), false));
    for participant in participants {
        accounts.push(AccountMeta::new((participant.0), false));
        accounts.push(AccountMeta::new((participant.1), false));
    }
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data
    })
}
