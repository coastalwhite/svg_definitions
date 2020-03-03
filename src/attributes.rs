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
    RadiusX,
    RadiusY,
    Width,
    Height,
    ViewBox,
    Reference,
    PathDefinition,
    AttributeName,
    Values,
    Begin,
    End,
    Min,
    Max,
    RepeatCount,
    RepeatDuration,
    Additive,
    Accumulative,
    Duration,
    AnimatePath,
    CalculationMode,
    KeyTimes,
    KeySplines,
    AnimationFrom,
    AnimationTo,
    AnimationBy,
    AnimateRotate,
    PathLength,
    ClipPath,
    Line,
    PositionX1,
    PositionX2,
    PositionY1,
    PositionY2,

    /// https://developer.mozilla.org/en-US/docs/Web/SVG/Attribute/transform
    GradiantTransform,
    GradiantUnits,
    SpreadMethod,
    Offset,
    StopColor,
    StopOpacity,
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

#[cfg(test)]
mod tests {
    use crate::prelude::*;

    #[test]
    fn test_identifier_string() {
        assert_eq!(AttrValue::new_id("hi@").unwrap_err(), 2);
        assert_eq!(AttrValue::new_id("@hi@").unwrap_err(), 0);
        assert_eq!(AttrValue::new_reference("hi@").unwrap_err(), 2);
        assert_eq!(AttrValue::new_reference("@hi@").unwrap_err(), 0);
    }
}
