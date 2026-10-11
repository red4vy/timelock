use {
    arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs},
    solana_program_error::ProgramError,
    solana_program_pack::{IsInitialized, Pack, Sealed},
    solana_pubkey::Pubkey,
};

#[repr(C)]
#[derive(Debug, PartialEq)]
pub struct Vault {
    pub authority: Pubkey,
    pub amount: u64,
    pub unlock_timestamp: i64,
}
impl Sealed for Vault {}
impl IsInitialized for Vault {
    fn is_initialized(&self) -> bool {
        self.authority != Pubkey::default()
            || self.amount != 0
            || self.unlock_timestamp != 0
    }
}
impl Pack for Vault {
    const LEN: usize = 48;
    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, 48];
        let (authority, amount, unlock_timestamp) =
            array_refs![src, 32, 8, 8];

        Ok(Vault {
            authority: Pubkey::new_from_array(*authority),
            amount: u64::from_le_bytes(*amount),
            unlock_timestamp: i64::from_le_bytes(*unlock_timestamp),
        })
    }
    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, 48];
        let (authority, amount, unlock_timestamp) =
            mut_array_refs![dst, 32, 8, 8];

        authority.copy_from_slice(self.authority.as_ref());
        *amount = self.amount.to_le_bytes();
        *unlock_timestamp = self.unlock_timestamp.to_le_bytes();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_vault_packing() {
        let src: [u8; 48] = [0; 48];
        let mut vault = Vault::unpack_from_slice(&src).unwrap();
        assert!(!vault.is_initialized());

        vault.authority = Pubkey::new_from_array([1u8; 32]);
        vault.amount = 200;
        vault.unlock_timestamp = 2_000_000_000;

        let mut expect: [u8; 48] = [0; 48];
        expect[0..32].copy_from_slice(&[1u8; 32]);
        expect[32..40].copy_from_slice(&200u64.to_le_bytes());
        expect[40..48].copy_from_slice(&2_000_000_000i64.to_le_bytes());

        let mut packed: [u8; 48] = [0; 48];
        Vault::pack_into_slice(&vault, &mut packed);
        assert_eq!(packed, expect);

        let unpacked = Vault::unpack_from_slice(&packed).unwrap();
        assert_eq!(unpacked, vault);
    }
}
