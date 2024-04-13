use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Custom error message")]
    CustomError,

    #[msg("An overflow/underflow occurred during a math operation")]
    MathError,
}
