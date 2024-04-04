use solana_program::pubkey::Pubkey;
use std::{fmt::Debug, ops::Deref};

use crate::error::Error;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to define the proxy data of a proxied asset.
///
/// Assets with a `Proxied` standard is controlled by the proxy
/// program specified in this extension.
///
/// This extension can only be used in `Proxied` asset accounts.
pub struct Proxy<'a> {
    /// The proxy program.
    pub program: &'a Pubkey,

    /// The seeds for the PDA derivation.
    pub seeds: &'a [u8; 32],

    /// The bump for the PDA derivation.
    pub bump: &'a u8,

    /// Authority of the proxy extension.
    pub authority: &'a Pubkey,
}

impl<'a> ExtensionData<'a> for Proxy<'a> {
    const TYPE: ExtensionType = ExtensionType::Proxy;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let (program, remainder) = bytes.split_at(std::mem::size_of::<Pubkey>());
        let program = bytemuck::from_bytes(program);

        let (seeds, remainder) = remainder.split_at(std::mem::size_of::<[u8; 32]>());
        let seeds = bytemuck::from_bytes(seeds);

        let (bump, authority) = remainder.split_at(std::mem::size_of::<u8>());
        let bump = bytemuck::from_bytes(bump);
        let authority = bytemuck::from_bytes(authority);

        Self {
            program,
            seeds,
            bump,
            authority,
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
            .field("authority", &self.authority)
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

    /// Authority of the proxy extension.
    pub authority: &'a mut Pubkey,
}

impl<'a> ExtensionDataMut<'a> for ProxyMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Manager;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let (program, remainder) = bytes.split_at_mut(std::mem::size_of::<Pubkey>());
        let program = bytemuck::from_bytes_mut(program);

        let (seeds, remainder) = remainder.split_at_mut(std::mem::size_of::<[u8; 32]>());
        let seeds = bytemuck::from_bytes_mut(seeds);

        let (bump, authority) = remainder.split_at_mut(std::mem::size_of::<u8>());
        let bump = bytemuck::from_bytes_mut(bump);
        let authority = bytemuck::from_bytes_mut(authority);

        Self {
            program,
            seeds,
            bump,
            authority,
        }
    }
}

impl Lifecycle for ProxyMut<'_> {
    fn on_update(&mut self, other: &mut Self) -> Result<(), Error> {
        if self.program != other.program || self.seeds != other.seeds || self.bump != other.bump {
            Err(Error::CannotModifyDerivationData)
        } else {
            Ok(())
        }
    }
}

#[derive(Default)]
pub struct ProxyBuilder(Vec<u8>);

impl ProxyBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn with_buffer(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }

    pub fn set(
        &mut self,
        program: &Pubkey,
        seeds: &[u8; 32],
        bump: u8,
        authority: &Pubkey,
    ) -> &mut Self {
        // setting the data replaces any existing data
        self.0.clear();
        self.0.extend_from_slice(program.as_ref());
        self.0.extend_from_slice(seeds);
        self.0.push(bump);
        self.0.extend_from_slice(authority.as_ref());

        self
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
    use solana_program::{pubkey::Pubkey, system_program};

    #[test]
    fn test_set() {
        // default delegate address
        let mut builder = ProxyBuilder::default();
        builder.set(&system_program::ID, &[1u8; 32], 254, &Pubkey::default());
        let proxy = Proxy::from_bytes(&builder);

        assert_eq!(proxy.program, &system_program::ID);
        assert_eq!(proxy.seeds, &[1u8; 32]);
        assert_eq!(proxy.bump, &254);
        assert_eq!(proxy.authority, &Pubkey::default());
    }
}
