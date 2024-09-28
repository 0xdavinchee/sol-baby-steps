use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct UpdateArgs {
    pub value: u32,
}

pub enum CounterInstructions {
    Increment(UpdateArgs),
    Decrement(UpdateArgs),
    Update(UpdateArgs),
    Reset,
}

impl CounterInstructions {
    // this function unpacks the input, an instruction from a transaction
    // and we have implemented an instruction handler which returns which
    // counter instruction we want to execute
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => Self::Increment(UpdateArgs::try_from_slice(rest).unwrap()),
            1 => Self::Decrement(UpdateArgs::try_from_slice(rest).unwrap()),
            // try_from_slice tries to deserialize from a slice of bytes
            2 => Self::Update(UpdateArgs::try_from_slice(rest).unwrap()),
            3 => Self::Reset,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
