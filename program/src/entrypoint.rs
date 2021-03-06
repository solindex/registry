use crate::error::TokenRegistryError;
use crate::processor::Processor;
use num_traits::FromPrimitive;
use solana_program::{
    account_info::AccountInfo, decode_error::DecodeError, entrypoint::ProgramResult, msg,
    program_error::PrintProgramError, pubkey::Pubkey,
};

#[cfg(not(feature = "no-entrypoint"))]
use solana_program::entrypoint;
#[cfg(not(feature = "no-entrypoint"))]
entrypoint!(process_instruction);

/// The entrypoint to the Token registry program
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Entrypoint");
    if let Err(error) = Processor::process_instruction(program_id, accounts, instruction_data) {
        // catch the error so we can print it
        error.print::<TokenRegistryError>();
        return Err(error);
    }
    Ok(())
}

impl PrintProgramError for TokenRegistryError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError + FromPrimitive,
    {
        match self {
            TokenRegistryError::AccountNotRentExempt => {
                msg!("Error: Account not rent exempt")
            }
            TokenRegistryError::InvalidKey => {
                msg!("Error: Invalid key")
            }
            TokenRegistryError::InvalidTld => {
                msg!("Error: Invalid token TLD")
            }
            TokenRegistryError::NonWhiteListedSigner => {
                msg!("Error: Non white listed signer")
            }
            TokenRegistryError::InvalidNameProvided => {
                msg!("Error: Invalid name provided")
            }
        }
    }
}
