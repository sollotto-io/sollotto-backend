//! Instruction types
use crate::{check_program_account, error::LotteryError::InvalidInstruction};
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
    InitLottery {
        lottery_id: u32,
        charity_1_id: u32,
        charity_2_id: u32,
        charity_3_id: u32,
        charity_4_id: u32,
    },

    PurchaseTicket {
        charity_id: u32,
        user_wallet_pk: Pubkey,
        ticket_number_arr: [u8; 6],
    },
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

                let (charity_1_id, rest) = rest.split_at(4);
                let charity_1_id = charity_1_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (charity_2_id, rest) = rest.split_at(4);
                let charity_2_id = charity_2_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (charity_3_id, rest) = rest.split_at(4);
                let charity_3_id = charity_3_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (charity_4_id, _) = rest.split_at(4);
                let charity_4_id = charity_4_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                Self::InitLottery {
                    lottery_id,
                    charity_1_id,
                    charity_2_id,
                    charity_3_id,
                    charity_4_id,
                }
            }
            1 => {
                let (charity_id, rest) = rest.split_at(4);
                let charity_id = charity_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (user_wallet_pk, rest) = Self::unpack_pubkey(rest).unwrap();

                let (ticket_number_arr, _) = Self::unpack_ticket_number_arr(rest).unwrap();

                Self::PurchaseTicket {
                    charity_id,
                    user_wallet_pk,
                    ticket_number_arr: *ticket_number_arr,
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
                lottery_id,
                charity_1_id,
                charity_2_id,
                charity_3_id,
                charity_4_id,
            } => {
                buf.push(0);
                buf.extend_from_slice(&lottery_id.to_le_bytes());
                buf.extend_from_slice(&charity_1_id.to_le_bytes());
                buf.extend_from_slice(&charity_2_id.to_le_bytes());
                buf.extend_from_slice(&charity_3_id.to_le_bytes());
                buf.extend_from_slice(&charity_4_id.to_le_bytes());
            }

            Self::PurchaseTicket {
                charity_id,
                user_wallet_pk,
                ticket_number_arr,
            } => {
                buf.push(1);
                buf.extend_from_slice(&charity_id.to_le_bytes());
                buf.extend_from_slice(user_wallet_pk.as_ref());
                buf.extend_from_slice(&ticket_number_arr.as_ref());
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
    charity_1_id: u32,
    charity_2_id: u32,
    charity_3_id: u32,
    charity_4_id: u32,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::InitLottery {
        lottery_id,
        charity_1_id,
        charity_2_id,
        charity_3_id,
        charity_4_id,
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
