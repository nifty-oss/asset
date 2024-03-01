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

    fn as_bytes(&self) -> Vec<u8> {
        let mut bytes =
            Vec::with_capacity(size_of::<Account>() + std::mem::size_of_val(self.pubkeys));
        bytes.extend_from_slice(bytemuck::bytes_of(self.account));
        bytes.extend_from_slice(bytemuck::cast_slice(self.pubkeys));
        bytes
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

        // add the account to the data buffer.
        self.0.extend_from_slice(account.into_bytes().as_ref());

        // add the addresses to the data buffer.
        addresses.iter().for_each(|address| {
            self.0.extend_from_slice(address.as_ref());
        });
    }
}

impl ConstraintBuilder for PubkeyMatchBuilder {
    fn build(&mut self) -> Vec<u8> {
        if self.0.is_empty() {
            self.0.resize(std::mem::size_of::<Operator>(), 0);
        }

        let length = self.0.len() - std::mem::size_of::<Operator>();

        // manual byte wrangling because bytemuck doesn't work with newly
        // allocated Vec in BPF.
        self.0[0..4].copy_from_slice(&u32::to_le_bytes(OperatorType::PubkeyMatch as u32));
        self.0[4..8].copy_from_slice(&u32::to_le_bytes(length as u32));

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
