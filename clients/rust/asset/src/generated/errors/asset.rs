//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! [https://github.com/metaplex-foundation/kinobi]
//!

use num_derive::FromPrimitive;
use thiserror::Error;

#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum AssetError {
    /// 0 (0x0) - Asset already initialized
    #[error("Asset already initialized")]
    AlreadyInitialized,
    /// 1 (0x1) - Invalid account length
    #[error("Invalid account length")]
    InvalidAccountLength,
    /// 2 (0x2) - Incomplete extension data
    #[error("Incomplete extension data")]
    IncompleteExtensionData,
    /// 3 (0x3) - Uninitialized account
    #[error("Uninitialized account")]
    Uninitialized,
    /// 4 (0x4) - Extension not found
    #[error("Extension not found")]
    ExtensionNotFound,
    /// 5 (0x5) - Invalid alignment
    #[error("Invalid alignment")]
    InvalidAlignment,
    /// 6 (0x6) - Invalid holder or burn delegate
    #[error("Invalid holder or burn delegate")]
    InvalidBurnAuthority,
    /// 7 (0x7) - Invalid holder or transfer delegate
    #[error("Invalid holder or transfer delegate")]
    InvalidTransferAuthority,
    /// 8 (0x8) - Delegate not found
    #[error("Delegate not found")]
    DelegateNotFound,
    /// 9 (0x9) - Delegate role not active
    #[error("Delegate role not active")]
    DelegateRoleNotActive,
    /// 10 (0xA) - Invalid delegate
    #[error("Invalid delegate")]
    InvalidDelegate,
    /// 11 (0xB) - Invalid holder
    #[error("Invalid holder")]
    InvalidHolder,
    /// 12 (0xC) - Asset is locked
    #[error("Asset is locked")]
    LockedAsset,
    /// 13 (0xD) - Invalid authority
    #[error("Invalid authority")]
    InvalidAuthority,
    /// 14 (0xE) - Immutable asset
    #[error("Immutable asset")]
    ImmutableAsset,
    /// 15 (0xF) - Soulbound assets are non-transferable
    #[error("Soulbound assets are non-transferable")]
    CannotTransferSoulbound,
    /// 16 (0x10) - Extension data invalid
    #[error("Extension data invalid")]
    ExtensionDataInvalid,
    /// 17 (0x11) - Invalid group
    #[error("Invalid group")]
    InvalidGroup,
}

impl solana_program::program_error::PrintProgramError for AssetError {
    fn print<E>(&self) {
        solana_program::msg!(&self.to_string());
    }
}
