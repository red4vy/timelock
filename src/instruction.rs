use {
    solana_instruction::{AccountMeta, Instruction},
    solana_program_error::ProgramError,
    solana_pubkey::Pubkey,
    solana_sdk_ids::system_program,
    std::mem::size_of,
};

#[repr(C, u8)]
#[derive(Debug, PartialEq)]
pub enum TimelockInstruction {
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable, signer]` Authority.
    ///   1. `[writable]` Vault.
    ///   2. `[]` System program.
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
        AccountMeta::new_readonly(system_program::id(), false),
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_packing() {
        let check = TimelockInstruction::InitializeVault {
            amount: 2,
            unlock_timestamp: 1000,
        };
        let packed = check.pack();
        let mut expect = Vec::from([0u8]);
        expect.extend_from_slice(&2u64.to_le_bytes());
        expect.extend_from_slice(&1000i64.to_le_bytes());
        assert_eq!(packed, expect);
        let unpacked = TimelockInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = TimelockInstruction::Withdraw;
        let packed = check.pack();
        let expect = Vec::from([1u8]);
        assert_eq!(packed, expect);
        let unpacked = TimelockInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);
    }

    #[test]
    fn test_instruction_unpack_panic() {
        for i in 0..255u8 {
            for j in 1..10 {
                let mut data = vec![0; j];
                data[0] = i;
                let _no_panic = TimelockInstruction::unpack(&data);
            }
        }
    }
}
