use solana_program::pubkey::Pubkey;
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to define the delegate of a managed asset.
///
/// Assets with a `Managed` standard can be controlled by the delegate
/// specified in this extension.
///
/// This extension can only be used in `Managed` asset accounts.
pub struct Proxy<'a> {
    /// The proxy program.
    pub program: &'a Pubkey,

    /// The seeds for the PDA derivation.
    pub seeds: &'a [u8; 32],

    /// The bump for the PDA derivation.
    pub bump: &'a u8,
}

impl<'a> ExtensionData<'a> for Proxy<'a> {
    const TYPE: ExtensionType = ExtensionType::Proxy;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let (program, remainder) = bytes.split_at(std::mem::size_of::<Pubkey>());
        let program = bytemuck::from_bytes(program);

        let (seeds, bump) = remainder.split_at(std::mem::size_of::<[u8; 32]>());
        let seeds = bytemuck::from_bytes(seeds);
        let bump = bytemuck::from_bytes(bump);

        Self {
            program,
            seeds,
            bump,
        }
    }

    fn length(&self) -> usize {
        std::mem::size_of::<Pubkey>() + std::mem::size_of::<[u8; 32]>()
    }
}

impl Debug for Proxy<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Proxy")
            .field("program", &self.program)
            .field("seeds", &self.seeds)
            .field("bump", &self.bump)
            .finish()
    }
}

pub struct ProxyMut<'a> {
    /// The proxy program.
    pub program: &'a mut Pubkey,

    /// The seeds for the PDA derivation.
    pub seeds: &'a mut [u8; 32],

    /// The bump for the PDA derivation.
    pub bump: &'a mut u8,
}

impl<'a> ExtensionDataMut<'a> for ProxyMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Manager;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let (program, remainder) = bytes.split_at_mut(std::mem::size_of::<Pubkey>());
        let program = bytemuck::from_bytes_mut(program);

        let (seeds, bump) = remainder.split_at_mut(std::mem::size_of::<[u8; 32]>());
        let seeds = bytemuck::from_bytes_mut(seeds);
        let bump = bytemuck::from_bytes_mut(bump);

        Self {
            program,
            seeds,
            bump,
        }
    }
}

impl Lifecycle for ProxyMut<'_> {}

#[derive(Default)]
pub struct ProxyBuilder(Vec<u8>);

impl ProxyBuilder {
    pub fn set(&mut self, program: &Pubkey, seeds: &[u8; 32], bump: u8) {
        // setting the data replaces any existing data
        self.0.clear();
        self.0.extend_from_slice(program.as_ref());
        self.0.extend_from_slice(seeds);
        self.0.push(bump);
    }

    pub fn build(&self) -> Proxy {
        Proxy::from_bytes(&self.0)
    }

    pub fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl<'a> ExtensionBuilder<'a, Proxy<'a>> for ProxyBuilder {
    fn build(&'a self) -> Proxy<'a> {
        Proxy::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for ProxyBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::extensions::{ExtensionData, Proxy, ProxyBuilder};
    use solana_program::system_program;

    #[test]
    fn test_set() {
        // default delegate address
        let mut builder = ProxyBuilder::default();
        builder.set(&system_program::ID, &[1u8; 32], 254);
        let proxy = Proxy::from_bytes(&builder);

        assert_eq!(proxy.program, &system_program::ID);
        assert_eq!(proxy.seeds, &[1u8; 32]);
        assert_eq!(proxy.bump, &254);
    }
}
