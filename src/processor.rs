use solana_program::{account_info::{next_account_info,AccountInfo},
    entrypoint::ProgramResult,
    msg, program::invoke_signed,
    program_error::ProgramError,
    pubkey::Pubkey, rent::Rent, sysvar::Sysvar
};
use borsh::{BorshDeserialize, BorshSerialize};

use solana_system_interface::program;
use solana_system_interface::instruction::{create_account};
use crate::{errors::NoteErrors, instructions::NoteInstructions};
use crate::state::NoteState;

const MAX_TITLE_LEN: usize = 50;  // Max 50 chars for title
const MAX_BODY_LEN: usize = 200; // Max 200 chars for body

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


            let account_len = 1 + 4 + 4 + MAX_TITLE_LEN + MAX_BODY_LEN+8;
            let rent = Rent::get()?;
            let ren_lamports = rent.minimum_balance(account_len);
            
            msg!("Creating Account...");
            let (note_pda_account,bump_seed) = Pubkey::find_program_address(
                &[note_creator.key.as_ref(),&id.to_ne_bytes()], program_id);
            
            if *pda_account.key != note_pda_account
            {
                msg!("Invalid seeds for PDA");
                return Err(ProgramError::InvalidArgument);
            }


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
            let note_updater = next_account_info(accounts_iter)?;
            let pda_account = next_account_info(accounts_iter)?;
            let _system_program = next_account_info(accounts_iter)?;

            if pda_account.owner != program_id
            {
                return Err(NoteErrors::InvalidNoteAccount.into());
            }           
            if !note_updater.is_signer
            {
                msg!("Missing required signature");
                return Err(ProgramError::MissingRequiredSignature);
            }
            let (note_pda_account,bump_seed) = Pubkey::find_program_address(
                &[note_updater.key.as_ref(),&id.to_ne_bytes()], program_id);
            
            if *pda_account.key != note_pda_account
            {
                msg!("Invalid seeds for PDA");
                return Err(ProgramError::InvalidArgument);
            }
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
        NoteInstructions::DeleteNode { id } => {
            let accounts_iter = &mut accounts.iter();
            let note_deleter = next_account_info(accounts_iter)?;
            let pda_account = next_account_info(accounts_iter)?;
            let system_program = next_account_info(accounts_iter)?;

            if !note_deleter.is_signer {
                msg!("Missing required signature");
                return Err(ProgramError::MissingRequiredSignature);
            }

            if pda_account.owner != program_id {
                msg!("This is not a note account owned by program");
                return Err(NoteErrors::InvalidNoteAccount.into());
            }

            // Check PDA derivation
            let (expected_pda, bump) = Pubkey::find_program_address(
                &[note_deleter.key.as_ref(), &id.to_le_bytes()],
                program_id
            );
            if expected_pda != *pda_account.key {
                msg!("PDA derivation mismatch");
                return Err(ProgramError::InvalidArgument);
            }

            // ✅ Drain lamports to user
            **note_deleter.try_borrow_mut_lamports()? += pda_account.lamports();
            **pda_account.try_borrow_mut_lamports()? = 0;

            msg!("✅ Note deleted and lamports returned");
            Ok(())
        }
    }
}