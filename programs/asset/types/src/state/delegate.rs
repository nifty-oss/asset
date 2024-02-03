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
