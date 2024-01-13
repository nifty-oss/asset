use podded::types::U8PrefixStr;

use super::{ExtensionData, ExtensionType};

/// Extension to add a list of creators.
#[repr(C)]
pub struct Blob<'a> {
    /// The content type of the blob.
    pub content_type: U8PrefixStr<'a>,

    /// The raw data of the extension.
    pub data: &'a [u8],
}

impl<'a> ExtensionData<'a> for Blob<'a> {
    const TYPE: ExtensionType = ExtensionType::Blob;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let content_type = U8PrefixStr::from_bytes(bytes);
        let data = &bytes[content_type.size()..];
        Self { content_type, data }
    }

    fn length(&self) -> usize {
        self.content_type.size() + self.data.len()
    }
}
