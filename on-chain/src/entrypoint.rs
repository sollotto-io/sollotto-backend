use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, pubkey::Pubkey,
};

use crate::processor::Processor;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_id: u8,
    lottery_id: u32,
    charity_ids: &[u32; 4],
    winner_user_wallet_pk: [u8; 32],
    total_pool_value: f64,
    total_registrations: u32,
    ticket_price: f64,
    charity_id: u32,
    user_wallet_pk: [u8; 32],
    ticket_number_arr: [u8; 6],
) -> ProgramResult {
    Processor::process(
        program_id,
        accounts,
        instruction_id,
        lottery_id,
        charity_ids,
        ticket_price,
        charity_id,
        user_wallet_pk,
        ticket_number_arr,
    )
}
