use borsh::{BorshDeserialize, BorshSerialize};
use nifty_asset_types::state::{Delegate, DelegateRole};

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DelegateRoles(u8);

impl DelegateRoles {
    pub fn to_vec(&self) -> Vec<DelegateRole> {
        Delegate::decode_roles(self.0)
    }
}
