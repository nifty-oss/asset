//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use crate::generated::types::Discriminator;
use crate::generated::types::State;
use crate::generated::types::Standard;
use solana_program::pubkey::Pubkey;
use crate::generated::types::Delegate;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AssetAccountData {
pub discriminator: Discriminator,
pub state: State,
pub standard: Standard,
pub mutable: bool,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub owner: Pubkey,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub group: Pubkey,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::DisplayFromStr>"))]
pub authority: Pubkey,
pub delegate: Delegate,
#[cfg_attr(feature = "serde", serde(with = "serde_with::As::<serde_with::Bytes>"))]
pub name: [u8; 35],
}


