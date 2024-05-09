use bytemuck::{Pod, Zeroable};
use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to add (typed) properties to an asset – e.g., `"id": 1`.
pub struct Properties<'a> {
    values: Vec<Property<'a>>,
}

impl Properties<'_> {
    /// Get the value of a property by name.
    ///
    /// If no value is found under the `name`, returns `None`.
    pub fn get(&self, name: &str) -> Option<&dyn Value> {
        self.values
            .iter()
            .find(|t| t.name.as_str() == name)
            .map(|p| p.value.as_ref())
    }
}

impl<'a> Deref for Properties<'a> {
    type Target = Vec<Property<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> ExtensionData<'a> for Properties<'a> {
    const TYPE: ExtensionType = ExtensionType::Attributes;

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let mut cursor = 0;
        let mut values = Vec::new();

        while cursor < bytes.len() {
            let p = Property::from_bytes(&bytes[cursor..]);
            cursor += p.size();
            values.push(p);
        }
        Self { values }
    }

    fn length(&self) -> usize {
        self.values.iter().map(|t| t.size()).sum()
    }
}

impl Debug for Properties<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Properties")
            .field("values", &self.values)
            .finish()
    }
}

pub struct PropertiesMut<'a> {
    values: Vec<PropertyMut<'a>>,
}

impl<'a> Deref for PropertiesMut<'a> {
    type Target = Vec<PropertyMut<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> ExtensionDataMut<'a> for PropertiesMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Attributes;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let mut values = Vec::new();
        // mutable reference to the current bytes
        let mut bytes = bytes;

        while !bytes.is_empty() {
            let p = Property::from_bytes(bytes);
            let cursor = p.size();
            drop(p);

            let (current, remainder) = bytes.split_at_mut(cursor);
            let p = PropertyMut::from_bytes_mut(current);
            bytes = remainder;

            values.push(p);
        }
        Self { values }
    }
}

impl Lifecycle for PropertiesMut<'_> {}

pub struct Property<'a> {
    name: U8PrefixStr<'a>,

    value: Box<dyn Value + 'a>,
}

impl<'a> Property<'a> {
    pub fn as_u64(&self) -> Option<u64> {
        self.value.as_u64()
    }

    pub fn as_str(&self) -> Option<&str> {
        self.value.as_str()
    }

    pub fn size(&self) -> usize {
        self.name.size() + self.value.size()
    }

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let name = U8PrefixStr::from_bytes(bytes);

        let (_, value) = bytes.split_at(name.size());

        let value = match value[0].into() {
            ValueType::Numeric => Box::new(NumericValue::from_bytes(value)) as Box<dyn Value>,
            ValueType::String => Box::new(StringValue::from_bytes(value)) as Box<dyn Value>,
        };

        Self { name, value }
    }
}

impl Debug for Property<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Property")
            .field("name", &self.name.as_str())
            .field("value", &self.value)
            .finish()
    }
}

pub struct PropertyMut<'a> {
    name: U8PrefixStrMut<'a>,

    value: Box<dyn Value + 'a>,
}

impl<'a> PropertyMut<'a> {
    pub fn as_u64(&self) -> Option<u64> {
        self.value.as_u64()
    }

    pub fn as_str(&self) -> Option<&str> {
        self.value.as_str()
    }

    pub fn size(&self) -> usize {
        self.name.len() + self.value.size()
    }

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let name = U8PrefixStr::from_bytes(bytes);

        let (name, value) = bytes.split_at_mut(name.size());

        let name = U8PrefixStrMut::from_bytes_mut(name);

        let value = match value[0].into() {
            ValueType::Numeric => Box::new(NumericValue::from_bytes(value)) as Box<dyn Value>,
            ValueType::String => Box::new(StringValue::from_bytes(value)) as Box<dyn Value>,
        };

        Self { name, value }
    }
}

pub trait Value: Debug {
    fn as_u64(&self) -> Option<u64>;

    fn as_str(&self) -> Option<&str>;

    fn size(&self) -> usize;
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum ValueType {
    String,
    Numeric,
}

unsafe impl Pod for ValueType {}

unsafe impl Zeroable for ValueType {}

impl From<ValueType> for u8 {
    fn from(value: ValueType) -> Self {
        match value {
            ValueType::String => 0,
            ValueType::Numeric => 1,
        }
    }
}

impl From<u8> for ValueType {
    fn from(value: u8) -> Self {
        match value {
            0 => ValueType::String,
            1 => ValueType::Numeric,
            _ => panic!("invalid value type: {}", value),
        }
    }
}

trait TypedValue: Value {
    const TYPE: ValueType;
}

pub struct NumericValue<'a> {
    pub value: &'a [u8; 8],
}

impl TypedValue for NumericValue<'_> {
    const TYPE: ValueType = ValueType::Numeric;
}

impl Value for NumericValue<'_> {
    fn as_str(&self) -> Option<&str> {
        None
    }

    fn as_u64(&self) -> Option<u64> {
        Some(u64::from_le_bytes(*self.value))
    }

    fn size(&self) -> usize {
        std::mem::size_of::<ValueType>() + std::mem::size_of_val(self.value)
    }
}

impl Debug for NumericValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_u64().unwrap())
    }
}

impl<'a> NumericValue<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        let (_, value) = bytes.split_at(std::mem::size_of::<ValueType>());

        Self {
            value: bytemuck::from_bytes(value),
        }
    }
}

pub struct StringValue<'a> {
    pub value: U8PrefixStr<'a>,
}

impl TypedValue for StringValue<'_> {
    const TYPE: ValueType = ValueType::String;
}

impl Value for StringValue<'_> {
    fn as_str(&self) -> Option<&str> {
        Some(self.value.deref())
    }

    fn as_u64(&self) -> Option<u64> {
        None
    }

    fn size(&self) -> usize {
        std::mem::size_of::<ValueType>() + self.value.size()
    }
}

impl Debug for StringValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl<'a> StringValue<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let (_, value) = bytes.split_at(std::mem::size_of::<ValueType>());

        Self {
            value: U8PrefixStr::from_bytes(value),
        }
    }
}

/// Builder for an `Properties` extension.
#[derive(Default)]
pub struct PropertiesBuilder(Vec<u8>);

impl PropertiesBuilder {
    pub fn with_capacity(capacity: usize) -> Self {
        Self(Vec::with_capacity(capacity))
    }

    pub fn with_buffer(buffer: Vec<u8>) -> Self {
        let mut s = Self(buffer);
        s.0.clear();
        s
    }

    /// Add a new string property to the extension.
    pub fn add_string(&mut self, name: &str, value: &str) -> &mut Self {
        // add the length of the name + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; name.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(name);

        // add the value type
        self.0.push(ValueType::String.into());

        // add the length of the value + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; value.len() + 1]);
        let mut value_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        value_str.copy_from_str(value);

        self
    }

    /// Add a new numeric property to the extension.
    pub fn add_numeric(&mut self, name: &str, value: u64) -> &mut Self {
        // add the length of the name + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; name.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(name);

        // add the value type
        self.0.push(ValueType::Numeric.into());

        // add the numeric value to the data buffer.
        self.0.extend_from_slice(&value.to_le_bytes());

        self
    }
}

impl<'a> ExtensionBuilder<'a, Properties<'a>> for PropertiesBuilder {
    fn build(&self) -> Properties {
        Properties::from_bytes(&self.0)
    }

    fn data(&mut self) -> Vec<u8> {
        std::mem::take(&mut self.0)
    }
}

impl Deref for PropertiesBuilder {
    type Target = Vec<u8>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::PropertiesBuilder;
    use crate::extensions::ExtensionBuilder;

    #[test]
    pub fn test_create_property() {
        let mut builder = PropertiesBuilder::default();
        builder.add_string("name", "asset");
        builder.add_numeric("version", 1);
        let properties = builder.build();

        assert_eq!(properties.values.len(), 2);
        assert_eq!(properties.values[0].name.as_str(), "name");
        assert_eq!(properties.values[1].name.as_str(), "version");

        assert_eq!(properties.get("name").unwrap().as_str().unwrap(), "asset");
        assert_eq!(properties.get("version").unwrap().as_u64().unwrap(), 1);
    }
}
