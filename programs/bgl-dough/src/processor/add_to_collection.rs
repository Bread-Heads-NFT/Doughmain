use mpl_core::types::{
    ExternalPluginAdapterInitInfo, ExternalPluginAdapterSchema, LinkedAppDataInitInfo,
    PluginAuthority,
};
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult};

use crate::{
    error::BglDoughError, instruction::accounts::AddToCollectionV1Accounts, state::PROGRAM_SIGNER,
};

pub(crate) fn add_to_collection_v1<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = AddToCollectionV1Accounts::context(accounts)?;

    // Guards.
    // The system program is checked in Core.
    if ctx.accounts.mpl_core_program.key != &mpl_core::ID {
        return Err(BglDoughError::InvalidMplCoreProgram.into());
    }

    mpl_core::instructions::AddCollectionExternalPluginAdapterV1Cpi {
        __program: ctx.accounts.mpl_core_program,
        collection: ctx.accounts.collection,
        payer: ctx.accounts.payer,
        authority: ctx.accounts.authority,
        system_program: ctx.accounts.system_program,
        log_wrapper: None,
        __args: mpl_core::instructions::AddCollectionExternalPluginAdapterV1InstructionArgs {
            init_info: ExternalPluginAdapterInitInfo::LinkedAppData(LinkedAppDataInitInfo {
                data_authority: PluginAuthority::Address {
                    address: PROGRAM_SIGNER,
                },
                init_plugin_authority: None,
                schema: Some(ExternalPluginAdapterSchema::Binary),
            }),
        },
    }
    .invoke()?;
    Ok(())
}
