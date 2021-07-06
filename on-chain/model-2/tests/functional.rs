use solana_program::{
    hash::Hash,
    instruction::InstructionError,
    native_token::{lamports_to_sol, sol_to_lamports},
    program_pack::Pack,
    system_instruction,
};
use solana_program_test::*;
use solana_sdk::{
    signature::Keypair, system_transaction, transaction::TransactionError,
    transport::TransportError,
};
use sollotto_model_2::{
    processor::id,
    processor::Processor,
    state::{LotteryData, LotteryResultData},
};
use {
    solana_program::pubkey::Pubkey,
    solana_sdk::{signature::Signer, transaction::Transaction},
};

// TODO: functional tests
