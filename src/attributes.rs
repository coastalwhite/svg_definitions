use std::clone::Clone;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::string::ToString;

use crate::color::Color;

/// An attribute to an Element
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Attribute {
    StrokeWidth,
    StrokeColor,
    FillColor,
    CenterX,
    CenterY,
    Identifier,
    PositionX,
    PositionY,
    Radius,
    Width,
    Height,
    ViewBox,
    Reference,
}

/// A value an Element Attribute may have
#[derive(Debug)]
pub enum AttributeValue {
    ViewBox(ViewBoxProps),
    Identifier(IdentifierProps),
    Color(Color),
    Number(i32),
    Reference(IdentifierProps),
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ViewBoxProps {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct IdentifierProps {
    inner_string: String,
}

// Implementation of Attribute
impl ToString for Attribute {
    fn to_string(&self) -> String {
        use Attribute::*;

        String::from(match self {
            StrokeWidth => "stroke-width",
            StrokeColor => "stroke",
            FillColor => "fill",
            CenterX => "cx",
            CenterY => "cy",
            Identifier => "id",
            PositionX => "x",
            PositionY => "y",
            Radius => "r",
            Width => "width",
            Height => "height",
            ViewBox => "viewBox",
            Reference => "href",
        })
    }
}

// Implementation of AttributeValue
impl AttributeValue {
    pub fn new_viewbox(x: i32, y: i32, width: i32, height: i32) -> AttributeValue {
        AttributeValue::ViewBox(ViewBoxProps::new(x, y, width, height))
    }

    pub fn new_id(id: &str) -> Result<AttributeValue, usize> {
        Ok(AttributeValue::Identifier(IdentifierProps::new(id)?))
    }

    pub fn new_color(red: u8, green: u8, blue: u8) -> AttributeValue {
        AttributeValue::Color(Color::new(red, green, blue))
    }

    pub fn new_number(number: i32) -> AttributeValue {
        AttributeValue::Number(number)
    }

    pub fn new_reference(reference_id: &str) -> Result<AttributeValue, usize> {
        Ok(AttributeValue::Reference(IdentifierProps::new(
            reference_id,
        )?))
    }
}

impl Clone for AttributeValue {
    fn clone(&self) -> Self {
        use AttributeValue::*;

        match self {
            ViewBox(props) => ViewBox(props.clone()),
            Identifier(props) => Identifier(props.clone()),
            Color(props) => Color(props.clone()),
            Number(props) => Number(props.clone()),
            Reference(props) => Reference(props.clone()),
        }
    }
}

impl ToString for AttributeValue {
    fn to_string(&self) -> String {
        use AttributeValue::*;

        match self {
            ViewBox(props) => props.to_string(),
            Identifier(props) => props.to_string(),
            Color(props) => props.to_string(),
            Number(props) => props.to_string(),
            Reference(props) => format!("#{}", props.to_string()),
        }
    }
}

impl From<Color> for AttributeValue {
    fn from(color: Color) -> AttributeValue {
        AttributeValue::Color(color)
    }
}

impl From<i32> for AttributeValue {
    fn from(number: i32) -> AttributeValue {
        AttributeValue::Number(number)
    }
}

impl From<(i32, i32, i32, i32)> for AttributeValue {
    fn from(view_box: (i32, i32, i32, i32)) -> AttributeValue {
        AttributeValue::new_viewbox(view_box.0, view_box.1, view_box.2, view_box.3)
    }
}

impl Hash for AttributeValue {
    fn hash<T: Hasher>(&self, state: &mut T) {
        use AttributeValue::*;

        match self {
            ViewBox(props) => props.hash(state),
            Identifier(props) => props.hash(state),
            Color(props) => props.hash(state),
            Number(props) => props.hash(state),
            Reference(props) => props.hash(state),
        }
    }
}

// Implementation of ViewBoxProps
impl ViewBoxProps {
    fn new(x: i32, y: i32, width: i32, height: i32) -> ViewBoxProps {
        ViewBoxProps {
            x,
            y,
            width,
            height,
        }
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

impl Hash for ViewBoxProps {
    fn hash<T: Hasher>(&self, state: &mut T) {
        self.x.hash(state);
        self.y.hash(state);
        self.width.hash(state);
        self.height.hash(state);
    }
}

// Implementation of IdentifierProps
impl IdentifierProps {
    fn new(inner: &str) -> Result<IdentifierProps, usize> {
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
