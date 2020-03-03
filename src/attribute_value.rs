pub mod animate;
pub mod clip_reference;
pub mod color;
pub mod identifier;
pub mod path;
pub mod url;
pub mod values;
pub mod viewbox;
pub mod transform_list;

use std::hash::{Hash, Hasher};

use crate::attributes::Attribute;
use animate::*;
use clip_reference::ClipReferenceProps;
use color::{Color, TColor};
use identifier::IdentifierProps;
use path::PathDefinitionString;
use url::URLProps;
use values::{ValueProps, ValuesProps};
use viewbox::ViewBoxProps;
use transform_list::TransformListProps;

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

    Percentage(f64),

    URL(URLProps),
    AttributeName(Attribute),
    Values(ValuesProps),

    Duration(DurationProps),
    Count(CountProps),

    Value(ValueProps),

    Restart(RestartProps),

    Additive(AdditiveProps),
    Accumulative(AccumulativeProps),

    AnimateRotate(AnimateRotateProps),

    KeySplines(KeySplinesProps),

    TransformList(TransformListProps),

    GradiantUnit(GradiantUnitProps),
    SpreadMethod(SpreadMethodProps),
    StopColor(StopColorProps),
    Opacity(OpacityProps)


    Auto,

    CalculationMode(CalculationModeProps),

    ClipReference(ClipReferenceProps),

    Reference(IdentifierProps),
    PathDefinitionValue(PathDefinitionString),
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
