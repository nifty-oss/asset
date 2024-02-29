use crate::constraints::{Constraint, FromBytes};

use super::{ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

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

impl Lifecycle for Royalties<'_> {}

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

impl Lifecycle for RoyaltiesMut<'_> {}

#[derive(Default)]
pub struct RoyaltiesBuilder(Vec<u8>);

impl RoyaltiesBuilder {
    pub fn set(&mut self, basis_points: u64, constraint: Constraint) {
        // setting the data replaces any existing data
        self.0.clear();

        self.0.extend_from_slice(&basis_points.to_le_bytes());
        self.0.extend_from_slice(&constraint.as_bytes());
    }

    pub fn build(&self) -> Royalties {
        Royalties::from_bytes(&self.0)
    }

    pub fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}
