use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::ops::Deref;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

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

pub struct BlobMut<'a> {
    /// The content type of the blob.
    pub content_type: U8PrefixStrMut<'a>,

    /// The raw data of the extension.
    pub data: &'a mut [u8],
}

impl<'a> ExtensionDataMut<'a> for BlobMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Blob;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let content_type = U8PrefixStr::from_bytes(bytes);
        let size = content_type.size();

        let (content_type, data) = bytes.split_at_mut(size);
        let content_type = U8PrefixStrMut::from_bytes_mut(content_type);

        Self { content_type, data }
    }
}

impl Lifecycle for BlobMut<'_> {}

/// Builder for a `Blob` extension.
#[derive(Default)]
pub struct BlobBuilder(Vec<u8>);

impl BlobBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn with_buffer(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }

    /// Add a new attribute to the extension.
    pub fn set_data(&mut self, content_type: &str, data: &[u8]) -> &mut Self {
        // setting the data replaces any existing data
        self.0.clear();

        // add the length of the content type + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; content_type.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(content_type);

        // add the data to the buffer
        self.0.extend_from_slice(data);

        self
    }
}

impl<'a> ExtensionBuilder<'a, Blob<'a>> for BlobBuilder {
    fn build(&'a self) -> Blob<'a> {
        Blob::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for BlobBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
