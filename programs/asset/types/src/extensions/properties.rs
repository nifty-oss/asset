use bytemuck::{Pod, Zeroable};
use podded::types::{U8PrefixStr, U8PrefixStrMut};
use std::{fmt::Debug, ops::Deref};

use super::{ExtensionBuilder, ExtensionData, ExtensionDataMut, ExtensionType, Lifecycle};

/// Extension to add (typed) properties to an asset.
///
/// The extension currently supports:
/// * `&str`: `"type": "asset"`
/// * `u64`: `"version": 1`
/// * `bool`: `"alpha": false`
pub struct Properties<'a> {
    values: Vec<Property<'a>>,
}

impl Properties<'_> {
    /// Get the value of a property by name.
    ///
    /// If no value is found under the `name`, returns `None`.
    pub fn get<T: Value>(&self, name: &str) -> Option<&T> {
        if let Some(pointer) = self
            .values
            .iter()
            .find(|t| t.name.as_str() == name)
            .map(|p| &*p.value as *const dyn Value as *const T)
        {
            unsafe { pointer.as_ref() }
        } else {
            None
        }
    }
}

impl<'a> Deref for Properties<'a> {
    type Target = Vec<Property<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> ExtensionData<'a> for Properties<'a> {
    const TYPE: ExtensionType = ExtensionType::Properties;

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

/// Mutable version of the `Properties` extension.
pub struct PropertiesMut<'a> {
    values: Vec<Property<'a>>,
}

impl PropertiesMut<'_> {
    /// Get the value of a property by name.
    ///
    /// If no value is found under the `name`, returns `None`.
    pub fn get<T: Value>(&self, name: &str) -> Option<&T> {
        if let Some(pointer) = self
            .values
            .iter()
            .find(|t| t.name.as_str() == name)
            .map(|p| &*p.value as *const dyn Value as *const T)
        {
            unsafe { pointer.as_ref() }
        } else {
            None
        }
    }

    pub fn remove(&mut self, name: &str) {
        self.values.retain(|p| p.name.as_str() != name);
    }
}

impl<'a> Deref for PropertiesMut<'a> {
    type Target = Vec<Property<'a>>;

    fn deref(&self) -> &Self::Target {
        &self.values
    }
}

impl<'a> ExtensionDataMut<'a> for PropertiesMut<'a> {
    const TYPE: ExtensionType = ExtensionType::Properties;

    fn from_bytes_mut(bytes: &'a mut [u8]) -> Self {
        let mut cursor = 0;
        let mut values = Vec::new();

        while cursor < bytes.len() {
            let p = Property::from_bytes(&bytes[cursor..]);
            cursor += p.size();
            values.push(p);
        }
        Self { values }
    }
}

impl Lifecycle for PropertiesMut<'_> {}

/// A property with a name and a value.
pub struct Property<'a> {
    pub name: U8PrefixStr<'a>,

    pub value: Box<dyn Value + 'a>,
}

impl<'a> Property<'a> {
    pub fn size(&self) -> usize {
        self.name.size() + self.value.size()
    }

    fn from_bytes(bytes: &'a [u8]) -> Self {
        let name = U8PrefixStr::from_bytes(bytes);

        let (_, value) = bytes.split_at(name.size());

        let value = match value[0].into() {
            Type::Text => Box::new(Text::from_bytes(value)) as Box<dyn Value>,
            Type::Number => Box::new(Number::from_bytes(value)) as Box<dyn Value>,
            Type::Boolean => Box::new(Boolean::from_bytes(value)) as Box<dyn Value>,
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

/// Trait representing a value in a property.
pub trait Value: Debug {
    fn size(&self) -> usize;
}

/// Type of the value in a property.
///
/// This is used to "encode" the type on the serialized data.
#[repr(u8)]
#[derive(Clone, Copy)]
pub enum Type {
    Text,
    Number,
    Boolean,
}

unsafe impl Pod for Type {}

unsafe impl Zeroable for Type {}

impl From<Type> for u8 {
    fn from(value: Type) -> Self {
        match value {
            Type::Text => 0,
            Type::Number => 1,
            Type::Boolean => 2,
        }
    }
}

impl From<u8> for Type {
    fn from(value: u8) -> Self {
        match value {
            0 => Type::Text,
            1 => Type::Number,
            2 => Type::Boolean,
            _ => panic!("invalid value type: {}", value),
        }
    }
}

/// A string value in a property.
pub struct Text<'a> {
    pub value: U8PrefixStr<'a>,
}

impl<'a> Text<'a> {
    pub fn from_bytes(bytes: &'a [u8]) -> Self {
        let (_, value) = bytes.split_at(std::mem::size_of::<Type>());

        Self {
            value: U8PrefixStr::from_bytes(value),
        }
    }
}

impl Value for Text<'_> {
    fn size(&self) -> usize {
        std::mem::size_of::<Type>() + self.value.size()
    }
}

impl Debug for Text<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

impl Deref for Text<'_> {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.value.as_str()
    }
}

/// A numeric value in a property.
pub struct Number<'a> {
    pub value: &'a [u8; 8],
}

impl<'a> Number<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        const START: usize = std::mem::size_of::<Type>();
        let value = bytemuck::from_bytes(&bytes[START..START + std::mem::size_of::<u64>()]);

        Self { value }
    }
}

impl Value for Number<'_> {
    fn size(&self) -> usize {
        std::mem::size_of::<Type>() + std::mem::size_of_val(self.value)
    }
}

impl Debug for Number<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl Deref for Number<'_> {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.value as *const u8 as *const u64) }
    }
}

/// A boolean value in a property.
pub struct Boolean<'a> {
    pub value: &'a u8,
}

impl<'a> Boolean<'a> {
    fn from_bytes(bytes: &'a [u8]) -> Self {
        const START: usize = std::mem::size_of::<Type>();
        let value = bytemuck::from_bytes(&bytes[START..START + std::mem::size_of::<u8>()]);
        Self { value }
    }
}

impl Value for Boolean<'_> {
    fn size(&self) -> usize {
        std::mem::size_of::<Type>() + std::mem::size_of_val(self.value)
    }
}

impl Debug for Boolean<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.deref())
    }
}

impl Deref for Boolean<'_> {
    type Target = bool;

    fn deref(&self) -> &Self::Target {
        unsafe { std::mem::transmute::<&u8, &bool>(self.value) }
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
    pub fn add_text(&mut self, name: &str, value: &str) -> &mut Self {
        // add the length of the name + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; name.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(name);

        // add the value type
        self.0.push(Type::Text.into());

        // add the length of the value + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; value.len() + 1]);
        let mut value_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        value_str.copy_from_str(value);

        self
    }

    /// Add a new numeric property to the extension.
    pub fn add_number(&mut self, name: &str, value: u64) -> &mut Self {
        // add the length of the name + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; name.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(name);

        // add the value type
        self.0.push(Type::Number.into());

        // add the numeric value to the data buffer.
        self.0.extend_from_slice(&value.to_le_bytes());

        self
    }

    /// Add a new boolean property to the extension.
    pub fn add_boolean(&mut self, name: &str, value: bool) -> &mut Self {
        // add the length of the name + prefix to the data buffer.
        let cursor = self.0.len();
        self.0.append(&mut vec![0u8; name.len() + 1]);
        let mut name_str = U8PrefixStrMut::new(&mut self.0[cursor..]);
        name_str.copy_from_str(name);

        // add the value type
        self.0.push(Type::Boolean.into());

        // add the boolean value to the data buffer.
        self.0.push(if value { 1 } else { 0 });

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
    use crate::extensions::{
        Boolean, ExtensionBuilder, ExtensionDataMut, Number, PropertiesMut, Text,
    };

    #[test]
    pub fn test_create_property() {
        let mut builder = PropertiesBuilder::default();
        builder.add_text("name", "asset");
        builder.add_number("version", 1);
        builder.add_boolean("alpha", false);
        let properties = builder.build();

        assert_eq!(properties.values.len(), 3);
        assert_eq!(properties.values[0].name.as_str(), "name");
        assert_eq!(properties.values[1].name.as_str(), "version");
        assert_eq!(properties.values[2].name.as_str(), "alpha");

        let name: &str = properties.get::<Text>("name").unwrap();
        assert_eq!(name, "asset");

        let version: &u64 = properties.get::<Number>("version").unwrap();
        assert_eq!(version, &1u64);

        let alpha: &bool = properties.get::<Boolean>("alpha").unwrap();
        assert_eq!(alpha, &false);
    }

    #[test]
    pub fn test_remove_property() {
        let mut builder = PropertiesBuilder::default();
        builder.add_text("name", "asset");
        builder.add_number("version", 1);
        let mut data = builder.data();

        let mut properties = PropertiesMut::from_bytes_mut(&mut data);

        assert_eq!(properties.values.len(), 2);
        assert_eq!(properties.values[0].name.as_str(), "name");
        assert_eq!(properties.values[1].name.as_str(), "version");

        properties.remove("name");

        assert_eq!(properties.values.len(), 1);
        assert_eq!(properties.values[0].name.as_str(), "version");
    }
}
