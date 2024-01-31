use podded::ZeroCopy;
use std::ops::Deref;

use crate::constraints::{
    Assertable, Assertion, AssertionResult, Constraint, ConstraintBuilder, Context, FromBytes,
    Operator, OperatorType,
};

/// A constrait that inverts the outcome of another constraint.
///
/// This is useful to define negation of existing constraints. For example, a constraint
/// that succeeds when an account is not owned by a specific program.
pub struct Not<'a> {
    pub constraint: Constraint<'a>,
}

impl<'a> FromBytes<'a> for Not<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        let constraint = Constraint::from_bytes(bytes);
        Self { constraint }
    }
}

impl Assertable for Not<'_> {
    fn assert(&self, context: &Context) -> AssertionResult {
        match self.constraint.assert(context)? {
            Assertion::Pass => Ok(Assertion::Failure),
            Assertion::Failure => Ok(Assertion::Pass),
        }
    }
}

/// Builder for an `OwnedBy` constraint.
#[derive(Default)]
pub struct NotBuilder(Vec<u8>);

impl NotBuilder {
    /// Set the constraint to negate.
    pub fn set(&mut self, constraint: &mut dyn ConstraintBuilder) {
        // clears any previous value
        self.0.resize(std::mem::size_of::<Operator>(), 0);
        self.0.extend_from_slice(constraint.build().as_ref());
    }
}

impl ConstraintBuilder for NotBuilder {
    fn build(&mut self) -> Vec<u8> {
        let length = self.0.len() - std::mem::size_of::<Operator>();
        let operator = Operator::load_mut(&mut self.0);
        operator.set_operator_type(OperatorType::Not);
        operator.set_size(length as u32);

        std::mem::take(&mut self.0)
    }
}

impl Deref for NotBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::constraints::{
        Account, Constraint, ConstraintBuilder, FromBytes, NotBuilder, OperatorType, OwnedByBuilder,
    };
    use solana_program::pubkey::Pubkey;

    #[test]
    pub fn test_build() {
        let mut constraint = OwnedByBuilder::default();
        constraint.set(Account::Authority, &[Pubkey::default()]);

        let mut builder = NotBuilder::default();
        builder.set(&mut constraint);
        let bytes = builder.build();

        let constraint = Constraint::from_bytes(&bytes);
        assert_eq!(constraint.operator.operator_type(), OperatorType::Not);
    }
}
