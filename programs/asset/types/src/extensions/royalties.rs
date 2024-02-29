use crate::constraints::{Constraint, FromBytes};

use super::{ExtensionData, ExtensionType};

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
