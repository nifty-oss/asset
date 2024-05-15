use std::ops::Deref;

use crate::constraints::{Constraint, ConstraintBuilder, FromBytes};
use crate::error::Error;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

pub struct Royalties<'a> {
    pub basis_points: &'a u64,
    pub constraint: Constraint<'a>,
}

impl<'a> ExtensionData<'a> for Royalties<'a> {
    const TYPE: ExtensionType = ExtensionType::Royalties;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let (basis_points, constraint) = bytes.split_at(std::mem::size_of::<u64>());

        let basis_points = bytemuck::from_bytes(basis_points);
        let constraint = Constraint::from_bytes(constraint);

        Self {
            basis_points,
            constraint,
        }
    }

    fn length(&self) -> usize {
        std::mem::size_of::<u64>() + self.constraint.size()
    }
}

pub struct RoyaltiesMut<'a> {
    pub basis_points: &'a mut u64,
    pub constraint: Constraint<'a>,
}

impl<'a> ExtensionDataMut<'a> for RoyaltiesMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Royalties;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let (basis_points, constraint) = bytes.split_at_mut(std::mem::size_of::<u64>());

        let basis_points = bytemuck::from_bytes_mut(basis_points);
        let constraint = Constraint::from_bytes(constraint);

        Self {
            basis_points,
            constraint,
        }
    }
}

impl Lifecycle for RoyaltiesMut<'_> {
    fn on_create(
        &mut self,
        _authority: Option<&solana_program::pubkey::Pubkey>,
    ) -> Result<(), Error> {
        if *self.basis_points > 10000 {
            return Err(Error::InvalidRoyaltyBasisPoints);
        }

        Ok(())
    }

    fn on_update(
        &mut self,
        other: &mut Self,
        _authority: Option<&solana_program::pubkey::Pubkey>,
    ) -> Result<(), Error> {
        if *other.basis_points > 10000 {
            return Err(Error::InvalidRoyaltyBasisPoints);
        }

        Ok(())
    }
}

#[derive(Default)]
pub struct RoyaltiesBuilder(Vec<u8>);

impl RoyaltiesBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn with_buffer(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }

    pub fn set(&mut self, basis_points: u64, constraint: &mut dyn ConstraintBuilder) -> &mut Self {
        // setting the data replaces any existing data
        self.0.clear();

        self.0.extend_from_slice(&basis_points.to_le_bytes());
        self.0.extend_from_slice(&constraint.build());

        self
    }
}

impl<'a> ExtensionBuilder<'a, Royalties<'a>> for RoyaltiesBuilder {
    fn build(&'a self) -> Royalties<'a> {
        Royalties::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for RoyaltiesBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
