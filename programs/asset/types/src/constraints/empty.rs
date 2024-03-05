use std::{marker::PhantomData, ops::Deref, vec};

use crate::constraints::{
    Assertable, Assertion, AssertionResult, ConstraintBuilder, Context, FromBytes,
};

use super::{Operator, OperatorType};

/// A constrait that inverts the outcome of another constraint.
///
/// This is useful to define negation of existing constraints. For example, a constraint
/// that succeeds when an account is not owned by a specific program.
pub struct Empty<'a> {
    phantom: PhantomData<&'a Self>,
}

impl<'a> FromBytes<'a> for Empty<'a> {
    fn from_bytes(_bytes: &'a [u8]) -> Self {
        Self {
            phantom: PhantomData,
        }
    }
}

impl Assertable for Empty<'_> {
    fn assert(&self, _context: &Context) -> AssertionResult {
        Ok(Assertion::Pass)
    }

    fn as_bytes(&self) -> Vec<u8> {
        vec![]
    }
}

/// Builder for an `OwnedBy` constraint.
#[derive(Default)]
pub struct EmptyBuilder(Vec<u8>);

impl ConstraintBuilder for EmptyBuilder {
    fn build(&mut self) -> Vec<u8> {
        if self.0.is_empty() {
            self.0.resize(std::mem::size_of::<Operator>(), 0);
        }

        let length = self.0.len() - std::mem::size_of::<Operator>();

        // manual byte wrangling because bytemuck doesn't work with newly
        // allocated Vec in BPF.
        self.0[0..4].copy_from_slice(&u32::to_le_bytes(OperatorType::Empty as u32));
        self.0[4..8].copy_from_slice(&u32::to_le_bytes(length as u32));

        std::mem::take(&mut self.0)
    }
}

impl Deref for EmptyBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::constraints::{
        Constraint, ConstraintBuilder, EmptyBuilder, FromBytes, OperatorType,
    };

    #[test]
    pub fn test_build() {
        let mut builder = EmptyBuilder::default();
        let bytes = builder.build();

        let constraint = Constraint::from_bytes(&bytes);
        assert_eq!(constraint.operator.operator_type(), OperatorType::Empty);
    }
}
