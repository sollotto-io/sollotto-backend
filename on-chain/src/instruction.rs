use crate::error::LotteryError::InvalidInstruction;
use solana_program::program_error::ProgramError;

pub enum LotteryInstruction {
    InitLottery {
        lottery_id: u32,
        charity_ids: vec![u32; 4],
        winner_user_wallet_pk: Option<[u8; 32]>,
        total_pool_value: f64,
        total_registrations: u32,
        ticket_price: f64,
    },
    PurchaseTicket {
        charity_id: u32,
        user_wallet_pk: [u8; 32],
        ticket_number_arr: [u8; 6],
    },
}

impl LotteryInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(
        instruction_id: u8,
        lottery_id: u32,
        charity_ids: &[u32; 4],
        ticket_price: f64,
        charity_id: u32,
        user_wallet_pk: [u8; 32],
        ticket_number_arr: [u8; 6],
    ) -> Result<Self, ProgramError> {
        Ok(match instruction_id {
            0 => Self::InitLottery {
                lottery_id: lottery_id,
                charity_ids: charity_ids,
                ticket_price: ticket_price,
                winner_user_wallet_pk: None,
                total_pool_value: 0.0,
                total_registrations: 0,
            },
            1 => Self::PurchaseTicket {
                charity_id: charity_id,
                user_wallet_pk: user_wallet_pk,
                ticket_number_arr: ticket_number_arr,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }
}
