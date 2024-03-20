use solana_program::pubkey::Pubkey;
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to define the subscription authority.
///
/// Assets with a `Subscription` standard can be controlled by the authority
/// specified in this extension.
///
/// This extension can only be used in `Subscription` asset accounts.
pub struct Subscription<'a> {
    /// The number of assets in the group.
    pub authority: &'a Pubkey,
}

impl<'a> ExtensionData<'a> for Subscription<'a> {
    const TYPE: ExtensionType = ExtensionType::Subscription;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let authority = bytemuck::from_bytes(bytes);
        Self { authority }
    }

    fn length(&self) -> usize {
        std::mem::size_of::<Pubkey>()
    }
}

impl Debug for Subscription<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Subscription")
            .field("authority", &self.authority.to_string())
            .finish()
    }
}

pub struct SubscriptionMut<'a> {
    /// The number of assets in the group.
    pub authority: &'a mut Pubkey,
}

impl<'a> ExtensionDataMut<'a> for SubscriptionMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Subscription;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let authority = bytemuck::from_bytes_mut(bytes);
        Self { authority }
    }
}

impl Lifecycle for SubscriptionMut<'_> {}

#[derive(Default)]
pub struct SubscriptionBuilder(Vec<u8>);

impl SubscriptionBuilder {
    pub fn set(&mut self, authority: &Pubkey) {
        // setting the data replaces any existing data
        self.0.clear();
        self.0.extend_from_slice(authority.as_ref());
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
    use solana_program::pubkey::Pubkey;

    use crate::extensions::{ExtensionData, Subscription, SubscriptionBuilder};

    #[test]
    fn test_set() {
        let mut builder = SubscriptionBuilder::default();
        builder.set(&Pubkey::default());
        let subscription = Subscription::from_bytes(&builder);

        assert_eq!(*subscription.authority, Pubkey::default());
    }
}
