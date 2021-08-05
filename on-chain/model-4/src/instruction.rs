//! Instruction types
use crate::check_program_account;
use crate::error::LotteryError::InvalidInstruction;
use solana_program::{instruction::{AccountMeta, Instruction}, program_error::ProgramError, pubkey::Pubkey, system_program, sysvar};
use std::{convert::TryInto, mem::size_of};

/// Instructions supported by the Lottery program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum LotteryInstruction {
    /// Get the random number, find winner and pay reward from prize pool.
    /// 30% of the prize pool pays to the charity.
    /// Lottery id, winner's wallet are recorded into chain.
    ///
    /// Accounts expected by this instruction:
    /// 0. `[writable, signer]` Prize Pool SOL wallet (must be a system account)
    /// 1. `[writable]` Solloto rewards wallet account (must be a system account)
    /// 2. `[writable]` SLOT holders wallet account (must be a system account)
    /// 3. `[writable]` Solloto labs wallet account (must be a system account)
    /// 4. `[writable]` Lottery Result Data account
    /// 5. `[signer]` Lifetime Ticket Token owner
    /// 6. `[]` Lifetime Ticket Token Mint
    /// 7. `[]` System program account
    /// 8. `[]` Rent sysvar
    /// The accounts pairs for every lottery participant:
    /// 0. `[writable]` User wallet (for getting reward)
    /// 1. `[]` User Lifetime Ticket Token Account (for check validness)
    RewardWinner {
        /// Inner identifier for lottery (will be recorded on-chain)
        lottery_id: u32,
        /// Random winner number from 0 to participant count
        random_number: u32,
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
            Self::RewardWinner {
                lottery_id,
                random_number,
            } => {
                buf.push(0);
                buf.extend_from_slice(&lottery_id.to_le_bytes());
                buf.extend_from_slice(&random_number.to_le_bytes());
            }
        };
        buf
    }
}

/// Creates a `RewardWinner` instruction
pub fn reward_winner(
    program_id: &Pubkey,
    lottery_id: u32,
    random_number: u32,
    prize_pool: &Pubkey,
    sollotto_rewards_wallet: &Pubkey,
    slot_holders_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    lottery_result: &Pubkey,
    lifetime_ticket_token_owner: &Pubkey,
    lifetime_ticket_token_mint: &Pubkey,
    participants: &Vec<(Pubkey, Pubkey)>,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::RewardWinner {
        lottery_id,
        random_number,
    }
    .pack();

    let mut accounts = Vec::with_capacity(9 + participants.len());
    accounts.push(AccountMeta::new(*prize_pool, true));
    accounts.push(AccountMeta::new(*sollotto_rewards_wallet, false));
    accounts.push(AccountMeta::new(*slot_holders_wallet, false));
    accounts.push(AccountMeta::new(*sollotto_labs_wallet, false));
    accounts.push(AccountMeta::new(*lottery_result, false));
    accounts.push(AccountMeta::new_readonly(*lifetime_ticket_token_owner, true));
    accounts.push(AccountMeta::new_readonly(*lifetime_ticket_token_mint, false));
    accounts.push(AccountMeta::new_readonly(system_program::id(), false));
    accounts.push(AccountMeta::new_readonly(sysvar::rent::id(), false));
    for participant in participants {
        accounts.push(AccountMeta::new(participant.0, false));
        accounts.push(AccountMeta::new_readonly(participant.1, false));
    }

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
