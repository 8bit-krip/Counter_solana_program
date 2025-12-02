use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
    program_error::ProgramError,
};
use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
enum InstructionType {
    Increment(u32),
    Decrement(u32),
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
struct Counter {
    count: u32,
}

entrypoint!(counter_program);

pub fn counter_program(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {

    msg!("Counter program executed!");

    let account_iter = &mut accounts.iter();
    let counter_account_info = next_account_info(account_iter)?;

    if counter_account_info.owner != program_id {
        msg!("Account does not belong to this program!");
        return Err(ProgramError::IllegalOwner);
    }

    let instruction = InstructionType::try_from_slice(instruction_data)?;

    let mut counter_data = Counter::try_from_slice(&counter_account_info.data.borrow())?;

    match instruction {
        InstructionType::Increment(value) => {
            msg!("Increment by: {}", value);
            counter_data.count += value;
        }
        InstructionType::Decrement(value) => {
            msg!("Decrement by: {}", value);
            counter_data.count = counter_data.count.saturating_sub(value);
        }
    }

    counter_data.serialize(&mut &mut counter_account_info.data.borrow_mut()[..])?;
    msg!("New counter value: {}", counter_data.count);

    Ok(())
}
