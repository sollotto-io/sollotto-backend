use thiserror::Error;

use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum LotteryError {
    /// Invalid charity id
    #[error("Invalid Charity")]
    InvalidCharity,
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
}

impl From<LotteryError> for ProgramError {
    fn from(e: LotteryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
