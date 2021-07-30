//! State transition types
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

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

