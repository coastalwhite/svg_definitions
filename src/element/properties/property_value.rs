use crate::color::Color;

use std::clone::Clone;
use std::hash::{Hash, Hasher};
use std::string::ToString;

#[derive(Debug)]
pub enum PropertyValue {
    ViewBox(ViewBoxProps),
    Identifier(IdentifierProps),
    Color(Color),
    Number(i32),
}

impl PropertyValue {
    pub fn new_viewbox(x: i32, y: i32, width: i32, height: i32) -> PropertyValue {
        PropertyValue::ViewBox(ViewBoxProps::new(x, y, width, height))
    }

    pub fn new_id(id: &str) -> Result<PropertyValue, usize> {
        Ok(PropertyValue::Identifier(IdentifierProps::new(id)?))
    }

    pub fn new_color(red: u8, green: u8, blue: u8) -> PropertyValue {
        PropertyValue::Color(Color::new(red, green, blue))
    }

    pub fn new_number(number: i32) -> PropertyValue {
        PropertyValue::Number(number)
    }
}

impl Clone for PropertyValue {
    fn clone(&self) -> Self {
        use PropertyValue::*;

        match self {
            ViewBox(props) => ViewBox(props.clone()),
            Identifier(props) => Identifier(props.clone()),
            Color(props) => Color(props.clone()),
            Number(props) => Number(props.clone()),
        }
    }
}

impl ToString for PropertyValue {
    fn to_string(&self) -> String {
        use PropertyValue::*;

        match self {
            ViewBox(props) => props.to_string(),
            Identifier(props) => props.to_string(),
            Color(props) => props.to_string(),
            Number(props) => props.to_string(),
        }
    }
}

impl Hash for PropertyValue {
    fn hash<T: Hasher>(&self, state: &mut T) {
        use PropertyValue::*;

        match self {
            ViewBox(props) => props.hash(state),
            Identifier(props) => props.hash(state),
            Color(props) => props.hash(state),
            Number(props) => props.hash(state),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ViewBoxProps {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

impl ViewBoxProps {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> ViewBoxProps {
        ViewBoxProps {
            x,
            y,
            width,
            height,
        }
    }
}

impl Hash for ViewBoxProps {
    fn hash<T: Hasher>(&self, state: &mut T) {
        self.x.hash(state);
        self.y.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

impl ToString for ViewBoxProps {
    fn to_string(&self) -> String {
        format!(
            "{x} {y} {width} {height}",
            x = self.x,
            y = self.y,
            width = self.width,
            height = self.height
        )
    }
}

#[derive(Debug, Clone)]
pub struct IdentifierProps {
    inner_string: String,
}

impl IdentifierProps {
    pub fn new(inner: &str) -> Result<IdentifierProps, usize> {
        for (index, character) in inner.as_bytes().iter().enumerate() {
            if !IdentifierProps::is_allowed_character(character.clone()) {
                return Err(index);
            }
        }

        Ok(IdentifierProps {
            inner_string: String::from(inner),
        })
    }

    /// Allow numbers, alphabetic characters and seperators
    fn is_allowed_character(character: u8) -> bool {
        (character >= b'0' && character <= b'9')
            || (character >= b'a' && character <= b'z')
            || (character >= b'A' && character <= b'Z')
            || character == b'-'
            || character == b'_'
            || character == b' '
    }
}

impl ToString for IdentifierProps {
    fn to_string(&self) -> String {
        self.inner_string.clone()
    }
}

impl Hash for IdentifierProps {
    fn hash<T: Hasher>(&self, state: &mut T) {
        self.inner_string.hash(state);
    }
}
