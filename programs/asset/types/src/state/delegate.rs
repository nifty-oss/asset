use borsh::{BorshDeserialize, BorshSerialize};
use bytemuck::{Pod, Zeroable};
use podded::pod::PodOption;

use super::{Nullable, NullablePubkey};

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable)]
pub struct Delegate {
    pub address: NullablePubkey,
    pub roles: u8,
}

impl Delegate {
    pub const ALL_ROLES_MASK: u8 = 0b111;

    pub fn is_active(&self, role: DelegateRole) -> bool {
        self.roles & role.mask() > 0
    }

    pub fn enable(&mut self, role: DelegateRole) {
        self.roles |= role.mask();
    }

    pub fn disable(&mut self, role: DelegateRole) {
        self.roles &= !role.mask();
    }

    pub fn has_active_roles(&self) -> bool {
        self.roles > 0
    }

    pub fn decode_roles(roles: u8) -> Vec<DelegateRole> {
        let mut result = Vec::new();
        if roles == 0 {
            result.push(DelegateRole::None);
            return result;
        }
        for i in 0..3 {
            if roles & (0b1u8 << i) > 0 {
                result.push(DelegateRole::from(i + 1));
            }
        }
        result
    }
}

impl Nullable for Delegate {
    fn is_some(&self) -> bool {
        self.address.is_some()
    }

    fn is_none(&self) -> bool {
        self.address.is_none()
    }
}

impl From<Delegate> for PodOption<Delegate> {
    fn from(value: Delegate) -> Self {
        PodOption::new(value)
    }
}

#[repr(u8)]
#[derive(BorshSerialize, BorshDeserialize, Clone, Copy, Debug, Default, PartialEq)]
pub enum DelegateRole {
    #[default]
    None,
    Transfer,
    Lock,
    Burn,
}

impl DelegateRole {
    pub fn mask(&self) -> u8 {
        match *self {
            DelegateRole::None => 0,
            _ => 0b1u8 << ((*self as u8) - 1),
        }
    }
}

impl From<u8> for DelegateRole {
    fn from(value: u8) -> Self {
        match value {
            0 => DelegateRole::None,
            1 => DelegateRole::Transfer,
            2 => DelegateRole::Lock,
            3 => DelegateRole::Burn,
            _ => panic!("invalid delegate role value: {value}"),
        }
    }
}

impl From<DelegateRole> for u8 {
    fn from(value: DelegateRole) -> Self {
        match value {
            DelegateRole::None => 0,
            DelegateRole::Transfer => 1,
            DelegateRole::Lock => 2,
            DelegateRole::Burn => 3,
        }
    }
}

unsafe impl Pod for DelegateRole {}

unsafe impl Zeroable for DelegateRole {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_roles() {
        // Test case 1: roles = 0b101 (Transfer and Burn)
        let roles = 0b101;
        let expected_result = vec![DelegateRole::Transfer, DelegateRole::Burn];
        assert_eq!(Delegate::decode_roles(roles), expected_result);

        // Test case 2: roles = 0b010 (Lock)
        let roles = 0b010;
        let expected_result = vec![DelegateRole::Lock];
        assert_eq!(Delegate::decode_roles(roles), expected_result);

        // Test case 3: roles = 0b000 (None)
        let roles = 0b000;
        let expected_result = vec![DelegateRole::None];
        assert_eq!(Delegate::decode_roles(roles), expected_result);

        // Test case 4: roles = 0b111 (All roles)
        let roles = 0b111;
        let expected_result = vec![
            DelegateRole::Transfer,
            DelegateRole::Lock,
            DelegateRole::Burn,
        ];
        assert_eq!(Delegate::decode_roles(roles), expected_result);
    }
}
