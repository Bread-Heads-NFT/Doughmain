mod add_to_asset;
mod add_to_collection;
mod crank;
mod feed_spl_token;

pub(crate) use add_to_asset::*;
pub(crate) use add_to_collection::*;
pub(crate) use crank::*;
pub(crate) use feed_spl_token::*;

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
            add_to_collection_v1(accounts)
        }
        BglDoughInstruction::AddToAssetV1(args) => {
            msg!("Instruction: AddToAssetV1");
            add_to_asset_v1(accounts, args)
        }
        BglDoughInstruction::CrankV1 => {
            msg!("Instruction: CrankV1");
            crank_v1(accounts)
        }
        BglDoughInstruction::FeedSplTokenV1(args) => {
            msg!("Instruction: FeedSplTokenV1");
            feed_spl_token_v1(accounts, args)
        }
    }
}
