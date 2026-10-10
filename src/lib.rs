#[cfg(not(feature = "no-entrypoint"))]
mod entrypoint;

pub mod error;
pub mod instruction;
pub mod processor;
pub mod state;

solana_pubkey::declare_id!("TLnxmTpQfoeuPKqJwWeBp9qeg2igUqh2UbrhMZiX53h");
