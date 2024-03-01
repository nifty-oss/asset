use std::ops::Deref;

use crate::constraints::{
    Assertable, Assertion, AssertionResult, Constraint, ConstraintBuilder, Context, FromBytes,
    Operator, OperatorType,
};

/// A list of constraints that must all succeed for the assertion to pass.
pub struct And<'a> {
    pub constraints: Vec<Constraint<'a>>,
}

impl<'a> FromBytes<'a> for And<'a> {
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

impl Assertable for And<'_> {
    fn assert(&self, context: &Context) -> AssertionResult {
        for constraint in &self.constraints {
            let result = constraint.assert(context)?;
            if result == Assertion::Failure {
                return Ok(result);
            }
        }

        Ok(Assertion::Pass)
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        for constraint in &self.constraints {
            bytes.extend_from_slice(constraint.as_bytes().as_ref());
        }
        bytes
    }
}

/// Builder for an `And` constraint.
pub struct AndBuilder(Vec<u8>);

impl Default for AndBuilder {
    fn default() -> Self {
        Self(vec![0u8; std::mem::size_of::<Operator>()])
    }
}

impl AndBuilder {
    /// Add a new constraint.
    pub fn add(&mut self, constraint: &mut dyn ConstraintBuilder) {
        self.0.extend_from_slice(constraint.build().as_ref());
    }
}

impl ConstraintBuilder for AndBuilder {
    fn build(&mut self) -> Vec<u8> {
        if self.0.is_empty() {
            self.0.resize(std::mem::size_of::<Operator>(), 0);
        }

        let length = self.0.len() - std::mem::size_of::<Operator>();

        // manual byte wrangling because bytemuck doesn't work with newly
        // allocated Vec in BPF.
        self.0[0..4].copy_from_slice(&u32::to_le_bytes(OperatorType::And as u32));
        self.0[4..8].copy_from_slice(&u32::to_le_bytes(length as u32));

        std::mem::take(&mut self.0)
    }
}

impl Deref for AndBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::constraints::{
        Account, AndBuilder, Constraint, ConstraintBuilder, FromBytes, OperatorType, OwnedByBuilder,
    };
    use solana_program::pubkey::Pubkey;

    #[test]
    pub fn test_build() {
        let mut constraint1 = OwnedByBuilder::default();
        constraint1.set(Account::Authority, &[Pubkey::default()]);
        let mut constraint2 = OwnedByBuilder::default();
        constraint2.set(Account::Recipient, &[Pubkey::default()]);

        let mut builder = AndBuilder::default();

        builder.add(&mut constraint1);
        builder.add(&mut constraint2);
        let bytes = builder.build();

        let constraint = Constraint::from_bytes(&bytes);
        assert_eq!(constraint.operator.operator_type(), OperatorType::And);
    }
}
