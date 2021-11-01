//! Error types
use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Errors that may be returned by the Lottery program.
#[derive(Error, Debug, Copy, Clone)]
pub enum LotteryError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Lottery account is not initialized
    #[error("Lottery account is not initialized")]
    NotInitialized,
    /// Lottery account is initialized
    #[error("Lottery account is initialized")]
    Initialized,
    /// Invalid sollotto accounts
    #[error("Invalid sollotto account")]
    InvalidSollottoAccount,
    /// Prize pool is empty
    #[error("Priez pool is empty")]
    EmptyPrizePool,
}

impl From<LotteryError> for ProgramError {
    fn from(e: LotteryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
