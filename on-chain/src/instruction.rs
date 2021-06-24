//! Instruction types
use crate::error::LotteryError::InvalidInstruction;
use crate::state::{IncomingLotteryData, TicketData};
use borsh::BorshDeserialize;
use solana_program::program_error::ProgramError;

/// Instructions supported by the Lottery program.
pub enum LotteryInstruction {
    InitLottery {
        is_lottery_initialised: bool,
        lottery_id: u32,
        charity_1_id: u32,
        charity_2_id: u32,
        charity_3_id: u32,
        charity_4_id: u32,
        charity_1_vc: u32,
        charity_2_vc: u32,
        charity_3_vc: u32,
        charity_4_vc: u32,
        total_registrations: u32,
    },
    PurchaseTicket {
        charity_id: u32,
        user_wallet_pk: [u8; 32],
        ticket_number_arr: [u8; 6],
    },
}

impl LotteryInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => Self::InitLottery {
                is_lottery_initialised: true,
                lottery_id: IncomingLotteryData::try_from_slice(&rest)
                    .unwrap()
                    .lottery_id,
                charity_1_id: IncomingLotteryData::try_from_slice(&rest)
                    .unwrap()
                    .charity_1_id,
                charity_2_id: IncomingLotteryData::try_from_slice(&rest)
                    .unwrap()
                    .charity_2_id,
                charity_3_id: IncomingLotteryData::try_from_slice(&rest)
                    .unwrap()
                    .charity_3_id,
                charity_4_id: IncomingLotteryData::try_from_slice(&rest)
                    .unwrap()
                    .charity_4_id,
                charity_1_vc: 0,
                charity_2_vc: 0,
                charity_3_vc: 0,
                charity_4_vc: 0,
                total_registrations: 0,
            },
            1 => Self::PurchaseTicket {
                charity_id: TicketData::try_from_slice(&rest).unwrap().charity_id,
                user_wallet_pk: TicketData::try_from_slice(&rest).unwrap().user_wallet_pk,
                ticket_number_arr: TicketData::try_from_slice(&rest).unwrap().ticket_number_arr,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
