//! Instruction types
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
    /// User purchases new ticket.
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable]` User FQTicket account
    /// 1. `[writable, signer]` User SOL account (must be a system account)
    /// 2. `[]` User SLOT account
    /// 3. `[writable]` FQTicket Mint
    /// 4. `[signer]` FQTicket mint_authority
    /// 5. `[]` SLOT Mint
    /// 6. `[signer]` SLOT mint_authority
    /// 7. `[writable]` Sollotto SOL account (must be a system account)
    /// 8. `[]` System program account
    /// 9. `[]` SPL Token account
    PurchaseTicket { amount: u64 },

    /// Rewarding the winners determined by indexing accounts with `idx`.
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Sollotto SOL Prize pool account (must be system account)
    /// 1. `[writable]` Sollotto Rewards account (must be a system account)
    /// 2. `[writable]` SLOT Holder Rewards account (must be a system account)
    /// 3. `[writable]` Sollotto labs account (must be a system account)
    /// 4. `[writable]` Sollotto Result account
    /// 5. `[]` System program account
    /// 5+N `[]` N lottery participants (sol_acc, fqticket_acc)
    RewardWinners {
        lottery_id: u32,
        random_number: u32,
    },
}

impl LotteryInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (amount, _) = rest.split_at(8);
                let amount = amount
                    .try_into()
                    .ok()
                    .map(u64::from_le_bytes)
                    .ok_or(InvalidInstruction)?;
                Self::PurchaseTicket { amount }
            }
            1 => {
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

                Self::RewardWinners {
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
            Self::PurchaseTicket { amount } => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
            }
            Self::RewardWinners {
                lottery_id,
                random_number,
            } => {
                buf.push(1);
                buf.extend_from_slice(&lottery_id.to_le_bytes());
                buf.extend_from_slice(&random_number.to_le_bytes());
            }
        }
        buf
    }
}

/// Creates a `PurchaseTicket` instruction
pub fn purchase_ticket(
    program_id: &Pubkey,
    amount: u64,
    user_fqticket_acc: &Pubkey,
    user_sol_acc: &Pubkey,
    user_slot_acc: &Pubkey,
    fqticket_mint: &Pubkey,
    fqticket_mint_authority: &Pubkey,
    slot_mint: &Pubkey,
    slot_mint_authority: &Pubkey,
    sollotto_sol_acc: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = LotteryInstruction::PurchaseTicket { amount }.pack();

    let mut accounts = Vec::with_capacity(10);
    accounts.push(AccountMeta::new(*user_fqticket_acc, false));
    accounts.push(AccountMeta::new(*user_sol_acc, true));
    accounts.push(AccountMeta::new(*user_slot_acc, false));
    accounts.push(AccountMeta::new(*fqticket_mint, false));
    accounts.push(AccountMeta::new(*fqticket_mint_authority, true));
    accounts.push(AccountMeta::new(*slot_mint, false));
    accounts.push(AccountMeta::new(*slot_mint_authority, true));
    accounts.push(AccountMeta::new(*sollotto_sol_acc, false));
    accounts.push(AccountMeta::new_readonly(
        solana_program::system_program::id(),
        false,
    ));
    accounts.push(AccountMeta::new_readonly(spl_token::id(), false));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a `RewardWinners` instruction
pub fn reward_winners(
    program_id: &Pubkey,
    lottery_id: u32,
    random_number: u32,
    prize_pool_sol: &Pubkey,
    sollotto_rewards: &Pubkey,
    slot_holder_rewards: &Pubkey,
    sollotto_labs: &Pubkey,
    sollotto_result: &Pubkey,
    participants: &Vec<(Pubkey, Pubkey)>,
) -> Result<Instruction, ProgramError> {
    let data = LotteryInstruction::RewardWinners {
        lottery_id,
        random_number,
    }
    .pack();

    let mut accounts = Vec::with_capacity(6 + participants.len());
    accounts.push(AccountMeta::new(*prize_pool_sol, true));
    accounts.push(AccountMeta::new(*sollotto_rewards, false));
    accounts.push(AccountMeta::new(*slot_holder_rewards, false));
    accounts.push(AccountMeta::new(*sollotto_labs, false));
    accounts.push(AccountMeta::new(*sollotto_result, true));
    accounts.push(AccountMeta::new_readonly(
        solana_program::system_program::id(),
        false,
    ));
    for participant in participants {
        accounts.push(AccountMeta::new(participant.0, false));
        accounts.push(AccountMeta::new(participant.1, false));
    }
    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}
