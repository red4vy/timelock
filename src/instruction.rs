use solana_program_error::ProgramError;

#[repr(u8)]
pub enum TimelockInstruction {
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable, signer]` Authority.
    ///   1. `[writable]` Vault.
    InitializeVault {
        amount: u64,
        unlock_timestamp: i64,
    },
}
impl TimelockInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(match tag {
            0 => {
                let (amount, rest) = Self::unpack_u64(rest)?;
                let (unlock_timestamp, _rest) = Self::unpack_i64(rest)?;
                Self::InitializeVault {
                    amount,
                    unlock_timestamp,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    // Helpers
    fn unpack_u64(input: &[u8]) -> Result<(u64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (value, rest) = input.split_at(8);
            let value = value
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(u64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((value, rest))
        } else {
            Err(ProgramError::InvalidInstructionData)
        }
    }
    fn unpack_i64(input: &[u8]) -> Result<(i64, &[u8]), ProgramError> {
        if input.len() >= 8 {
            let (value, rest) = input.split_at(8);
            let value = value
                .get(..8)
                .and_then(|slice| slice.try_into().ok())
                .map(i64::from_le_bytes)
                .ok_or(ProgramError::InvalidInstructionData)?;
            Ok((value, rest))
        } else {
            Err(ProgramError::InvalidInstructionData)
        }
    }
}
