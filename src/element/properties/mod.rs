use std::clone::Clone;
use std::string::ToString;

pub mod property_value;

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
pub enum Properties {
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

impl ToString for Properties {
    fn to_string(&self) -> String {
        use Properties::*;

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
