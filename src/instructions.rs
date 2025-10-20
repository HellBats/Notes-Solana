use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;


#[derive(BorshSerialize,BorshDeserialize)]
pub enum NoteInstructions
{
    CreateNote
    {
        title:String,
        body:String,
        id:u64
    },
    UpdateNote
    {
        title:String,
        body:String,
        id:u64
    },
    DeleteNode
    {
        id:u64
    }
}

#[derive(BorshDeserialize)]
struct InstructionPayload
{
    title:String,
    body:String,
    id:u64
}

impl NoteInstructions
{
    pub fn unpack(instruction_data:&[u8]) -> Result<Self,ProgramError>
    {
        let (&variant,rest) = instruction_data.split_first().ok_or(ProgramError::InvalidInstructionData)?;
        let payload = InstructionPayload::try_from_slice(rest)
        .map_err(|_| ProgramError::InvalidInstructionData)?;
        match variant
        {
            0 => {
                    Ok(Self::CreateNote 
                        { 
                            title: payload.title,
                            body: payload.body,
                            id: payload.id 
                        }
                    )
                },
            1 => {
                    Ok(Self::UpdateNote
                        { 
                            title: payload.title,
                            body: payload.body,
                            id: payload.id 
                        }
                    )
                },
            2 => Ok(Self::DeleteNode 
                { 
                    id: payload.id 
                }),
            _ => {Err(ProgramError::InvalidInstructionData)}
        }
    }
}