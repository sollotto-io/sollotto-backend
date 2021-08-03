//! Lottery program

use solana_program::{entrypoint::ProgramResult, program_error::ProgramError, pubkey::Pubkey};
mod entrypoint;
pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_program::declare_id!("urNhxed8ocNiFApoooLSAJ1xnWSMUiC9S6fKcRon1rk");

/// Checks that the supplied program ID is the correct
pub fn check_program_account(program_id: &Pubkey) -> ProgramResult {
    if program_id != &id() {
        return Err(ProgramError::IncorrectProgramId);
    }
    Ok(())
}
