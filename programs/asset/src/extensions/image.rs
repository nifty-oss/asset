use super::{Extension, ExtensionType};

/// Extension to add a list of creators.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Image<'a> {
    pub data: &'a [u8],
}

impl<'a> Extension<'a> for Image<'a> {
    const TYPE: ExtensionType = ExtensionType::Image;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { data: bytes }
    }

    fn length(&self) -> usize {
        self.data.len()
    }
}
