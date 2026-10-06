use {
    crate::instruction::TimelockInstruction,
    solana_account_info::AccountInfo,
    solana_program_error::ProgramResult,
    solana_pubkey::Pubkey,
};

pub struct Processor {}
impl Processor {
    pub fn process(
        _program_id: &Pubkey,
        _accounts: &[AccountInfo],
        input: &[u8],
    ) -> ProgramResult {
        let instruction = TimelockInstruction::unpack(input)?;
        match instruction {
            _ => todo!(),
        }
    }
}
