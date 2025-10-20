use borsh::{BorshDeserialize, BorshSerialize};




#[derive(BorshDeserialize,BorshSerialize,Default)]
pub struct NoteState
{
    pub is_initialized:bool,
    pub title:String,
    pub body:String,
    pub id:u64
}