use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::ops::Deref;

use super::{ExtensionBuilder, ExtensionData, ExtensionType};

/// Extension to add "binary large object" to an asset.
///
/// In most cases, this extension is used to add an image or document to an asset.
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

/// Builder for a `Blob` extension.
#[derive(Default)]
pub struct BlobBuilder(Vec<u8>);

impl BlobBuilder {
    /// Add a new attribute to the extension.
    pub fn set_data(&mut self, content_type: &str, data: &[u8]) {
        // setting the data replaces any existing data
        self.0.clear();

        // add the length of the content type + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; content_type.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(content_type);

        // add the data to the buffer
        self.0.extend_from_slice(data);
    }
}

impl ExtensionBuilder for BlobBuilder {
    const TYPE: ExtensionType = ExtensionType::Blob;

    fn build(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for BlobBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
