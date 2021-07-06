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
    // TODO
    /// Initialize new lottery data
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    /// 1. `[]` Rent sysvar
    InitLottery {
        lottery_id: u32,
        holding_wallet: Pubkey,
        rewards_wallet: Pubkey,
        slot_holders_rewards_wallet: Pubkey,
        sollotto_labs_wallet: Pubkey,
    },

    // TODO
    RewardWinner {},
}

impl LotteryInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (lottery_id, rest) = rest.split_at(4);
                let lottery_id = lottery_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (holding_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (slot_holders_rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (sollotto_labs_wallet, _) = Self::unpack_pubkey(rest).unwrap();

                Self::InitLottery {
                    lottery_id,
                    holding_wallet,
                    rewards_wallet,
                    slot_holders_rewards_wallet,
                    sollotto_labs_wallet,
                }
            }

            1 => Self::RewardWinner {},

            _ => return Err(InvalidInstruction.into()),
        })
    }

    /// Packs a LotteryInstruction into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::InitLottery {
                lottery_id,
                holding_wallet,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
            } => {
                buf.push(0);
                buf.extend_from_slice(&lottery_id.to_le_bytes());
                buf.extend_from_slice(holding_wallet.as_ref());
                buf.extend_from_slice(rewards_wallet.as_ref());
                buf.extend_from_slice(slot_holders_rewards_wallet.as_ref());
                buf.extend_from_slice(sollotto_labs_wallet.as_ref());
            }

            Self::RewardWinner {} => {
                buf.push(1);
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

    fn unpack_ticket_number_arr(input: &[u8]) -> Result<(&[u8; 6], &[u8]), ProgramError> {
        if input.len() < 6 {
            msg!("Cannot be unpacked");
            return Err(InvalidInstruction.into());
        }
        let (bytes, rest) = input.split_at(6);
        Ok((bytes.try_into().map_err(|_| InvalidInstruction)?, rest))
    }
}

/// Creates a `InitLottery` instruction
pub fn initialize_lottery(
    program_id: &Pubkey,
    lottery_id: u32,
    holding_wallet: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_rewards_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::InitLottery {
        lottery_id: lottery_id,
        holding_wallet: *holding_wallet,
        rewards_wallet: *rewards_wallet,
        slot_holders_rewards_wallet: *slot_holders_rewards_wallet,
        sollotto_labs_wallet: *sollotto_labs_wallet,
    }
    .pack();

    let mut accounts = Vec::with_capacity(2);
    accounts.push(AccountMeta::new(*lottery_authority, true));
    accounts.push(AccountMeta::new_readonly(sysvar::rent::id(), false));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

