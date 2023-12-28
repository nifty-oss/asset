use bytemuck::{Pod, Zeroable};
use shank::ShankType;
use solana_program::pubkey::Pubkey;

/// Extension to add a list of creators.
#[repr(C)]
#[derive(Copy, Clone, Pod, ShankType, Zeroable)]
pub struct Creators {
    pub creators: [Creator; 5],
}

/// Creator information.
#[repr(C)]
#[derive(Copy, Clone, Pod, ShankType, Zeroable)]
pub struct Creator {
    /// Pubkey address.
    pub address: Pubkey,
    /// Additional information.
    ///   0. verified flag
    ///   1. share of royalties
    ///   2-7. unused
    pub data: [u8; 8],
}

impl Creator {
    pub fn verified(&self) -> bool {
        self.data[0] == 1
    }

    pub fn set_verified(&mut self, verified: bool) {
        self.data[0] = verified as u8;
    }

    pub fn share(&self) -> u8 {
        self.data[1]
    }

    pub fn set_share(&mut self, share: u8) {
        self.data[1] = share;
    }
}
