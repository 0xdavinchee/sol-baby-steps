use borsh::{BorshDeserialize, BorshSerialize};
use {
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        program::invoke_signed,
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
    },
    spl_token::{
        instruction::transfer_checked,
        state::{Mint},
    },
};

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct TransferArgs {
    pub amount: u64,
}

solana_program::entrypoint!(process_instruction);
pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let source_info = next_account_info(account_info_iter)?;
    let mint_info = next_account_info(account_info_iter)?;
    let destination_info = next_account_info(account_info_iter)?;
    let authority_info = next_account_info(account_info_iter)?;
    let token_program_info = next_account_info(account_info_iter)?;

    let (expected_authority, bump_seed) = Pubkey::find_program_address(&[b"authority"], program_id);

    if expected_authority != *authority_info.key {
        return Err(ProgramError::InvalidSeeds);
    }

    // take amount as an instruction data arg
    let amount = TransferArgs::try_from_slice(instruction_data)?.amount;

    // unpack which amount
    let mint = Mint::unpack(&mint_info.try_borrow_data()?)?;
    // how much decimals
    let decimals = mint.decimals;

    // invoke transfer from spl_token
    invoke_signed(
        &transfer_checked(
            token_program_info.key,
            source_info.key,
            mint_info.key,
            destination_info.key,
            authority_info.key,
            &[],
            amount,
            decimals,
        )
        .unwrap(),
        &[
            source_info.clone(),
            mint_info.clone(),
            destination_info.clone(),
            authority_info.clone(),
            token_program_info.clone(),
        ],
        &[&[b"authority", &[bump_seed]]],
    )
}
