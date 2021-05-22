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
    pub charity_ids: vec![u32; 4],
    pub charity_vote_counts: vec![u32; 4],
    pub winner_user_wallet_pk: [u8; 32],
    pub total_pool_value: f64,
    pub total_registrations: u32,
    pub ticket_price: f64,
}
