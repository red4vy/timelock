use {
    crate::{
        error::TimelockError,
        instruction::TimelockInstruction,
        state::Vault,
    },
    solana_account_info::{next_account_info, AccountInfo},
    solana_clock::Clock,
    solana_program_error::{ProgramError, ProgramResult},
    solana_program_pack::{IsInitialized, Pack},
    solana_pubkey::Pubkey,
    solana_rent::{Rent, sysvar::GetSysvar},
    solana_sdk_ids::system_program,
};

pub struct Processor {}
impl Processor {
    pub fn process_initialize_vault(
        accounts: &[AccountInfo],
        amount: u64,
        unlock_timestamp: i64,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let authority_info = next_account_info(account_info_iter)?;
        let vault_info = next_account_info(account_info_iter)?;
        let rent = Rent::get()?;

        Self::check_signer(authority_info)?;

        let mut vault = Vault::unpack_unchecked(&vault_info.data.borrow())?;
        if vault.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        if amount == 0 {
            return Err(ProgramError::InvalidArgument);
        }
        let lamports = vault_info.lamports()
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;
        if !rent.is_exempt(lamports, vault_info.data_len()) {
            return Err(ProgramError::AccountNotRentExempt);
        }
        if unlock_timestamp <= Clock::get()?.unix_timestamp {
            return Err(ProgramError::InvalidArgument);
        }

        let mut authority_lamports = authority_info.try_borrow_mut_lamports()?;
        let mut vault_lamports = vault_info.try_borrow_mut_lamports()?;
        **authority_lamports = authority_lamports
            .checked_sub(amount)
            .ok_or(ProgramError::InsufficientFunds)?;
        **vault_lamports = vault_lamports
            .checked_add(amount)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        vault.authority = *authority_info.key;
        vault.amount = amount;
        vault.unlock_timestamp = unlock_timestamp;

        Vault::pack(vault, &mut vault_info.data.borrow_mut())?;
        Ok(())
    }
    pub fn process_withdraw(
        accounts: &[AccountInfo],
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let authority_info = next_account_info(account_info_iter)?;
        let vault_info = next_account_info(account_info_iter)?;

        Self::check_signer(authority_info)?;

        let vault = Vault::unpack(&vault_info.data.borrow())?;
        if vault.authority != *authority_info.key {
            return Err(ProgramError::IncorrectAuthority);
        }
        if vault.unlock_timestamp > Clock::get()?.unix_timestamp {
            return Err(TimelockError::NotUnlockedYet.into());
        }

        let lamports = vault_info.lamports();
        **vault_info.try_borrow_mut_lamports()? = 0;
        **authority_info.try_borrow_mut_lamports()? = authority_info
            .lamports()
            .checked_add(lamports)
            .ok_or(ProgramError::ArithmeticOverflow)?;

        vault_info.assign(&system_program::id());
        vault_info.resize(0)?;
        Ok(())
    }
    pub fn process(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = TimelockInstruction::unpack(input)?;
        match instruction {
            TimelockInstruction::InitializeVault {
                amount, unlock_timestamp,
            } => {
                Self::process_initialize_vault(accounts, amount, unlock_timestamp)
            },
            TimelockInstruction::Withdraw => {
                Self::process_withdraw(accounts)
            },
        }
    }

    // Helpers
    fn check_signer(account: &AccountInfo) -> ProgramResult {
        if !account.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }
        Ok(())
    }
}
