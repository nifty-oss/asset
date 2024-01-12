use podded::types::U8PrefixStr;
use std::fmt::Debug;

use super::{ExtensionData, ExtensionType};

/// Extension to add attributes.
pub struct Links<'a> {
    pub values: Vec<Link<'a>>,
}

impl<'a> ExtensionData<'a> for Links<'a> {
    const TYPE: ExtensionType = ExtensionType::Links;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let mut cursor = 0;
        let mut values = Vec::new();

        while cursor < bytes.len() {
            let link = Link::from_bytes(&bytes[cursor..]);
            cursor += link.length();
            values.push(link);
        }
        Self { values }
    }

    fn length(&self) -> usize {
        self.values.iter().map(|link| link.length()).sum()
    }
}

impl Debug for Links<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Links")
            .field("values", &self.values)
            .finish()
    }
}

/// Link information.
pub struct Link<'a> {
    /// Name of the link.
    pub name: U8PrefixStr<'a>,

    /// URI value.
    pub uri: U8PrefixStr<'a>,
}

impl<'a> Link<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let name = U8PrefixStr::from_bytes(bytes);
        let uri = U8PrefixStr::from_bytes(&bytes[name.size()..]);
        Self { name, uri }
    }

    pub fn length(&self) -> usize {
        self.name.size() + self.uri.size()
    }
}

impl Debug for Link<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Link")
            .field("name", &self.name.as_str())
            .field("uri", &self.uri.as_str())
            .finish()
    }
}
