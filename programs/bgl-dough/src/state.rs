use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{pubkey, pubkey::Pubkey};

pub(crate) const PREFIX: &str = "bgl_dough";
pub(crate) const ESCROW_PREFIX: &str = "escrow";

pub(crate) const PROGRAM_SIGNER: Pubkey = pubkey!("8rNE2yecH6AsLVpSPmbUE2UTCcQDhzah9rab6kW1iENy");
pub(crate) const PROGRAM_SIGNER_BUMP: u8 = 254;

/// Set to roughly 72.8 hours (2^18), closest to 3 days while still being a power of 2.
// pub(crate) const HAPPINESS_PERIOD_SHIFT: u8 = 18;
/// For testing
pub(crate) const HAPPINESS_PERIOD_SHIFT: u8 = 6;
/// Set to roughly 18.2 hours (2^16), closest to a day while still being a power of 2.
// pub(crate) const HUNGER_PERIOD_SHIFT: u8 = 16;
/// For testing
pub(crate) const HUNGER_PERIOD_SHIFT: u8 = 4;

/// The maximum health.
pub(crate) const MAX_HEALTH: u8 = 10;
/// The maximum happiness.
pub(crate) const MAX_HAPPINESS: u8 = 10;
/// The maximum hunger.
pub(crate) const MAX_HUNGER: u8 = 10;

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
    /// The total points earned by the Dough Pet.
    pub points: u32,
    /// The last time the crank was run.
    pub last_crank: i64,
}

impl DoughData {
    pub(crate) fn new(name: String, start_time: i64) -> Self {
        Self {
            name,
            health: MAX_HEALTH,
            happiness: MAX_HAPPINESS,
            hunger: MAX_HUNGER,
            points: 0,
            last_crank: start_time,
        }
    }
}
