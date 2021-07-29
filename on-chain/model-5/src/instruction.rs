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
    /// 3.           `[signer]` FQTicket Mint (must be a system account)
    /// 4.           `[signer]` SLOT Token mint_authority (must be a system account)
    /// 5.         `[writable]` Sollotto SOL account (must be a system account)
    /// 6.                 `[]` System program account
    /// 7.                 `[]` SPL Token account (must be a system account)
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
    /// 4.                 `[]` System program account
    /// 4+N                `[]` N lottery participants
    RewardWinners { idx: u64, prize_pool: u64 },
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
                    idx, prize_pool
                }
            },
            _ => return Err(InvalidInstruction.into())
        })
    }

    /// Packs a LotteryInstruction into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        /* @@@
        match self {
        };
        */
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

/* TODO: Implement those
/// Creates a `PurchaseTicket` instruction
pub fn purchase_ticket(
    program_id: &Pubkey,
    charity: &Pubkey,
    user_wallet_pk: &Pubkey,
    ticket_number_arr: &[u8; 6],
    user_ticket_key: &Pubkey,
    holding_wallet_key: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    // FIXME
    Ok(())
}

/// Creates a `RewardWinners` instruction
pub fn reward_winners(
    program_id: &Pubkey,
    lottery_authority: &Pubkey,
    lottery_result: &Pubkey,
    holding_wallet: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    charities: &[Pubkey; 4],
    participants: &Vec<(Pubkey, Pubkey)>,
) -> Result<Instruction, ProgramError> {
    // FIXME
    Ok(())
}
*/
