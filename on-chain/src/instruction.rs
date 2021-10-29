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
    /// Initialize new lottery data
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    /// 1. `[]` Rent sysvar
    InitLottery {
        lottery_id: u32,
        charity_1: Pubkey,
        charity_2: Pubkey,
        charity_3: Pubkey,
        charity_4: Pubkey,
        holding_wallet: Pubkey,
        rewards_wallet: Pubkey,
        slot_holders_rewards_wallet: Pubkey,
        sollotto_labs_wallet: Pubkey,
        randomness_account: Pubkey,
    },

    /// User purchases new ticket for lottery
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    /// 1. `[writable]` Users ticket data account
    /// 2. `[writable,signer]` User funding account (must be a system account)
    /// 3. `[writable]` Sollotto holding wallet account (must be a system account)
    /// 4. `[]` Rent sysvar
    /// 5. `[]` System program account
    PurchaseTicket {
        charity: Pubkey,
        user_wallet_pk: Pubkey,
        ticket_number_arr: [u8; 6],
    },

    /// Store the winning combination into lottery data account
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    /// 1. `[]` Vrf account
    StoreWinningNumbers {},

    /// Check users number combinations and find the lottery winner.
    /// Information obout winner sotored in LotteryResultData account,
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    /// 1. `[writable]` Lottery result data account
    /// 2. `[writable, signer]` Sollotto holding wallet account (must be a system account)
    /// 3. `[writable]` Solloto rewards wallet account (must be a system account)
    /// 4. `[writable]` SLOT holders wallet account (must be a system account)
    /// 5. `[writable]` Solloto labs wallet account (must be a system account)
    /// 6-9. `[writable]` Charities wallet accounts (must be a system account)
    /// 10. `[]` System program account
    /// 10 + N*2. `[]` N*2 readonly percipients accounts pairs: (ticket_acc, user_wallet_acc (system account))
    RewardWinners {},

    /// Update charity wallets in lottery data account
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    UpdateCharity {
        charity_1: Pubkey,
        charity_2: Pubkey,
        charity_3: Pubkey,
        charity_4: Pubkey,
    },

    /// Update sollotto wallets in lottery data account
    /// Accounts expected by this instruction:
    ///
    /// 0. `[writable, signer]` Lottery data account
    UpdateSollottoWallets {
        holding_wallet: Pubkey,
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
                let (lottery_id, rest) = rest.split_at(4);
                let lottery_id = lottery_id
                    .try_into()
                    .ok()
                    .map(u32::from_le_bytes)
                    .ok_or(InvalidInstruction)?;

                let (charity_1, rest) = Self::unpack_pubkey(rest).unwrap();
                let (charity_2, rest) = Self::unpack_pubkey(rest).unwrap();
                let (charity_3, rest) = Self::unpack_pubkey(rest).unwrap();
                let (charity_4, rest) = Self::unpack_pubkey(rest).unwrap();
                let (holding_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (slot_holders_rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (sollotto_labs_wallet, res) = Self::unpack_pubkey(rest).unwrap();
                let (randomness_account, _) = Self::unpack_pubkey(rest).unwrap();

                Self::InitLottery {
                    lottery_id,
                    charity_1,
                    charity_2,
                    charity_3,
                    charity_4,
                    holding_wallet,
                    rewards_wallet,
                    slot_holders_rewards_wallet,
                    sollotto_labs_wallet,
                    randomness_account,
                }
            }

            1 => {
                let (charity, rest) = Self::unpack_pubkey(rest).unwrap();
                let (user_wallet_pk, rest) = Self::unpack_pubkey(rest).unwrap();
                let (ticket_number_arr, _) = Self::unpack_ticket_number_arr(rest).unwrap();

                Self::PurchaseTicket {
                    charity,
                    user_wallet_pk,
                    ticket_number_arr: *ticket_number_arr,
                }
            }

            2 => Self::StoreWinningNumbers {},

            3 => Self::RewardWinners {},

            4 => {
                let (charity_1, rest) = Self::unpack_pubkey(rest).unwrap();
                let (charity_2, rest) = Self::unpack_pubkey(rest).unwrap();
                let (charity_3, rest) = Self::unpack_pubkey(rest).unwrap();
                let (charity_4, _) = Self::unpack_pubkey(rest).unwrap();

                Self::UpdateCharity {
                    charity_1,
                    charity_2,
                    charity_3,
                    charity_4,
                }
            }

            5 => {
                let (holding_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (slot_holders_rewards_wallet, rest) = Self::unpack_pubkey(rest).unwrap();
                let (sollotto_labs_wallet, _) = Self::unpack_pubkey(rest).unwrap();

                Self::UpdateSollottoWallets {
                    holding_wallet,
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
                lottery_id,
                charity_1,
                charity_2,
                charity_3,
                charity_4,
                holding_wallet,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
                randomness_account,
            } => {
                buf.push(0);
                buf.extend_from_slice(&lottery_id.to_le_bytes());
                buf.extend_from_slice(charity_1.as_ref());
                buf.extend_from_slice(charity_2.as_ref());
                buf.extend_from_slice(charity_3.as_ref());
                buf.extend_from_slice(charity_4.as_ref());
                buf.extend_from_slice(holding_wallet.as_ref());
                buf.extend_from_slice(rewards_wallet.as_ref());
                buf.extend_from_slice(slot_holders_rewards_wallet.as_ref());
                buf.extend_from_slice(sollotto_labs_wallet.as_ref());
                buf.extend_from_slice(randomness_account.as_ref());
            }

            Self::PurchaseTicket {
                charity,
                user_wallet_pk,
                ticket_number_arr,
            } => {
                buf.push(1);
                buf.extend_from_slice(charity.as_ref());
                buf.extend_from_slice(user_wallet_pk.as_ref());
                buf.extend_from_slice(&ticket_number_arr.as_ref());
            }

            Self::StoreWinningNumbers {} => {
                buf.push(2);
            }

            Self::RewardWinners {} => {
                buf.push(3);
            }

            Self::UpdateCharity {
                charity_1,
                charity_2,
                charity_3,
                charity_4,
            } => {
                buf.push(4);
                buf.extend_from_slice(charity_1.as_ref());
                buf.extend_from_slice(charity_2.as_ref());
                buf.extend_from_slice(charity_3.as_ref());
                buf.extend_from_slice(charity_4.as_ref());
            }

            Self::UpdateSollottoWallets {
                holding_wallet,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
            } => {
                buf.push(5);
                buf.extend_from_slice(holding_wallet.as_ref());
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
    charity_1: &Pubkey,
    charity_2: &Pubkey,
    charity_3: &Pubkey,
    charity_4: &Pubkey,
    holding_wallet: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_rewards_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    randomness_account: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::InitLottery {
        lottery_id: lottery_id,
        charity_1: *charity_1,
        charity_2: *charity_2,
        charity_3: *charity_3,
        charity_4: *charity_4,
        holding_wallet: *holding_wallet,
        rewards_wallet: *rewards_wallet,
        slot_holders_rewards_wallet: *slot_holders_rewards_wallet,
        sollotto_labs_wallet: *sollotto_labs_wallet,
        randomness_account: *randomness_account,
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
    check_program_account(program_id)?;
    let data = LotteryInstruction::PurchaseTicket {
        charity: *charity,
        user_wallet_pk: *user_wallet_pk,
        ticket_number_arr: *ticket_number_arr,
    }
    .pack();

    let mut accounts = Vec::with_capacity(6);
    accounts.push(AccountMeta::new(*lottery_authority, true));
    accounts.push(AccountMeta::new(*user_ticket_key, false));
    accounts.push(AccountMeta::new(*user_wallet_pk, true));
    accounts.push(AccountMeta::new(*holding_wallet_key, false));
    accounts.push(AccountMeta::new_readonly(sysvar::rent::id(), false));
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

/// Creates a `StoreWinningNumbers` instruction
pub fn store_winning_numbers(
    program_id: &Pubkey,
    winning_numbers_arr: &[u8; 6],
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::StoreWinningNumbers {}.pack();

    let mut accounts = Vec::with_capacity(1);
    accounts.push(AccountMeta::new(*lottery_authority, true));

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
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
    check_program_account(program_id)?;
    let data = LotteryInstruction::RewardWinners {}.pack();

    let mut accounts = Vec::with_capacity(9 + participants.len());
    accounts.push(AccountMeta::new(*lottery_authority, true));
    accounts.push(AccountMeta::new(*lottery_result, false));
    accounts.push(AccountMeta::new(*holding_wallet, true));
    accounts.push(AccountMeta::new(*rewards_wallet, false));
    accounts.push(AccountMeta::new(*slot_holders_wallet, false));
    accounts.push(AccountMeta::new(*sollotto_labs_wallet, false));
    for charity in charities {
        accounts.push(AccountMeta::new(*charity, false));
    }
    accounts.push(AccountMeta::new_readonly(
        solana_program::system_program::id(),
        false,
    ));
    for participant in participants {
        accounts.push(AccountMeta::new_readonly(participant.0, false));
        accounts.push(AccountMeta::new(participant.1, false));
    }

    Ok(Instruction {
        program_id: *program_id,
        accounts,
        data,
    })
}

/// Creates a `UpdateCharity` instruction
pub fn update_charity(
    program_id: &Pubkey,
    charity_1: &Pubkey,
    charity_2: &Pubkey,
    charity_3: &Pubkey,
    charity_4: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::UpdateCharity {
        charity_1: *charity_1,
        charity_2: *charity_2,
        charity_3: *charity_3,
        charity_4: *charity_4,
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

/// Creates a `UpdateSollottoWallets` instruction
pub fn update_sollotto_wallets(
    program_id: &Pubkey,
    holding_wallet: &Pubkey,
    rewards_wallet: &Pubkey,
    slot_holders_rewards_wallet: &Pubkey,
    sollotto_labs_wallet: &Pubkey,
    lottery_authority: &Pubkey,
) -> Result<Instruction, ProgramError> {
    check_program_account(program_id)?;
    let data = LotteryInstruction::UpdateSollottoWallets {
        holding_wallet: *holding_wallet,
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
