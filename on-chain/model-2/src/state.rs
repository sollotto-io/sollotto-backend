//! State transition types
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

/// Main information about Sollotto lottery
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LotteryData {
    pub is_initialized: bool,
    pub lottery_id: u32,
    // TODO: fields
    pub prize_pool_amount: u64,
    pub stake_pool_wallet: Pubkey,
    pub rewards_wallet: Pubkey,
    pub slot_holders_rewards_wallet: Pubkey,
    pub sollotto_labs_wallet: Pubkey,
}

impl Sealed for LotteryData {}

impl IsInitialized for LotteryData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LotteryResultData {
    pub lottery_id: u32,
    pub winner: Pubkey,
}

impl Sealed for LotteryResultData {}
