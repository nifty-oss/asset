use bytemuck::bytes_of;
use podded::ZeroCopy;
use std::{fmt::Debug, ops::Deref};

use crate::state::Delegate;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to define the subscription authority.
///
/// Assets with a `Subscription` standard can be controlled by the authority
/// specified in this extension.
///
/// This extension can only be used in `Subscription` asset accounts.
pub struct Subscription<'a> {
    /// The subscription delegate address.
    pub delegate: &'a Delegate,
}

impl<'a> ExtensionData<'a> for Subscription<'a> {
    const TYPE: ExtensionType = ExtensionType::Subscription;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let delegate = Delegate::load(bytes);
        Self { delegate }
    }

    fn length(&self) -> usize {
        std::mem::size_of::<Delegate>()
    }
}

impl Debug for Subscription<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("delegate", &self.delegate.address)
            .finish()
    }
}

pub struct SubscriptionMut<'a> {
    /// The subscription delegate address.
    pub delegate: &'a mut Delegate,
}

impl<'a> ExtensionDataMut<'a> for SubscriptionMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Subscription;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let delegate = Delegate::load_mut(bytes);
        Self { delegate }
    }
}

impl Lifecycle for SubscriptionMut<'_> {}

#[derive(Default)]
pub struct SubscriptionBuilder(Vec<u8>);

impl SubscriptionBuilder {
    pub fn set(&mut self, delegate: &Delegate) {
        // setting the data replaces any existing data
        self.0.clear();
        self.0.extend_from_slice(bytes_of(delegate));
    }

    pub fn build(&self) -> Subscription {
        Subscription::from_bytes(&self.0)
    }

    pub fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl<'a> ExtensionBuilder<'a, Subscription<'a>> for SubscriptionBuilder {
    fn build(&'a self) -> Subscription<'a> {
        Subscription::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for SubscriptionBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        extensions::{ExtensionData, Subscription, SubscriptionBuilder},
        state::{Delegate, NullablePubkey},
    };
    use podded::pod::Nullable;
    use solana_program::sysvar;

    #[test]
    fn test_set() {
        // default delegate address
        let mut builder = SubscriptionBuilder::default();
        builder.set(&Delegate::default());
        let subscription = Subscription::from_bytes(&builder);

        assert!(subscription.delegate.is_none());
        assert_eq!(subscription.delegate.address, Delegate::default().address);

        // "custom" delegate address
        let mut builder = SubscriptionBuilder::default();
        builder.set(&Delegate {
            address: NullablePubkey::new(sysvar::ID),
            roles: Delegate::ALL_ROLES_MASK,
        });
        let subscription = Subscription::from_bytes(&builder);

        assert!(subscription.delegate.is_some());
        assert_eq!(subscription.delegate.roles, Delegate::ALL_ROLES_MASK);
        assert_eq!(
            subscription.delegate.address,
            NullablePubkey::new(sysvar::ID)
        );
    }
}
