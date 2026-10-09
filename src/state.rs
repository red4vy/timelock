use {
    arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs},
    solana_program_error::ProgramError,
    solana_program_pack::{IsInitialized, Pack, Sealed},
    solana_pubkey::Pubkey,
};

#[repr(C)]
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
