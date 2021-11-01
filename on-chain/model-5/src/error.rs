//! Error types
use solana_program::program_error::ProgramError;
use thiserror::Error;

/// Errors that may be returned by the Lottery program.
#[derive(Error, Debug, Copy, Clone)]
pub enum LotteryError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Lottery account is not initialized
    #[error("Lottery account is not initialized")]
    NotInitialized,
    /// Number is invalid
    #[error("Number is invalid")]
    InvalidNumber,
    /// Invalid participants accounts
    #[error("Invalid participants accounts")]
    InvalidParticipantsAccounts,
    /// Invalid sollotto accounts
    #[error("Invalid sollotto account")]
    InvalidSollottoAccount,
    /// Exceeded SLOT Cap for purchasing FQTickets
    #[error("Exceeded SLOT Cap for purchasing FQTickets")]
    SLOTCapExceeded,
    /// Invalid SLOT Account
    #[error("Invalid SLOT Account")]
    InvalidSLOTAccount,
    /// Not enough Fixed-Quality Tokens
    #[error("Not enough Fixed-Quality Tokens")]
    NotEnoughFQTokens,
    /// Empty Prize Pool
    #[error("Empty Prize Pool")]
    EmptyPrizePool,
}

impl From<LotteryError> for ProgramError {
    fn from(e: LotteryError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
