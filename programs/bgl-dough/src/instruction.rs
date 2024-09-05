use borsh::{BorshDeserialize, BorshSerialize};
use shank::{ShankContext, ShankInstruction};

use crate::processor::AddToAssetV1Args;

#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, ShankContext, ShankInstruction)]
#[rustfmt::skip]
pub enum BglDoughInstruction {
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
}
