//! Prelude for this crate, this contains a lot of useful exports

pub use crate::Element as SVGElem;
pub use crate::Point2D;

pub use crate::attributes::Attribute as Attr;
pub use crate::tag_name::TagName as Tag;

pub use crate::path::PathDefinitionString as PathData;

#[cfg(feature = "parsing")]
pub use crate::parser::{parse_file as SVGParseFile, parse_text as SVGParseText};
