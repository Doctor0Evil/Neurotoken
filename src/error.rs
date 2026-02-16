use thiserror::Error;

/// Common result type used across Neurotoken.
pub type NeuroResult<T> = Result<T, NeuroError>;

/// Unified error type for Neurotoken operations.
#[derive(Debug, Error)]
pub enum NeuroError {
    #[error("account not found: {0}")]
    AccountNotFound(String),

    #[error("token not found: {0}")]
    TokenNotFound(String),

    #[error("insufficient balance; required={required}, available={available}")]
    InsufficientBalance {
        required: u128,
        available: u128,
    },

    #[error("overflow on operation")]
    Overflow,

    #[error("invalid amount: {0}")]
    InvalidAmount(u128),

    #[error("duplicate account alias: {0}")]
    DuplicateAccountAlias(String),

    #[error("ledger internal error: {0}")]
    Internal(String),
}
