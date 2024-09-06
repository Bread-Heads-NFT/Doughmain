use borsh::{BorshDeserialize, BorshSerialize};
use mpl_core::{
    accounts::BaseAssetV1,
    fetch_external_plugin_adapter_data_info,
    types::{ExternalPluginAdapterKey, LinkedDataKey, PluginAuthority},
};
use solana_program::{
    account_info::AccountInfo, clock::Clock, entrypoint::ProgramResult, sysvar::Sysvar,
};

use crate::{
    error::BglDoughError,
    instruction::accounts::CrankV1Accounts,
    state::{
        DoughData, HAPPINESS_PERIOD_SHIFT, HUNGER_PERIOD_SHIFT, PREFIX, PROGRAM_SIGNER,
        PROGRAM_SIGNER_BUMP,
    },
};

pub(crate) fn crank_v1<'a>(accounts: &'a [AccountInfo<'a>]) -> ProgramResult {
    // Accounts.
    let ctx = CrankV1Accounts::context(accounts)?;

    // Guards.
    // The system program is checked in Core.
    if ctx.accounts.mpl_core_program.key != &mpl_core::ID {
        return Err(BglDoughError::InvalidMplCoreProgram.into());
    }

    if ctx.accounts.program_signer.key != &PROGRAM_SIGNER {
        return Err(BglDoughError::InvalidProgramSigner.into());
    }

    let (data_offset, data_len) = fetch_external_plugin_adapter_data_info::<BaseAssetV1>(
        ctx.accounts.asset,
        None,
        &ExternalPluginAdapterKey::DataSection(LinkedDataKey::LinkedAppData(
            PluginAuthority::Address {
                address: PROGRAM_SIGNER,
            },
        )),
    )?;
    let mut dough_data = DoughData::try_from_slice(
        &ctx.accounts.asset.data.borrow()[data_offset..data_offset + data_len],
    )?;

    let timestamp = Clock::get().unwrap().unix_timestamp;
    let new_happiness = calculate_new_stat(
        dough_data.happiness,
        dough_data.last_crank,
        timestamp,
        HAPPINESS_PERIOD_SHIFT,
    );
    let new_hunger = calculate_new_stat(
        dough_data.hunger,
        dough_data.last_crank,
        timestamp,
        HUNGER_PERIOD_SHIFT,
    );

    if let Some(new_happiness) = new_happiness {
        dough_data.happiness = new_happiness;
    }
    if let Some(new_hunger) = new_hunger {
        dough_data.hunger = new_hunger;
    }
    if new_happiness.is_some() || new_hunger.is_some() {
        dough_data.last_crank = timestamp;
    }
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
            data: Some(dough_data.try_to_vec()?),
        },
    }
    .invoke_signed(&[&[PREFIX.as_bytes(), &[PROGRAM_SIGNER_BUMP]]])
}

fn calculate_new_stat(current: u8, last_crank: i64, current_time: i64, shift: u8) -> Option<u8> {
    let last_counter = (last_crank as u64) >> shift;
    let current_counter = (current_time as u64) >> shift;
    let decrement = current_counter - last_counter;

    if decrement == 0 {
        None
    } else {
        Some(current.saturating_sub(decrement as u8))
    }
}
