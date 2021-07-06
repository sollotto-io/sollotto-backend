//! Program state processor
use crate::{
    error::LotteryError,
    instruction::LotteryInstruction,
    state::{LotteryData, LotteryResultData},
};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    native_token::{lamports_to_sol, sol_to_lamports},
    program::invoke,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

// Sollotto program_id
solana_program::declare_id!("urNhxed8ocNiFApoooLSAJ1xnWSMUiC9S6fKcRon1rk");

/// Checks that the supplied program ID is the correct
pub fn check_program_account(program_id: &Pubkey) -> ProgramResult {
    if program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}

/// Program state handler.
pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        check_program_account(program_id)?;

        let instruction = LotteryInstruction::unpack(instruction_data)?;
        match instruction {
            LotteryInstruction::InitLottery {
                lottery_id,
                holding_wallet,
                rewards_wallet,
                slot_holders_rewards_wallet,
                sollotto_labs_wallet,
            } => {
                msg!("Instruction: InitLottery");
                Self::process_init_lottery(
                    program_id,
                    accounts,
                    lottery_id,
                    holding_wallet,
                    rewards_wallet,
                    slot_holders_rewards_wallet,
                    sollotto_labs_wallet,
                )
            }

            LotteryInstruction::RewardWinner {} => {
                msg!("Instruction: reward winners");
                Self::process_reward_winner(program_id, accounts)
            }
        }
    }

    pub fn process_init_lottery(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        lottery_id: u32,
        holding_wallet: Pubkey,
        rewards_wallet: Pubkey,
        slot_holders_rewards_wallet: Pubkey,
        sollotto_labs_wallet: Pubkey,
    ) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // TODO

        Ok(())
    }

    pub fn process_reward_winner(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        // TODO

        Ok(())
    }
}

// Unit tests
#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{instruction::Instruction, program_pack::Pack};
    use solana_sdk::account::{
        create_account_for_test, create_is_signer_account_infos, Account as SolanaAccount,
        ReadableAccount,
    };

    // fn lottery_minimum_balance() -> u64 {
    //     Rent::default().minimum_balance(LotteryData::get_packed_len())
    // }

    // fn lottery_result_minimum_balance() -> u64 {
    //     Rent::default().minimum_balance(LotteryResultData::get_packed_len())
    // }

    fn do_process(instruction: Instruction, accounts: Vec<&mut SolanaAccount>) -> ProgramResult {
        let mut meta = instruction
            .accounts
            .iter()
            .zip(accounts)
            .map(|(account_meta, account)| (&account_meta.pubkey, account_meta.is_signer, account))
            .collect::<Vec<_>>();

        let account_infos = create_is_signer_account_infos(&mut meta);
        Processor::process(&instruction.program_id, &account_infos, &instruction.data)
    }

    // TODO: unit tests
}
