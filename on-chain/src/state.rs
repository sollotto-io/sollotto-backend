use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct TicketData {
    pub charity_id: u32,
    pub user_wallet_pk: [u8; 32],
    pub ticket_number_arr: [u8; 6],
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct LotteryData {
    pub is_lottery_initialised: bool,
    pub lottery_id: u32,
    pub charity_1_id: u32,
    pub charity_2_id: u32,
    pub charity_3_id: u32,
    pub charity_4_id: u32,
    pub charity_1_vc: u32,
    pub charity_2_vc: u32,
    pub charity_3_vc: u32,
    pub charity_4_vc: u32,
    pub total_pool_value: u32,
    pub total_registrations: u32,
    pub ticket_price: u32,
}
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct IncomingLotteryData {
    pub lottery_id: u32,
    pub charity_1_id: u32,
    pub charity_2_id: u32,
    pub charity_3_id: u32,
    pub charity_4_id: u32,
    pub ticket_price: u32,
}
