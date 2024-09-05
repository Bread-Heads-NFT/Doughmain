mod add_to_asset;
mod add_to_collection;

pub(crate) use add_to_asset::*;
pub(crate) use add_to_collection::*;

use borsh::BorshDeserialize;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instruction::BglDoughInstruction;

pub fn process_instruction<'a>(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction: BglDoughInstruction = BglDoughInstruction::try_from_slice(instruction_data)?;
    match instruction {
        BglDoughInstruction::AddToCollectionV1 => {
            msg!("Instruction: AddToCollectionV1");
            add_to_collection(accounts)
        }
        BglDoughInstruction::AddToAssetV1(args) => {
            msg!("Instruction: AddToAssetV1");
            add_to_asset(accounts, args)
        }
    }
}
