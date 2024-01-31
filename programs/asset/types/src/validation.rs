//! Self validation for types.
//!
//! This module contains the `Validatable` trait and the `ValidationError` enum. These are used
//! to validate the "state" of a type.

/// Possible validation errors.
#[derive(Debug)]
pub enum ValidationError {
    InvalidShareTotal,
}

/// Trait for self validation.
///
/// This trait is used to validate the "state" of a type.
pub trait Validatable {
    fn validate(&self) -> Result<(), ValidationError> {
        Ok(())
    }
}
