use crate::error::LotteryError::InvalidInstruction;
use crate::state::{LotteryData, TicketData};
use borsh::BorshDeserialize;
use solana_program::{msg, program_error::ProgramError};

pub enum LotteryInstruction {
    InitLottery {
        is_lottery_initialised: bool,
        lottery_id: u32,
        charity_ids: [u32; 4],
        charity_vote_counts: [u32; 4],
        winner_user_wallet_pk: [u8; 32],
        total_pool_value: u32,
        total_registrations: u32,
        ticket_price: u32,
    },
    PurchaseTicket {
        charity_id: u32,
        user_wallet_pk: [u8; 32],
        ticket_number_arr: [u8; 6],
    },
}

impl LotteryInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        msg!("Tag: {}", tag);
        msg!("Rest: {:?}", rest);
        msg!("Rest: {:?}", LotteryData::try_from_slice(&rest));
        msg!("Rest: {:?}", &rest);
        Ok(match tag {
            0 => Self::InitLottery {
                is_lottery_initialised: true,
                lottery_id: LotteryData::try_from_slice(&rest).unwrap().lottery_id,
                charity_ids: LotteryData::try_from_slice(&rest).unwrap().charity_ids,
                charity_vote_counts: [0, 0, 0, 0],
                winner_user_wallet_pk: LotteryData::try_from_slice(&rest)
                    .unwrap()
                    .winner_user_wallet_pk,
                total_pool_value: 0,
                total_registrations: 0,
                ticket_price: LotteryData::try_from_slice(&rest).unwrap().ticket_price,
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
