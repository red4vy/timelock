use {
    mollusk_svm::{Mollusk, program, result::Check},
    solana_account::{Account, ReadableAccount},
    solana_instruction::Instruction,
    solana_program_error::{ProgramError, ProgramResult},
    solana_program_pack::Pack,
    solana_pubkey::Pubkey,
    solana_rent::Rent,
    solana_sdk_ids::system_program,
    solana_system_interface::error::SystemError,
    timelock::{
        instruction::initialize_vault,
        state::Vault,
    },
};

fn do_process_instruction(
    mollusk: &mut Mollusk,
    instruction: Instruction,
    mut accounts: Vec<&mut Account>,
    checks: &[Check],
) -> ProgramResult {
    let instruction_accounts: Vec<(Pubkey, Account)> = instruction
        .accounts
        .iter()
        .zip(&accounts)
        .map(|(account_meta, account)| (account_meta.pubkey, (*account).clone()))
        .collect();

    let result =
        mollusk.process_and_validate_instruction(&instruction, &instruction_accounts, checks);

    for (original, (_, updated)) in accounts
        .iter_mut()
        .zip(result.resulting_accounts.into_iter())
    {
        original.data = updated.data().to_vec();
        original.lamports = updated.lamports();
        original.owner = *updated.owner();
    }

    result
        .raw_result
        .map_err(|e| ProgramError::try_from(e).unwrap())
}

fn vault_minimum_balance() -> u64 {
    Rent::default().minimum_balance(Vault::get_packed_len())
}

#[test]
fn test_initialize_vault() {
    let mut mollusk = Mollusk::new(&timelock::id(), "timelock");
    mollusk.sysvars.clock.unix_timestamp = 1_700_000_000;

    let program_id = timelock::id();
    let authority_key = Pubkey::new_unique();
    let mut authority_account = Account::new(42, 0, &system_program::id());
    let vault_key = Pubkey::new_unique();
    let mut vault_account = Account::new(0, Vault::get_packed_len(), &program_id);
    let (_, mut system_program_account) = program::keyed_account_for_system_program();

    let mut instruction = initialize_vault(&program_id, &authority_key, &vault_key, 1, 2_000_000_000);
    instruction.accounts[0].is_signer = false;

    // authority does not sign
    assert_eq!(
        Err(ProgramError::MissingRequiredSignature),
        do_process_instruction(
            &mut mollusk,
            instruction,
            vec![&mut authority_account, &mut vault_account, &mut system_program_account],
            &[Check::err(ProgramError::MissingRequiredSignature)],
        ),
    );

    // vault is not rent-exempt
    assert_eq!(
        Err(ProgramError::AccountNotRentExempt),
        do_process_instruction(
            &mut mollusk,
            initialize_vault(&program_id, &authority_key, &vault_key, 1, 2_000_000_000),
            vec![&mut authority_account, &mut vault_account, &mut system_program_account],
            &[Check::err(ProgramError::AccountNotRentExempt)],
        ),
    );

    vault_account.lamports = vault_minimum_balance();

    // amount == 0
    assert_eq!(
        Err(ProgramError::InvalidArgument),
        do_process_instruction(
            &mut mollusk,
            initialize_vault(&program_id, &authority_key, &vault_key, 0, 2_000_000_000),
            vec![&mut authority_account, &mut vault_account, &mut system_program_account],
            &[Check::err(ProgramError::InvalidArgument)],
        ),
    );

    // amount > authority_account.lamports
    assert_eq!(
        Err(SystemError::ResultWithNegativeLamports.into()),
        do_process_instruction(
            &mut mollusk,
            initialize_vault(&program_id, &authority_key, &vault_key, 50, 2_000_000_000),
            vec![&mut authority_account, &mut vault_account, &mut system_program_account],
            &[Check::err(SystemError::ResultWithNegativeLamports.into())],
        ),
    );

    // unlock_timestamp < current_timestamp
    assert_eq!(
        Err(ProgramError::InvalidArgument),
        do_process_instruction(
            &mut mollusk,
            initialize_vault(&program_id, &authority_key, &vault_key, 1, 1_600_000_000),
            vec![&mut authority_account, &mut vault_account, &mut system_program_account],
            &[Check::err(ProgramError::InvalidArgument)],
        ),
    );

    // unlock_timestamp == current_timestamp
    assert_eq!(
        Err(ProgramError::InvalidArgument),
        do_process_instruction(
            &mut mollusk,
            initialize_vault(&program_id, &authority_key, &vault_key, 1, 1_700_000_000),
            vec![&mut authority_account, &mut vault_account, &mut system_program_account],
            &[Check::err(ProgramError::InvalidArgument)],
        ),
    );

    do_process_instruction(
        &mut mollusk,
        initialize_vault(&program_id, &authority_key, &vault_key, 1, 2_000_000_000),
        vec![&mut authority_account, &mut vault_account, &mut system_program_account],
        &[Check::success()],
    )
    .unwrap();

    assert_eq!(
        authority_account.lamports,
        41,
    );
    assert_eq!(
        vault_account.lamports,
        vault_minimum_balance() + 1,
    );

    // create twice
    assert_eq!(
        Err(ProgramError::AccountAlreadyInitialized),
        do_process_instruction(
            &mut mollusk,
            initialize_vault(&program_id, &authority_key, &vault_key, 1, 2_000_000_000),
            vec![&mut authority_account, &mut vault_account, &mut system_program_account],
            &[Check::err(ProgramError::AccountAlreadyInitialized)],
        ),
    );
}
