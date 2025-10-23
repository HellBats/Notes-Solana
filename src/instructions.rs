use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{msg, program_error::ProgramError};


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

impl NoteInstructions {
    pub fn unpack(instruction_data: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = instruction_data.split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match variant {
            0 | 1 => {
                // Create & Update expect full payload
                let payload = InstructionPayload::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;

                if variant == 0 {
                    Ok(Self::CreateNote {
                        title: payload.title,
                        body: payload.body,
                        id: payload.id,
                    })
                } else {
                    Ok(Self::UpdateNote {
                        title: payload.title,
                        body: payload.body,
                        id: payload.id,
                    })
                }
            }

            2 => {
                // ✅ DeleteNode expects only id: u64 (8 bytes)
                if rest.len() != 8 {
                    return Err(ProgramError::InvalidInstructionData);
                }
                let id = u64::from_le_bytes(rest.try_into().unwrap());
                Ok(Self::DeleteNode { id })
            }

            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
