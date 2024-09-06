use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

use crate::processor::{AddToAssetV1Args, FeedSplTokenV1Args};

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub(crate) enum BglDoughInstruction {
    /// Configure a collection to accept Dough Pets.
    /// Configure Dough Pets for a collection by adding a LinkedAppData to the Collection.
    #[account(0, writable, name="collection", desc = "The address of the collection")]
    #[account(1, writable, signer, name="payer", desc = "The payer for additional rent")]
    #[account(2, optional, signer, name="authority", desc = "Authority for the collection. If not provided, the payer will be used.")]
    #[account(3, name="mpl_core_program", desc = "The MPL Core program")]
    #[account(4, name="system_program", desc = "The system program")]
    AddToCollectionV1,

    /// Create a Dough Pet.
    /// Initialize a new Dough Pet on an asset by writing to the LinkedAppData Data Section of the Asset.
    #[account(0, writable, name="asset", desc = "The address of the asset that will host the Dough Pet")]
    #[account(1, writable, name="collection", desc = "The address of the collection with a LinkedAppData for Dough Pets")]
    #[account(2, writable, signer, name="payer", desc = "The payer for additional rent")]
    #[account(3, name="program_signer", desc = "The program signer which is writing to the Dough Pet")]
    #[account(4, name="mpl_core_program", desc = "The MPL Core program")]
    #[account(5, name="system_program", desc = "The system program")]
    AddToAssetV1(AddToAssetV1Args),

    /// Crank.
    /// Perform all time related actions for the Dough Pet.
    #[account(0, writable, name="asset", desc = "The address of the asset that will host the Dough Pet")]
    #[account(1, writable, name="collection", desc = "The address of the collection with a LinkedAppData for Dough Pets")]
    #[account(2, writable, signer, name="payer", desc = "The payer for additional rent")]
    #[account(3, name="program_signer", desc = "The program signer which is writing to the Dough Pet")]
    #[account(4, name="mpl_core_program", desc = "The MPL Core program")]
    #[account(5, name="system_program", desc = "The system program")]
    CrankV1,

    /// Feed a Token to the Dough Pet.
    /// Transfer an SPL Token or SPL Token 2022 to the Dough Pet via CPI.
    #[account(0, writable, name="asset", desc = "The address of the asset being fed")]
    #[account(1, writable, name="collection", desc = "The address of the VPet collection")]
    #[account(2, writable, name="escrow", desc = "The PDA of the virtual pet from the asset")]
    #[account(3, name="mint", desc = "The address of the SPL Token mint")]
    #[account(4, writable, name="feeder_ata", desc = "The address of the SPL Token account of the feeder")]
    #[account(5, writable, name="escrow_ata", desc = "The address of the SPL Token account of the asset")]
    #[account(6, writable, name="feeder", desc = "The address of the feeder")]
    #[account(7, name="program_signer", desc = "The global signer for the program")]
    #[account(8, name="mpl_core_program", desc = "The MPL Core program")]
    #[account(9, name="system_program", desc = "The system program")]
    #[account(10, name="token_program", desc = "The SPL Token program")]
    #[account(11, optional, name="associated_token_program", desc = "The associated token program")]
    FeedSplTokenV1(FeedSplTokenV1Args),
}
