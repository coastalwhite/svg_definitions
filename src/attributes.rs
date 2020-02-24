//! This module provides an easy and safe way to interact with attributes of [Elements](../struct.Element.html)
//!
//! # Note
//! In the [crate::prelude](../prelude/index.html) the name for
//! [Attribute](enum.Attribute.html) is [Attr](../prelude/index.html) and for
//! [AttributeValue](enum.AttributeValue.html) is [AttrValue](../prelude/index.html)
//!
//! # Examples
//! ## 1) Setting attributes of a circle
//! ```
//! use svg_definitions::prelude::*;
//!
//! let circle = SVGElem::new(Tag::Circle)
//!     .set(Attr::Radius, (10.0).into())
//!     .set(Attr::CenterX, 15.into())
//!     .set(Attr::CenterY, 15.into())
//!     .set(Attr::StrokeWidth, 1.into())
//!     .set(Attr::StrokeColor, RGB::new(0,0,0).into())
//!     .set(Attr::FillColor, RGBT::Transparent.into());
//! ```
//!
//! ## 2) Setting attributes of a triangle
//! ```
//! use svg_definitions::prelude::*;
//!
//! let triangle = SVGElem::new(Tag::SVGPath)
//!     .set(Attr::StrokeWidth, 1.into())
//!     .set(Attr::StrokeColor, RGB::new(0,0,0).into())
//!     .set(Attr::FillColor, RGBT::Transparent.into())
//!     .set(Attr::PathDefinition, PathString::new()
//!         .move_to((0.0, 0.0))
//!         .line_to((10.0, 0.0))
//!         .line_to((0.0, 10.0))
//!         .line_to((0.0, 0.0))
//!         .close_path()
//!         .into()
//!     );
//! ```

use std::clone::Clone;
use std::convert::From;
use std::hash::{Hash, Hasher};
use std::string::ToString;

use crate::color::{Color, TColor};
use crate::path::PathDefinitionString;

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
    PathDefinition,
}

/// A value an Element Attribute may have
#[derive(Debug)]
pub enum AttributeValue {
    ViewBox(ViewBoxProps),
    Identifier(IdentifierProps),
    Color(Color),
    TColor(TColor),
    Integer(i32),

    /// # NOTE
    /// Will be rounded to two decimal points
    Float(f64),
    Reference(IdentifierProps),
    PathDefinitionValue(PathDefinitionString),
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
            PathDefinition => "d",
        })
    }
}

// Implementation of AttributeValue
impl AttributeValue {
    /// Create new AttributeValue::ViewBox
    ///
    /// # Note
    /// For a shorthand look at: [From<(i32, i32, i32, i32)>](#impl-From<(i32%2C%20i32%2C%20i32%2C%20i32)>)
    pub fn new_viewbox(x: i32, y: i32, width: i32, height: i32) -> AttributeValue {
        AttributeValue::ViewBox(ViewBoxProps::new(x, y, width, height))
    }

    /// Create new AttributeValue::Identifier
    ///
    /// # Note
    /// Will return error if not allowed characters are used
    pub fn new_id(id: &str) -> Result<AttributeValue, usize> {
        Ok(AttributeValue::Identifier(IdentifierProps::new(id)?))
    }

    /// Create new AttributeValue::Color
    ///
    /// # Note
    /// For a shorthand look at: [From<(Color)>](#impl-From<Color>)
    pub fn new_color(red: u8, green: u8, blue: u8) -> AttributeValue {
        AttributeValue::Color(Color::new(red, green, blue))
    }

    /// Create new AttributeValue::Integer
    ///
    /// # Note
    /// For a shorthand look at: [From<i32>](#impl-From<i32>)
    pub fn new_integer(number: i32) -> AttributeValue {
        AttributeValue::Integer(number)
    }

    /// Create new AttributeValue::Float
    ///
    /// # Note
    /// For a shorthand look at: [From<f64>](#impl-From<f64>)
    pub fn new_float(number: f64) -> AttributeValue {
        AttributeValue::Float(number)
    }

    /// Create new AttributeValue::Reference
    ///
    /// # Note
    /// Will return error if not allowed characters are used
    pub fn new_reference(reference_id: &str) -> Result<AttributeValue, usize> {
        Ok(AttributeValue::Reference(IdentifierProps::new(
            reference_id,
        )?))
    }

    /// Create a new AttributeValue::PathDefinitionValue
    pub fn new_path_definition(path_string: PathDefinitionString) -> AttributeValue {
        AttributeValue::PathDefinitionValue(path_string)
    }
}

impl Clone for AttributeValue {
    fn clone(&self) -> Self {
        use AttributeValue::*;

        match self {
            ViewBox(props) => ViewBox(props.clone()),
            Identifier(props) => Identifier(props.clone()),
            Color(props) => Color(props.clone()),
            TColor(props) => TColor(props.clone()),
            Integer(props) => Integer(props.clone()),
            Float(props) => Float(props.clone()),
            Reference(props) => Reference(props.clone()),
            PathDefinitionValue(props) => PathDefinitionValue(props.clone()),
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
            TColor(props) => props.to_string(),
            Integer(props) => props.to_string(),
            Float(props) => format!("{:.2}", props),
            Reference(props) => format!("#{}", props.to_string()),
            PathDefinitionValue(props) => props.to_string(),
        }
    }
}

/// Shorthand to create [AttributeValue::Color]
impl From<Color> for AttributeValue {
    fn from(color: Color) -> AttributeValue {
        AttributeValue::Color(color)
    }
}

/// Shorthand to create [AttributeValue::TColor]
impl From<TColor> for AttributeValue {
    fn from(color: TColor) -> AttributeValue {
        AttributeValue::TColor(color)
    }
}

/// Shorthand to create [AttributeValue::Integer]
impl From<i32> for AttributeValue {
    fn from(number: i32) -> AttributeValue {
        AttributeValue::Integer(number)
    }
}

/// Shorthand to create [AttributeValue::Float]
impl From<f64> for AttributeValue {
    fn from(number: f64) -> AttributeValue {
        AttributeValue::Float(number)
    }
}

/// Shorthand to create [AttributeValue::ViewBox]
impl From<(i32, i32, i32, i32)> for AttributeValue {
    fn from(view_box: (i32, i32, i32, i32)) -> AttributeValue {
        AttributeValue::new_viewbox(view_box.0, view_box.1, view_box.2, view_box.3)
    }
}

/// Shorthand to create [AttributeValue::PathDefinitionValue]
impl From<PathDefinitionString> for AttributeValue {
    fn from(path_string: PathDefinitionString) -> AttributeValue {
        AttributeValue::new_path_definition(path_string)
    }
}

impl Hash for AttributeValue {
    fn hash<T: Hasher>(&self, state: &mut T) {
        use AttributeValue::*;

        match self {
            ViewBox(props) => props.hash(state),
            Identifier(props) => props.hash(state),
            Color(props) => props.hash(state),
            TColor(props) => props.hash(state),
            Integer(props) => props.hash(state),
            Float(props) => (format!("{:.2}", props)).hash(state),
            Reference(props) => props.hash(state),
            PathDefinitionValue(props) => props.hash(state),
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
