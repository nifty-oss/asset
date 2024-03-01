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

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(self.constraint.size());
        bytes.extend_from_slice(bytemuck::bytes_of(self.constraint.operator));
        bytes.extend_from_slice(self.constraint.as_bytes().as_ref());
        bytes
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
        if self.0.is_empty() {
            self.0.resize(std::mem::size_of::<Operator>(), 0);
        }

        let length = self.0.len() - std::mem::size_of::<Operator>();

        // manual byte wrangling because bytemuck doesn't work with newly
        // allocated Vec in BPF.
        self.0[0..4].copy_from_slice(&u32::to_le_bytes(OperatorType::Not as u32));
        self.0[4..8].copy_from_slice(&u32::to_le_bytes(length as u32));

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
