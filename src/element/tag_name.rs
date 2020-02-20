use std::string::ToString;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum TagName {
    SVG,
    Defs,
    SVGPath,
    Circle,
    Rectangle,
    Group,
    Use,
}

impl ToString for TagName {
    fn to_string(&self) -> String {
        use TagName::*;

        String::from(match self {
            SVG => "svg",
            Defs => "defs",
            SVGPath => "path",
            Circle => "circle",
            Rectangle => "rect",
            Group => "g",
            Use => "use",
        })
    }
}
