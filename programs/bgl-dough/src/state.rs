use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey, pubkey::Pubkey};

pub(crate) const PREFIX: &str = "bgl_dough";
pub(crate) const PROGRAM_SIGNER: Pubkey = pubkey!("8rNE2yecH6AsLVpSPmbUE2UTCcQDhzah9rab6kW1iENy");
pub(crate) const PROGRAM_SIGNER_BUMP: u8 = 254;

#[repr(C)]
#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug, Clone)]
pub struct DoughData {
    /// The name of the Dough Pet.
    pub name: String,
    /// The overall health of the Dough Pet.
    pub health: u8,
    /// The happiness of the Dough Pet.
    pub happiness: u8,
    /// How hungry the Dough Pet is.
    pub hunger: u8,
}

impl DoughData {
    pub(crate) fn new(name: String) -> Self {
        Self {
            name,
            health: 10,
            happiness: 10,
            hunger: 10,
        }
    }
}
