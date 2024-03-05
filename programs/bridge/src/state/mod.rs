pub mod bridged_asset;
mod vault;

pub use vault::*;

use bytemuck::{Pod, Zeroable};
use shank::ShankType;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, ShankType)]
pub enum Discriminator {
    #[default]
    Uninitialized,
    Vault,
}

impl From<u8> for Discriminator {
    fn from(value: u8) -> Self {
        match value {
            0 => Discriminator::Uninitialized,
            1 => Discriminator::Vault,
            _ => panic!("invalid key value: {value}"),
        }
    }
}

impl From<Discriminator> for u8 {
    fn from(value: Discriminator) -> Self {
        match value {
            Discriminator::Uninitialized => 0,
            Discriminator::Vault => 1,
        }
    }
}

unsafe impl Pod for Discriminator {}

unsafe impl Zeroable for Discriminator {}

/// Represent a vault state.
///
/// A vault on an "Active" state is holding a token in its token account and the corresponding asset
/// has been transferred to the original token owner. A bridge on an "Idle" state represents the case
/// where the asset for the token has been created, but the asset is held in the vault account.
#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, ShankType)]
pub enum State {
    #[default]
    Idle,
    Active,
}

impl From<u8> for State {
    fn from(value: u8) -> Self {
        match value {
            0 => State::Idle,
            1 => State::Active,
            _ => panic!("invalid state value: {value}"),
        }
    }
}

impl From<State> for u8 {
    fn from(value: State) -> Self {
        match value {
            State::Idle => 0,
            State::Active => 1,
        }
    }
}

unsafe impl Pod for State {}

unsafe impl Zeroable for State {}
