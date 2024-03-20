use thiserror::Error;

#[derive(Error, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// 0 - Invalid creators total share
    #[error("Total creators share is invalid (expected {0}, got {1})")]
    InvalidCreatorsTotalShare(u8, u8),

    /// 1 - Cannot unverify creator
    #[error("Cannot unverify creator")]
    CannotUnverifyCreator,

    /// 2 - Cannot remove verified creator
    #[error("Cannot remove verified creator")]
    CannotRemoveVerifiedCreator,

    /// 3 - Cannot modify the size of a group
    #[error("Cannot modify the size of a group")]
    InvalidGroupSize,

    /// 4 - Invalid maximum group size
    #[error("Maximum group size if invalid (expected at least {0}, got {1})")]
    InvalidMaximumGroupSize(u64, u64),

    /// 5 - Invalid extension type
    #[error("Invalid extension type: {0}")]
    InvalidExtensionType(u32),
}
