use std::ops::Deref;

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to add binary data to an asset.
#[repr(C)]
pub struct Bucket<'a> {
    /// The raw data of the extension.
    pub data: &'a [u8],
}

impl<'a> ExtensionData<'a> for Bucket<'a> {
    const TYPE: ExtensionType = ExtensionType::Bucket;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        Self { data: bytes }
    }

    fn length(&self) -> usize {
        self.data.len()
    }
}

pub struct BucketMut<'a> {
    /// The raw data of the extension.
    pub data: &'a mut [u8],
}

impl<'a> ExtensionDataMut<'a> for BucketMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Bucket;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        Self { data: bytes }
    }
}

impl Lifecycle for BucketMut<'_> {}

/// Builder for a `Bucket` extension.
#[derive(Default)]
pub struct BucketBuilder(Vec<u8>);

impl BucketBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn with_buffer(buffer: Vec<u8>) -> Self {
        Self(buffer)
    }

    /// Set the data of the bucket.
    pub fn set_data(&mut self, data: &[u8]) -> &mut Self {
        // setting the data replaces any existing data
        self.0.clear();
        // add the data to the buffer
        self.0.extend_from_slice(data);

        self
    }
}

impl<'a> ExtensionBuilder<'a, Bucket<'a>> for BucketBuilder {
    fn build(&'a self) -> Bucket<'a> {
        Bucket::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for BucketBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
