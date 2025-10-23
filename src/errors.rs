use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error,Debug)]
pub enum NoteErrors
{
    #[error("Unauthroised access, You are not the owner")]
    Forbidden,
    #[error("Invalid note length, Your text exceeds the allowed limit")]
    InvalidLength,
    #[error("Invalid account, Note account does not exist or note acoount is note owned by this program")]
    InvalidNoteAccount,
}

impl From<NoteErrors> for ProgramError {
    fn from(e: NoteErrors) -> Self {
        ProgramError::Custom(e as u32)
    }
}