mod asset;
mod delegate;

use std::ops::{Deref, DerefMut};

pub use asset::*;
pub use delegate::*;

use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use podded::pod::Nullable;
use shank::ShankType;
use solana_program::pubkey::Pubkey;

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, ShankType)]
pub enum Discriminator {
    #[default]
    Uninitialized,
    Asset,
}

impl From<u8> for Discriminator {
    fn from(value: u8) -> Self {
        match value {
            0 => Discriminator::Uninitialized,
            1 => Discriminator::Asset,
            _ => panic!("invalid key value: {value}"),
        }
    }
}

impl From<Discriminator> for u8 {
    fn from(value: Discriminator) -> Self {
        match value {
            Discriminator::Uninitialized => 0,
            Discriminator::Asset => 1,
        }
    }
}

unsafe impl Pod for Discriminator {}

unsafe impl Zeroable for Discriminator {}

/// Defines the standard of an asset.
#[repr(u8)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, Default, PartialEq, ShankType)]
pub enum Standard {
    /// A unique (one-of-a-kind) asset.
    #[default]
    NonFungible,

    /// A unique asset representing a subscription.
    ///
    /// Holding this asset grants the holder access to a service, but does not
    /// grant permanent ownership rights.
    Subscription,
}

impl From<u8> for Standard {
    fn from(value: u8) -> Self {
        match value {
            0 => Standard::NonFungible,
            1 => Standard::Subscription,
            _ => panic!("invalid standard value: {value}"),
        }
    }
}

impl From<Standard> for u8 {
    fn from(value: Standard) -> Self {
        match value {
            Standard::NonFungible => 0,
            Standard::Subscription => 1,
        }
    }
}

unsafe impl Pod for Standard {}

unsafe impl Zeroable for Standard {}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, ShankType)]
pub enum State {
    #[default]
    Unlocked,
    Locked,
}

unsafe impl Pod for State {}

unsafe impl Zeroable for State {}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Pod, Zeroable)]
pub struct NullablePubkey(Pubkey);

impl NullablePubkey {
    pub fn new(pubkey: Pubkey) -> Self {
        Self(pubkey)
    }
}

impl Deref for NullablePubkey {
    type Target = Pubkey;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for NullablePubkey {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Nullable for NullablePubkey {
    fn is_some(&self) -> bool {
        self.0 != Pubkey::default()
    }

    fn is_none(&self) -> bool {
        self.0 == Pubkey::default()
    }
}
