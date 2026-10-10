use {
    solana_instruction::{AccountMeta, Instruction},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    std::mem::size_of,
};

#[repr(C, u8)]
pub enum TimelockInstruction {
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable, signer]` Authority.
    ///   1. `[writable]` Vault.
    InitializeVault {
        amount: u64,
        unlock_timestamp: i64,
    },

    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable, signer]` Authority.
    ///   1. `[writable]` Vault.
    Withdraw,
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
            1 => Self::Withdraw,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            Self::InitializeVault {
                amount,
                unlock_timestamp,
            } => {
                buf.push(0);
                buf.extend_from_slice(&amount.to_le_bytes());
                buf.extend_from_slice(&unlock_timestamp.to_le_bytes());
            }
            Self::Withdraw => buf.push(1),
        }
        buf
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

/// Creates an `InitializeVault` instruction.
pub fn initialize_vault(
    timelock_program: &Pubkey,
    authority_pubkey: &Pubkey,
    vault_pubkey: &Pubkey,
    amount: u64,
    unlock_timestamp: i64,
) -> Instruction {
    let data = TimelockInstruction::InitializeVault {
        amount,
        unlock_timestamp,
    }
    .pack();
    let accounts = vec![
        AccountMeta::new(*authority_pubkey, true),
        AccountMeta::new(*vault_pubkey, false),
    ];

    Instruction {
        program_id: *timelock_program,
        accounts,
        data,
    }
}

/// Creates a `Withdraw` instruction.
pub fn withdraw(
    timelock_program: &Pubkey,
    authority_pubkey: &Pubkey,
    vault_pubkey: &Pubkey,
) -> Instruction {
    let data = TimelockInstruction::Withdraw.pack();
    let accounts = vec![
        AccountMeta::new(*authority_pubkey, true),
        AccountMeta::new(*vault_pubkey, false),
    ];

    Instruction {
        program_id: *timelock_program,
        accounts,
        data,
    }
}
