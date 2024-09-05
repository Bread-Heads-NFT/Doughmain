use num_derive::FromPrimitive;
use solana_program::{
    decode_error::DecodeError,
    msg,
    program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq, FromPrimitive)]
pub enum BglDoughError {
    /// 0 - Invalid System Program
    #[error("Invalid System Program")]
    InvalidSystemProgram,
    /// 1 - Error deserializing account
    #[error("Error deserializing account")]
    DeserializationError,
    /// 2 - Error serializing account
    #[error("Error serializing account")]
    SerializationError,
    /// 3 - Invalid MPL Core Program
    #[error("Invalid MPL Core Program")]
    InvalidMplCoreProgram,
    /// 4 - Invalid Program Signer
    #[error("Invalid Program Signer")]
    InvalidProgramSigner,
}

impl PrintProgramError for BglDoughError {
    fn print<E>(&self) {
        msg!(&self.to_string());
    }
}

impl From<BglDoughError> for ProgramError {
    fn from(e: BglDoughError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl<T> DecodeError<T> for BglDoughError {
    fn type_of() -> &'static str {
        "Bgl Dough Error"
    }
}
