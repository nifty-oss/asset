mod and;
mod not;
mod or;
mod owned_by;
mod pubkey_match;

pub use and::*;
pub use not::*;
pub use or::*;
pub use owned_by::*;
pub use pubkey_match::*;

use std::fmt::{self, Debug};

use bytemuck::{Pod, Zeroable};
use podded::ZeroCopy;
use solana_program::{account_info::AccountInfo, program_error::ProgramError};

/// The result of a constraint evaluation.
pub type AssertionResult = Result<Assertion, ProgramError>;

/// The context of a constraint evaluation.
pub struct Context<'a, 'b> {
    /// The asset participating in the action.
    pub asset: &'b AccountInfo<'a>,

    /// The account authorizing the action.
    pub authority: &'b AccountInfo<'a>,

    /// The recipient account when the action is a transfer.
    pub recipient: Option<&'b AccountInfo<'a>>,
}

#[repr(u64)]
/// Account types involved in a constraint evaluation.
#[derive(Debug, Clone, Copy)]
pub enum Account {
    Asset,
    Authority,
    Recipient,
}

impl From<&str> for Account {
    fn from(value: &str) -> Self {
        match value {
            "asset" => Account::Asset,
            "authority" => Account::Authority,
            "recipient" => Account::Recipient,
            _ => panic!("invalid target value: {value}"),
        }
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let account_str = match self {
            Account::Asset => "asset",
            Account::Authority => "authority",
            Account::Recipient => "recipient",
        };
        write!(f, "{}", account_str)
    }
}

unsafe impl Pod for Account {}

unsafe impl Zeroable for Account {}

impl ZeroCopy<'_, Account> for Account {}

/// Helper macro to "extract" the account from a `Context` given its name.
#[macro_export]
macro_rules! get_account {
    ( $target:expr, $context:tt ) => {
        match (*$target).into() {
            $crate::constraints::Account::Asset => $context.asset,
            $crate::constraints::Account::Authority => $context.authority,
            $crate::constraints::Account::Recipient => $context
                .recipient
                .ok_or(solana_program::program_error::ProgramError::NotEnoughAccountKeys)?,
        }
    };
}

/// The outcome of a constraint evaluation.
///
/// This is used to determine whether a constraint is satisfied or not. Note that a`Failure` means
/// that the constraint was evaluated, but the result of the evaluation was not successful. Any error
/// that prevent the constraint from being evaluated should be propagated as a `ProgramError`.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Assertion {
    Pass,
    Failure,
}

pub trait Assertable {
    fn assert(&self, context: &Context) -> AssertionResult;
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OperatorType {
    And,
    Not,
    Or,
    OwnedBy,
    PubkeyMatch,
}

impl From<u32> for OperatorType {
    fn from(value: u32) -> Self {
        match value {
            0 => OperatorType::And,
            1 => OperatorType::Not,
            2 => OperatorType::Or,
            3 => OperatorType::OwnedBy,
            4 => OperatorType::PubkeyMatch,
            _ => panic!("invalid operator type: {value}"),
        }
    }
}

impl From<OperatorType> for u32 {
    fn from(value: OperatorType) -> Self {
        match value {
            OperatorType::And => 0,
            OperatorType::Not => 1,
            OperatorType::Or => 2,
            OperatorType::OwnedBy => 3,
            OperatorType::PubkeyMatch => 4,
        }
    }
}

/// An operator defines the header information for a constraint.
#[repr(C)]
#[derive(Debug, Clone, Copy, Pod, Zeroable)]
pub struct Operator {
    /// Header data.
    ///   0. operator type
    ///   1. size (in bytes) of the constraint data
    data: [u32; 2],
}

impl Operator {
    pub fn operator_type(&self) -> OperatorType {
        self.data[0].into()
    }

    pub fn set_operator_type(&mut self, operator_type: OperatorType) {
        self.data[0] = operator_type.into();
    }

    pub fn size(&self) -> u32 {
        self.data[1]
    }

    pub fn set_size(&mut self, length: u32) {
        self.data[1] = length;
    }
}

impl ZeroCopy<'_, Operator> for Operator {}

/// Macro to automate the code required to deserialize a constraint from a byte array.
#[macro_export]
macro_rules! assertable_from_bytes {
    ( $operator_type:ident, $slice:expr, $( $available:ident ),+ $(,)? ) => {
        match $operator_type {
            $(
                $crate::constraints::OperatorType::$available => {
                    Box::new($available::from_bytes($slice)) as Box<dyn Assertable>
                }
            )+
        }
    };
}

pub trait FromBytes<'a>: Sized {
    fn from_bytes(bytes: &'a [u8]) -> Self;
}

pub struct Constraint<'a> {
    pub operator: &'a Operator,

    pub assertable: Box<dyn Assertable + 'a>,
}

impl<'a> Constraint<'a> {
    pub fn size(&self) -> usize {
        std::mem::size_of::<Operator>() + self.operator.size() as usize
    }
}

impl<'a> FromBytes<'a> for Constraint<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        let (operator, assertable) = bytes.split_at(std::mem::size_of::<Operator>());
        let operator = Operator::load(operator);

        let operator_type = operator.operator_type();
        let length = operator.size() as usize;

        let assertable = assertable_from_bytes!(
            operator_type,
            &assertable[..length],
            And,
            Not,
            Or,
            OwnedBy,
            PubkeyMatch
        );

        Self {
            operator,
            assertable,
        }
    }
}

impl Assertable for Constraint<'_> {
    fn assert(&self, context: &Context) -> AssertionResult {
        self.assertable.assert(context)
    }
}

/// Trait for building a constraint.
///
/// The `ConstraintBuilder` encapsulates the logic for building a constraint by allocating the
/// necessary memory and writing the constraint data to a buffer. The `build` method can then
/// be used to retrieve the data buffer.
pub trait ConstraintBuilder {
    fn build(&mut self) -> Vec<u8>;
}
