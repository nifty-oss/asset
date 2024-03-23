use bytemuck::bytes_of;
use podded::ZeroCopy;
use std::{fmt::Debug, ops::Deref};

use crate::state::Delegate;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to define the delegate of a managed asset.
///
/// Assets with a `Managed` standard can be controlled by the delegate
/// specified in this extension.
///
/// This extension can only be used in `Managed` asset accounts.
pub struct Manager<'a> {
    /// The delegate address.
    pub delegate: &'a Delegate,
}

impl<'a> ExtensionData<'a> for Manager<'a> {
    const TYPE: ExtensionType = ExtensionType::Manager;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let delegate = Delegate::load(bytes);
        Self { delegate }
    }

    fn length(&self) -> usize {
        std::mem::size_of::<Delegate>()
    }
}

impl Debug for Manager<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Manager")
            .field("delegate", &self.delegate.address)
            .finish()
    }
}

pub struct ManagerMut<'a> {
    /// The delegate address.
    pub delegate: &'a mut Delegate,
}

impl<'a> ExtensionDataMut<'a> for ManagerMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Manager;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let delegate = Delegate::load_mut(bytes);
        Self { delegate }
    }
}

impl Lifecycle for ManagerMut<'_> {}

#[derive(Default)]
pub struct ManagerBuilder(Vec<u8>);

impl ManagerBuilder {
    pub fn set(&mut self, delegate: &Delegate) {
        // setting the data replaces any existing data
        self.0.clear();
        self.0.extend_from_slice(bytes_of(delegate));
    }

    pub fn build(&self) -> Manager {
        Manager::from_bytes(&self.0)
    }

    pub fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl<'a> ExtensionBuilder<'a, Manager<'a>> for ManagerBuilder {
    fn build(&'a self) -> Manager<'a> {
        Manager::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for ManagerBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        extensions::{ExtensionData, Manager, ManagerBuilder},
        state::{Delegate, NullablePubkey},
    };
    use podded::pod::Nullable;
    use solana_program::sysvar;

    #[test]
    fn test_set() {
        // default delegate address
        let mut builder = ManagerBuilder::default();
        builder.set(&Delegate::default());
        let manager = Manager::from_bytes(&builder);

        assert!(manager.delegate.is_none());
        assert_eq!(manager.delegate.address, Delegate::default().address);

        // "custom" delegate address
        let mut builder = ManagerBuilder::default();
        builder.set(&Delegate {
            address: NullablePubkey::new(sysvar::ID),
            roles: Delegate::ALL_ROLES_MASK,
        });
        let manager = Manager::from_bytes(&builder);

        assert!(manager.delegate.is_some());
        assert_eq!(manager.delegate.roles, Delegate::ALL_ROLES_MASK);
        assert_eq!(manager.delegate.address, NullablePubkey::new(sysvar::ID));
    }
}
