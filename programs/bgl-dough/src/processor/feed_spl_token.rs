use borsh::{BorshDeserialize, BorshSerialize};
use mpl_core::{
    accounts::BaseAssetV1,
    fetch_external_plugin_adapter_data_info,
    types::{ExternalPluginAdapterKey, LinkedDataKey, PluginAuthority},
};
use mpl_utils::{
    assert_derivation,
    token::{get_mint_decimals, SPL_TOKEN_PROGRAM_IDS},
};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program::invoke, program_pack::Pack,
};
use spl_token_2022::state::Account as TokenAccount;

use crate::{
    error::BglDoughError,
    instruction::accounts::FeedSplTokenV1Accounts,
    state::{DoughData, ESCROW_PREFIX, MAX_HUNGER, PREFIX, PROGRAM_SIGNER, PROGRAM_SIGNER_BUMP},
};

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub(crate) struct FeedSplTokenV1Args {
    /// The amount of tokens to feed.
    pub amount: u64,
}

pub(crate) fn feed_spl_token_v1<'a>(
    accounts: &'a [AccountInfo<'a>],
    args: FeedSplTokenV1Args,
) -> ProgramResult {
    // Accounts.
    let ctx = FeedSplTokenV1Accounts::context(accounts)?;

    // Guards.
    if ctx.accounts.system_program.key != &solana_program::system_program::ID {
        return Err(BglDoughError::InvalidSystemProgram.into());
    }

    if !SPL_TOKEN_PROGRAM_IDS.contains(ctx.accounts.token_program.key) {
        return Err(BglDoughError::InvalidTokenProgram.into());
    }

    if ctx.accounts.mpl_core_program.key != &mpl_core::ID {
        return Err(BglDoughError::InvalidMplCoreProgram.into());
    }

    if ctx.accounts.program_signer.key != &PROGRAM_SIGNER {
        return Err(BglDoughError::InvalidProgramSigner.into());
    }

    if let Some(associated_token_program) = ctx.accounts.associated_token_program {
        if associated_token_program.key != &spl_associated_token_account::ID {
            return Err(BglDoughError::InvalidAssociatedTokenProgram.into());
        }
    }

    let _ = assert_derivation(
        &crate::ID,
        ctx.accounts.escrow,
        &[ESCROW_PREFIX.as_bytes(), ctx.accounts.asset.key.as_ref()],
        BglDoughError::InvalidAssetEscrow,
    )?;

    // Check if the destination ATA exists, if not create it.
    if ctx.accounts.escrow_ata.lamports() == 0 {
        let create_ata_ix =
            spl_associated_token_account::instruction::create_associated_token_account(
                ctx.accounts.feeder.key,
                ctx.accounts.escrow.key,
                ctx.accounts.mint.key,
                ctx.accounts.token_program.key,
            );

        invoke(
            &create_ata_ix,
            &[
                ctx.accounts.feeder.clone(),
                ctx.accounts.escrow_ata.clone(),
                ctx.accounts.escrow.clone(),
                ctx.accounts.mint.clone(),
                ctx.accounts.token_program.clone(),
                ctx.accounts.system_program.clone(),
            ],
        )?;
    }

    // Get the decimals of the token.
    let decimals = get_mint_decimals(ctx.accounts.mint)?;

    let before_balance = TokenAccount::unpack(&ctx.accounts.escrow_ata.data.borrow())?.amount;

    // Transfer the tokens to the Virtual Pet escrow account.
    let transfer_ix = spl_token_2022::instruction::transfer_checked(
        ctx.accounts.token_program.key,
        ctx.accounts.feeder_ata.key,
        ctx.accounts.mint.key,
        ctx.accounts.escrow_ata.key,
        ctx.accounts.feeder.key,
        &[ctx.accounts.feeder.key],
        args.amount,
        decimals,
    )?;

    invoke(
        &transfer_ix,
        &[
            ctx.accounts.mint.clone(),
            ctx.accounts.feeder_ata.clone(),
            ctx.accounts.escrow_ata.clone(),
            ctx.accounts.feeder.clone(),
        ],
    )?;

    let after_balance = TokenAccount::unpack(&ctx.accounts.escrow_ata.data.borrow())?.amount;

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

    let diff = after_balance - before_balance;
    dough_data.hunger = std::cmp::min(dough_data.hunger as u64 + diff, MAX_HUNGER as u64) as u8;

    mpl_core::instructions::WriteExternalPluginAdapterDataV1Cpi {
        __program: ctx.accounts.mpl_core_program,
        asset: ctx.accounts.asset,
        collection: Some(ctx.accounts.collection),
        payer: ctx.accounts.feeder,
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
