mod asset;
mod delegate;

pub use asset::*;
pub use delegate::*;

use bytemuck::{Pod, Zeroable};
use shank::ShankType;

#[derive(Clone, Copy, Debug, Default, PartialEq, ShankType)]
pub enum Discriminator {
    #[default]
    Uninitialized,
    Asset,
}

impl From<u8> for Discriminator {
    fn from(value: u8) -> Self {
        match value {
            0 => Discriminator::Uninitialized,
            1 => Discriminator::Asset,
            _ => panic!("invalid key value: {value}"),
        }
    }
}

impl From<Discriminator> for u8 {
    fn from(value: Discriminator) -> Self {
        match value {
            Discriminator::Uninitialized => 0,
            Discriminator::Asset => 1,
        }
    }
}

unsafe impl Pod for Discriminator {}

unsafe impl Zeroable for Discriminator {}
