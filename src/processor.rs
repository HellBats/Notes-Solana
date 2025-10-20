use solana_program::{account_info::{next_account_info, AccountInfo}, entrypoint::ProgramResult, msg, program::invoke_signed, pubkey::Pubkey, rent::Rent, sysvar::Sysvar};

use borsh::{BorshDeserialize, BorshSerialize};

use solana_system_interface::instruction::{create_account};
use crate::instructions::NoteInstructions;
use crate::state::NoteState;

pub fn processor(program_id: &Pubkey,
    accounts:&[AccountInfo],
    instruction_data:&[u8]) -> ProgramResult
{
    let client_request = NoteInstructions::unpack(instruction_data)?;
    match client_request
    {
        NoteInstructions::CreateNote { title, body, id } =>
        {
            let accounts_iter = &mut accounts.iter();
            let note_creator = next_account_info(accounts_iter)?;
            let pda_account = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;


            let account_len = 4 + 4 + title.len() + body.len()+8;
            let rent = Rent::get()?;
            let ren_lamports = rent.minimum_balance(account_len);
            
            msg!("Creating Account...");
            let (_note_pda_account,bump_seed) = Pubkey::find_program_address(
                &[note_creator.key.as_ref(),&id.to_ne_bytes()], program_id);
            
            invoke_signed(&create_account(
                note_creator.key,
                pda_account.key,
                ren_lamports,
                account_len.try_into().unwrap(),
                program_id,
            ), &[note_creator.clone(), pda_account.clone(), system_program.clone()],
             &[&[note_creator.key.as_ref(), id.to_ne_bytes().as_ref(), &[bump_seed]]])?;
            let mut account_data = NoteState::try_from_slice(&pda_account.data.borrow())
                .unwrap_or(NoteState::default());
            msg!("Account Created");
            msg!("Initializing Data");
            account_data.title = title;
            account_data.body = body;
            account_data.id = id;
            account_data.is_initialized = true;

            account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
            msg!("Data succesfully Initialize");
            return Ok(())
        },
        NoteInstructions::UpdateNote { title, body, id } =>
        {
            let accounts_iter = &mut accounts.iter();
            let _note_updater = next_account_info(accounts_iter)?;
            let pda_account = next_account_info(accounts_iter)?;
            let _system_program = next_account_info(accounts_iter)?;

            
            msg!("Updating Account...");
            let mut account_data = NoteState::try_from_slice(&pda_account.data.borrow())
                .unwrap_or(NoteState::default());
            account_data.title = title;
            account_data.body = body;
            account_data.id = id;
            account_data.is_initialized = true;

            account_data.serialize(&mut &mut pda_account.data.borrow_mut()[..])?;
            msg!("Data succesfully Updated");
            return Ok(())
        }
        NoteInstructions::DeleteNode {id:_} =>
        {
            return Ok(())
        }
    }
}