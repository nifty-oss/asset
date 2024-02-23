use crate::constraints::{Constraint, FromBytes};

use super::{ExtensionData, ExtensionType};

pub struct Royalties<'a> {
    pub basis_points: u64,
    pub constraint: Constraint<'a>,
}

impl<'a> ExtensionData<'a> for Royalties<'a> {
    const TYPE: ExtensionType = ExtensionType::Royalties;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let basis_points = u64::from_le_bytes(bytes[0..8].try_into().unwrap());
        let constraint = Constraint::from_bytes(&bytes[8..]);

        Self {
            basis_points,
            constraint,
        }
    }

    fn length(&self) -> usize {
        std::mem::size_of::<u64>() + self.constraint.size()
    }
}
