use std::ops::Deref;

use podded::ZeroCopy;

use crate::constraints::{
    Assertable, Assertion, AssertionResult, Constraint, ConstraintBuilder, Context, FromBytes,
    Operator, OperatorType,
};

/// A list of constraints where one of them must succeed for the assertion to pass.
pub struct Or<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> FromBytes<'a> for Or<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        let mut constraints = Vec::new();
        let mut offset = 0;

        while offset < bytes.len() {
            let constraint = Constraint::from_bytes(&bytes[offset..]);
            offset += constraint.size();
            constraints.push(constraint);
        }

        Self { constraints }
    }
}

impl Assertable for Or<'_> {
    fn assert(&self, context: &Context) -> AssertionResult {
        let mut last = Assertion::Failure;

        for constraint in &self.constraints {
            let result = constraint.assert(context)?;
            match result {
                Assertion::Pass => return Ok(result),
                Assertion::Failure => last = result,
            }
        }

        Ok(last)
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for constraint in &self.constraints {
            bytes.extend_from_slice(constraint.as_bytes().as_ref());
        }
        bytes
    }
}

/// Builder for an `Or` constraint.
pub struct OrBuilder(Vec<u8>);

impl Default for OrBuilder {
    fn default() -> Self {
        Self(vec![0u8; std::mem::size_of::<Operator>()])
    }
}

impl OrBuilder {
    /// Add a new constraint.
    pub fn add(&mut self, constraint: &mut dyn ConstraintBuilder) {
        self.0.extend_from_slice(constraint.build().as_ref());
    }
}

impl ConstraintBuilder for OrBuilder {
    fn build(&mut self) -> Vec<u8> {
        let length = self.0.len() - std::mem::size_of::<Operator>();
        let operator = Operator::load_mut(&mut self.0);
        operator.set_operator_type(OperatorType::Or);
        operator.set_size(length as u32);

        std::mem::take(&mut self.0)
    }
}

impl Deref for OrBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::constraints::{
        Account, Constraint, ConstraintBuilder, FromBytes, OperatorType, OrBuilder, OwnedByBuilder,
    };
    use solana_program::pubkey::Pubkey;

    #[test]
    pub fn test_build() {
        let mut constraint1 = OwnedByBuilder::default();
        constraint1.set(Account::Authority, &[Pubkey::default()]);
        let mut constraint2 = OwnedByBuilder::default();
        constraint2.set(Account::Recipient, &[Pubkey::default()]);

        let mut builder = OrBuilder::default();

        builder.add(&mut constraint1);
        builder.add(&mut constraint2);
        let bytes = builder.build();

        let constraint = Constraint::from_bytes(&bytes);
        assert_eq!(constraint.operator.operator_type(), OperatorType::Or);
    }
}
