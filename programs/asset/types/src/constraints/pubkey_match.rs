use std::{mem::size_of, ops::Deref};

use podded::ZeroCopy;
use solana_program::pubkey::Pubkey;

use crate::{
    constraints::{
        Account, Assertable, Assertion, AssertionResult, ConstraintBuilder, Context, FromBytes,
        Operator, OperatorType,
    },
    get_account,
};

pub struct PubkeyMatch<'a> {
    /// The evaluation's field target.
    pub account: &'a Account,

    /// List of "valid" pubkeys.
    pub pubkeys: &'a [Pubkey],
}

impl<'a> FromBytes<'a> for PubkeyMatch<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        let (account, pubkeys) = bytes.split_at(size_of::<Account>());
        let account = Account::load(account);
        let pubkeys = bytemuck::cast_slice(pubkeys);
        Self { account, pubkeys }
    }
}

impl Assertable for PubkeyMatch<'_> {
    fn assert(&self, context: &Context) -> AssertionResult {
        let account = get_account!(self.account, context);

        Ok(if self.pubkeys.contains(account.key) {
            Assertion::Pass
        } else {
            Assertion::Failure
        })
    }
}

/// Builder for an `PubkeyMatch` constraint.
#[derive(Default)]
pub struct PubkeyMatchBuilder(Vec<u8>);

impl PubkeyMatchBuilder {
    /// Set the list of addresses (pubkeys).
    pub fn set(&mut self, account: Account, addresses: &[Pubkey]) {
        // clear any previous value
        self.0.resize(std::mem::size_of::<Operator>(), 0);

        let offset = self.0.len();
        self.0.append(&mut vec![0u8; size_of::<Account>()]);
        let account_ref = bytemuck::from_bytes_mut(&mut self.0[offset..]);
        *account_ref = account;

        // add the addresses to the data buffer.
        addresses.iter().for_each(|address| {
            self.0.extend_from_slice(address.as_ref());
        });
    }
}

impl ConstraintBuilder for PubkeyMatchBuilder {
    fn build(&mut self) -> Vec<u8> {
        let length = self.0.len() - std::mem::size_of::<Operator>();
        let operator = Operator::load_mut(&mut self.0);
        operator.set_operator_type(OperatorType::PubkeyMatch);
        operator.set_size(length as u32);

        std::mem::take(&mut self.0)
    }
}

impl Deref for PubkeyMatchBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::constraints::{
        Account, Constraint, ConstraintBuilder, FromBytes, OperatorType, PubkeyMatchBuilder,
    };
    use solana_program::pubkey::Pubkey;

    #[test]
    pub fn test_build() {
        let mut builder = PubkeyMatchBuilder::default();
        builder.set(Account::Asset, &[solana_program::pubkey::Pubkey::default()]);
        let bytes = builder.build();

        let constraint = Constraint::from_bytes(&bytes);
        assert_eq!(
            constraint.operator.operator_type(),
            OperatorType::PubkeyMatch
        );
        assert_eq!(
            constraint.operator.size(),
            (std::mem::size_of::<Account>() + std::mem::size_of::<Pubkey>()) as u32
        );
    }
}
