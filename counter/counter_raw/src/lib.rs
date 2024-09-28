use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub mod instructions;

use crate::instructions::CounterInstructions;

// Smart contracts on Solana don't hold state, so we
// need to have access to an Account to place the data in
#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

// Entrypoint to the smart contract
// everything calling our smart contract will call the process_instruction
entrypoint!(process_instruction);
// consider this the "main" of the smart contract
pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instructions_data: &[u8],
) -> ProgramResult {
    msg!("gm!");

    // we unpack the instruction here first using the impl we created in instructions.rs
    let instruction: CounterInstructions = CounterInstructions::unpack(instructions_data)?;

    // We need to make the accounts iterable so we can go over them
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    // now we match the instruction we unpacked above to an actual update of the account
    match instruction {
        CounterInstructions::Increment(args) => {
            counter_account.counter += args.value;
        }
        CounterInstructions::Decrement(args) => {
            if args.value > counter_account.counter {
                counter_account.counter = 0;
            } else {
                counter_account.counter -= args.value;
            }
        }
        CounterInstructions::Reset => {
            counter_account.counter = 0;
        }
        CounterInstructions::Update(args) => {
            counter_account.counter = args.value;
        }
    }

    // we serialize from CounterAccount struct back into bytes and this
    // step persists the changes we made back to Solana account's storage.
    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_counter() {
        // we need to mock a lot of accounts, data, etc.
        // because we are testing in rust
        // when testing client side, this is not necessary
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,  // the signer?
            false, // if this account will sign a txn
            true,  // if account is writable
            &mut lamports,
            &mut data,
            &owner,
            false,            // false because it's not a program/smart contract
            Epoch::default(), // when next rent is due
        );

        let accounts = vec![account];

        let mut increment_instruction_data: Vec<u8> = vec![0];
        let mut decrement_instruction_data: Vec<u8> = vec![1];
        let mut update_instruction_data: Vec<u8> = vec![2];
        let reset_instruction_data: Vec<u8> = vec![3];

        let increment_value: u32 = 21;

        increment_instruction_data.extend_from_slice(&increment_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &increment_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            21
        );

        let decrement_value: u32 = 22;

        decrement_instruction_data.extend_from_slice(&decrement_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &decrement_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        let update_value: u32 = 69;
        update_instruction_data.extend_from_slice(&update_value.to_le_bytes());

        process_instruction(&program_id, &accounts, &update_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            69
        );

        process_instruction(&program_id, &accounts, &reset_instruction_data).unwrap();

        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
