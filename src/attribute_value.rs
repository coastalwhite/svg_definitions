pub mod color;
pub mod identifier;
pub mod path;
pub mod viewbox;
pub mod length;
pub mod paint;
pub mod opacity;

use std::hash::{Hash, Hasher};

use crate::attributes::Attribute;
use crate::util::prec_num;
use color::{Color, TColor};
use identifier::IdentifierProps;
use path::PathDefinitionString;
use viewbox::ViewBoxProps;

/// A value an Element Attribute may have
#[derive(Clone)]
pub enum AttributeValue {
    ViewBox(ViewBoxProps),
    Identifier(IdentifierProps),
    Color(Color),
    TColor(TColor),
    Length(length::LengthProps),
    Integer(i32),
    Float(f32),
    Percentage(f32),
    Paint(PaintProps),

    /// # NOTE
    /// Will be rounded to two decimal points

    //URL(URLProps),
    // AttributeName(Attribute),
    //Values(ValuesProps),

    //Duration(DurationProps),
    //Count(CountProps),

    //Value(ValueProps),

    //Restart(RestartProps),

    // Additive(AdditiveProps),
    // Accumulative(AccumulativeProps),

    // AnimateRotate(AnimateRotateProps),

    // KeySplines(KeySplinesProps),

    // TransformList(TransformListProps),

    // GradiantUnit(GradiantUnitProps),
    // SpreadMethod(SpreadMethodProps),
    // StopColor(StopColorProps),
    // Opacity(OpacityProps)


    // Auto,

    // CalculationMode(CalculationModeProps),

    // ClipReference(ClipReferenceProps),

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
    pub fn new_float(number: f32) -> AttributeValue {
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

impl ToString for AttributeValue {
    fn to_string(&self) -> String {
        use AttributeValue::*;

        match self {
            ViewBox(props) => props.to_string(),
            Identifier(props) => props.to_string(),
            Color(props) => props.to_string(),
            TColor(props) => props.to_string(),
            Integer(props) => props.to_string(),
            Float(props) => prec_num(*props),
            Reference(props) => format!("#{}", props.to_string()),
            Length(props) => props.to_string(),
            PathDefinitionValue(props) => props.to_string(),
            Percentage(props) => format!("{}%", prec_num(*props))
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
            Float(props) => prec_num(*props).hash(state),
            Reference(props) => props.hash(state),
            Length(props) => props.hash(state),
            PathDefinitionValue(props) => props.hash(state),
            Percentage(props) => format!("{}%", prec_num(*props)).hash(state),
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
impl From<f32> for AttributeValue {
    fn from(number: f32) -> AttributeValue {
        AttributeValue::Float(number)
    }
}
