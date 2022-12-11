use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::CalculatorError::InvalidInstruction;

pub enum CalculatorInstruction {
    // calculates a+b
    Add {
        term_a: u64,
        term_b: u64,
    },
    //calculates a-b
    Subtract {
        term_a: u64,
        term_b: u64,
    },
}

impl CalculatorInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::Add {
                term_a: Self::unpack_term_a(rest)?,
                term_b: Self::unpack_term_b(rest)?,
            },
            //Paste your code here - for Exchange matching
            1 => Self::Subtract {
                term_a: Self::unpack_term_a(rest)?,
                term_b: Self::unpack_term_b(rest)?,
            },
            
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_term_a(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }

    fn unpack_term_b(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(8..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
