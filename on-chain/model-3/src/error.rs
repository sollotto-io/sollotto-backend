//! Error types
use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Errors that may be returned by the Lottery program.
#[derive(Error, Debug, Copy, Clone)]
pub enum LotteryError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Prize pool is empty
    #[error("Priez pool is empty")]
    EmptyPrizePool,
}

impl From<LotteryError> for ProgramError {
    fn from(e: LotteryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
