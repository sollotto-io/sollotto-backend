//! State transition types
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct LotteryData {
    pub is_initialized: bool,
    pub lottery_id: u32,
    pub charity_1_id: u32,
    pub charity_2_id: u32,
    pub charity_3_id: u32,
    pub charity_4_id: u32,
    pub charity_1_vc: u32,
    pub charity_2_vc: u32,
    pub charity_3_vc: u32,
    pub charity_4_vc: u32,
    pub total_registrations: u32,
    pub winning_numbers: [u8; 6],
}

impl Sealed for LotteryData {}

impl IsInitialized for LotteryData {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for LotteryData {
    /// 1 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 4 + 6 = 47
    const LEN: usize = 47;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 47];
        let (
            is_initialized,
            lottery_id,
            charity_1_id,
            charity_2_id,
            charity_3_id,
            charity_4_id,
            charity_1_vc,
            charity_2_vc,
            charity_3_vc,
            charity_4_vc,
            total_registrations,
            winning_numbers,
        ) = array_refs![src, 1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 6];

        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        let result = LotteryData {
            is_initialized,
            lottery_id: u32::from_le_bytes(*lottery_id),
            charity_1_id: u32::from_le_bytes(*charity_1_id),
            charity_2_id: u32::from_le_bytes(*charity_2_id),
            charity_3_id: u32::from_le_bytes(*charity_3_id),
            charity_4_id: u32::from_le_bytes(*charity_4_id),
            charity_1_vc: u32::from_le_bytes(*charity_1_vc),
            charity_2_vc: u32::from_le_bytes(*charity_2_vc),
            charity_3_vc: u32::from_le_bytes(*charity_3_vc),
            charity_4_vc: u32::from_le_bytes(*charity_4_vc),
            total_registrations: u32::from_le_bytes(*total_registrations),
            winning_numbers: *winning_numbers,
        };

        Ok(result)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 47];
        let (
            is_initialized_dst,
            lottery_id_dst,
            charity_1_id_dst,
            charity_2_id_dst,
            charity_3_id_dst,
            charity_4_id_dst,
            charity_1_vc_dst,
            charity_2_vc_dst,
            charity_3_vc_dst,
            charity_4_vc_dst,
            total_registrations_dst,
            winning_numbers_dst,
        ) = mut_array_refs![dst, 1, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 6];

        is_initialized_dst[0] = self.is_initialized as u8;
        *lottery_id_dst = self.lottery_id.to_le_bytes();
        *charity_1_id_dst = self.charity_1_id.to_le_bytes();
        *charity_2_id_dst = self.charity_2_id.to_le_bytes();
        *charity_3_id_dst = self.charity_3_id.to_le_bytes();
        *charity_4_id_dst = self.charity_4_id.to_le_bytes();
        *charity_1_vc_dst = self.charity_1_vc.to_le_bytes();
        *charity_2_vc_dst = self.charity_2_vc.to_le_bytes();
        *charity_3_vc_dst = self.charity_3_vc.to_le_bytes();
        *charity_4_vc_dst = self.charity_4_vc.to_le_bytes();
        *total_registrations_dst = self.total_registrations.to_le_bytes();
        *winning_numbers_dst = self.winning_numbers;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct TicketData {
    pub charity_id: u32,
    pub user_wallet_pk: Pubkey,
    pub ticket_number_arr: [u8; 6],
}

impl Sealed for TicketData {}

impl Pack for TicketData {
    /// 4 + 32 + 1 * 6 = 42
    const LEN: usize = 42;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 42];
        let (charity_id, user_wallet_pk, ticket_number_arr) = array_refs![src, 4, 32, 6];

        let result = TicketData {
            charity_id: u32::from_le_bytes(*charity_id),
            user_wallet_pk: Pubkey::new_from_array(*user_wallet_pk),
            ticket_number_arr: *ticket_number_arr,
        };

        Ok(result)
    }

    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 42];
        let (charity_id_dst, user_wallet_pk_dst, ticket_number_arr_dst) =
            mut_array_refs![dst, 4, 32, 6];

        *charity_id_dst = self.charity_id.to_le_bytes();
        user_wallet_pk_dst.copy_from_slice(self.user_wallet_pk.as_ref());
        ticket_number_arr_dst.copy_from_slice(self.ticket_number_arr.as_ref());
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
