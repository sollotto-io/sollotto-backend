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
    pub staking_pool_amount: u64,
    pub staking_pool_wallet: Pubkey,
    pub staking_pool_token_mint: Pubkey,
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

impl Pack for LotteryData {
    /// 1 + 8 + 32 + 32 + 32 + 32 + 32 = 169
    const LEN: usize = 165;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 169];
        let (
            is_initialized,
            staking_pool_amount,
            staking_pool_wallet,
            staking_pool_token_mint,
            rewards_wallet,
            slot_holders_rewards_wallet,
            sollotto_labs_wallet,
        ) = array_refs![src, 1, 8, 32, 32, 32, 32, 32];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let result = LotteryData {
            is_initialized,
            staking_pool_amount: u64::from_le_bytes(*staking_pool_amount),
            staking_pool_wallet: Pubkey::new_from_array(*staking_pool_wallet),
            staking_pool_token_mint: Pubkey::new_from_array(*staking_pool_token_mint),
            rewards_wallet: Pubkey::new_from_array(*rewards_wallet),
            slot_holders_rewards_wallet: Pubkey::new_from_array(*slot_holders_rewards_wallet),
            sollotto_labs_wallet: Pubkey::new_from_array(*sollotto_labs_wallet),
        };

        Ok(result)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 169];
        let (
            is_initialized_dst,
            staking_pool_amount_dst,
            staking_pool_wallet_dst,
            staking_pool_token_mint_dst,
            rewards_wallet_dst,
            slot_holders_rewards_wallet_dst,
            sollotto_labs_wallet_dst,
        ) = mut_array_refs![dst, 1, 8, 32, 32, 32, 32, 32];

        is_initialized_dst[0] = self.is_initialized as u8;
        *staking_pool_amount_dst = self.staking_pool_amount.to_le_bytes();
        staking_pool_wallet_dst.copy_from_slice(self.staking_pool_wallet.as_ref());
        staking_pool_token_mint_dst.copy_from_slice(self.staking_pool_token_mint.as_ref());
        rewards_wallet_dst.copy_from_slice(self.rewards_wallet.as_ref());
        slot_holders_rewards_wallet_dst.copy_from_slice(self.slot_holders_rewards_wallet.as_ref());
        sollotto_labs_wallet_dst.copy_from_slice(self.sollotto_labs_wallet.as_ref());
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LotteryResultData {
    pub lottery_id: u32,
    pub winner: Pubkey,
}

impl Sealed for LotteryResultData {}

impl Pack for LotteryResultData {
    /// 4 + 32 = 36
    const LEN: usize = 36;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 36];
        let (lottery_id, winner) = array_refs![src, 4, 32];

        let result = LotteryResultData {
            lottery_id: u32::from_le_bytes(*lottery_id),
            winner: Pubkey::new_from_array(*winner),
        };

        Ok(result)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 36];
        let (lottery_id_dst, winner_dst) = mut_array_refs![dst, 4, 32];

        *lottery_id_dst = self.lottery_id.to_le_bytes();
        winner_dst.copy_from_slice(self.winner.as_ref());
    }
}
