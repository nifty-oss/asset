use podded::types::U8PrefixStr;

use super::{ExtensionData, ExtensionType};

/// Extension to add attributes.
pub struct Attributes<'a> {
    pub values: Vec<Trait<'a>>,
}

impl<'a> ExtensionData<'a> for Attributes<'a> {
    const TYPE: ExtensionType = ExtensionType::Attributes;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let mut cursor = 0;
        let mut values = Vec::new();

        while cursor < bytes.len() {
            let t = Trait::from_bytes(&bytes[cursor..]);
            cursor += t.length();
            values.push(t);
        }
        Self { values }
    }

    fn length(&self) -> usize {
        self.values.iter().map(|link| link.length()).sum()
    }
}

/// Trait information.
pub struct Trait<'a> {
    /// Name of the trait.
    pub name: U8PrefixStr<'a>,

    /// Value of the trait.
    pub value: U8PrefixStr<'a>,
}

impl<'a> Trait<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let name = U8PrefixStr::from_bytes(bytes);
        let value = U8PrefixStr::from_bytes(&bytes[name.len()..]);
        Self { name, value }
    }

    pub fn length(&self) -> usize {
        // 2 bytes for the length prefix of the name and value.
        2 + self.name.len() + self.value.len()
    }
}
