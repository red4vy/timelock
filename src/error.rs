use solana_program_error::ProgramError;

pub enum TimelockError {
    NotUnlockedYet,
}
impl From<TimelockError> for ProgramError {
    fn from(e: TimelockError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
