use {
    solana_pubkey::Pubkey,
};

#[repr(C)]
pub struct Vault {
    pub authority: Pubkey,
    pub amount: u64,
    pub unlock_timestamp: i64,
}
