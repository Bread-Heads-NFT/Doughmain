use borsh::{BorshDeserialize, BorshSerialize};
use mpl_core::types::{ExternalPluginAdapterKey, PluginAuthority};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, sysvar::Sysvar,
};

use crate::{
    error::BglDoughError,
    instruction::accounts::AddToAssetV1Accounts,
    state::{DoughData, PREFIX, PROGRAM_SIGNER, PROGRAM_SIGNER_BUMP},
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct AddToAssetV1Args {
    /// The name of the Dough Pet.
    pub name: String,
}

pub(crate) fn add_to_asset_v1<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: AddToAssetV1Args,
) -> ProgramResult {
    // Accounts.
    let ctx = AddToAssetV1Accounts::context(accounts)?;

    // Guards.
    // The system program is checked in Core.
    if ctx.accounts.mpl_core_program.key != &mpl_core::ID {
        return Err(BglDoughError::InvalidMplCoreProgram.into());
    }

    if ctx.accounts.program_signer.key != &PROGRAM_SIGNER {
        return Err(BglDoughError::InvalidProgramSigner.into());
    }

    let dough_data = DoughData::new(args.name, Clock::get()?.unix_timestamp).try_to_vec()?;
    mpl_core::instructions::WriteExternalPluginAdapterDataV1Cpi {
        __program: ctx.accounts.mpl_core_program,
        asset: ctx.accounts.asset,
        collection: Some(ctx.accounts.collection),
        payer: ctx.accounts.payer,
        authority: Some(ctx.accounts.program_signer),
        buffer: None,
        system_program: ctx.accounts.system_program,
        log_wrapper: None,
        __args: mpl_core::instructions::WriteExternalPluginAdapterDataV1InstructionArgs {
            key: ExternalPluginAdapterKey::LinkedAppData(PluginAuthority::Address {
                address: PROGRAM_SIGNER,
            }),
            data: Some(dough_data),
        },
    }
    .invoke_signed(&[&[PREFIX.as_bytes(), &[PROGRAM_SIGNER_BUMP]]])
}
