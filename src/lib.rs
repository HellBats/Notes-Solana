use solana_program::{account_info::AccountInfo, entrypoint::{entrypoint, ProgramResult}, msg, program_error::ProgramError, pubkey::Pubkey};

mod processor;
mod instructions;
mod state;

entrypoint!(process_instruction);
fn process_instruction(
    program_id: &Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]) -> ProgramResult
{
    msg!("Instruction recieved");
    msg!("Program ID: {}", program_id);

    if instruction_data.is_empty() {
        return Err(ProgramError::InvalidInstructionData);
    }
    processor::processor(program_id,accounts,instruction_data)?;
    Ok(())
}