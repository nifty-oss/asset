use bytemuck::{Pod, Zeroable};
use shank::ShankType;
use solana_program::pubkey::Pubkey;

#[repr(C)]
#[derive(Clone, Copy, Default, Pod, Zeroable, ShankType)]
pub struct Delegate {
    pub role: DelegateRole,
    pub address: Pubkey,
}

#[repr(u8)]
#[derive(Clone, Copy, Debug, Default, PartialEq, ShankType)]
pub enum DelegateRole {
    #[default]
    None,
    Authority,
}

unsafe impl Pod for DelegateRole {}

unsafe impl Zeroable for DelegateRole {}
