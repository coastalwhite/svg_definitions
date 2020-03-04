//! Module containing the rgb and rgba object, and all it's logic

use std::hash::Hash;
use std::cmp::min;

use crate::util::internal_ratio::InternalRatio;

/// Represents rgb
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

/// Represents rgb
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct RGBA {
    red: u8,
    green: u8,
    blue: u8,
    alpha: InternalRatio,
}

/// Represents hsl
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct HSL {
    hue: u16,
    saturation: u16,
    light: u16,
}

/// Represents hsla
#[derive(Hash, PartialEq, Eq, Clone)]
pub struct HSLA {
    hue: u16,
    saturation: u16,
    light: u16,
    alpha: InternalRatio,
}

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct HexColor {
    red: u8,
    green: u8,
    blue: u8
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> RGB {
        RGB { red, green, blue }
    }
}

impl std::fmt::Display for RGB {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgb({},{},{})", self.red, self.green, self.blue)
    }
}

impl RGBA {
    pub fn new(red: u8, green: u8, blue: u8, alpha: InternalRatio) -> RGBA {
        RGBA { red, green, blue, alpha }
    }
}

impl std::fmt::Display for RGBA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "rgba({},{},{},{})", self.red, self.green, self.blue, self.alpha)
    }
}

impl HSL {
    pub fn new(hue: u16, saturation: u16, light: u16) -> HSL {
        HSL {
            hue: min(hue, 360),
            saturation: min(saturation, 360),
            light: min(light, 360)
        }
    }
}

impl std::fmt::Display for HSL {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsl({},{},{})", self.hue, self.saturation, self.light)
    }
}

impl HSLA {
    pub fn new(hue: u16, saturation: u16, light: u16, alpha: InternalRatio) -> HSLA {
        HSLA {
            hue: min(hue, 360),
            saturation: min(saturation, 360),
            light: min(light, 360),
            alpha,
        }
    }
}

impl std::fmt::Display for HSLA {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "hsla({},{},{},{})", self.hue, self.saturation, self.light, self.alpha)
    }
}

impl HexColor {
    pub fn new(string: &str) -> Result<HexColor,()> {
        let bytes = string.as_bytes();

        if bytes.len() != 4 && bytes.len() != 7 {
            return Err(());
        }

        if bytes[0] != b'#' {
            return Err(());
        }

        let red_str;
        let green_str;
        let blue_str;

        if bytes.len() == 4 {
            red_str = &string[1..1];
            green_str = &string[2..2];
            blue_str = &string[3..3];
        } else {
            red_str = &string[1..2];
            green_str = &string[3..4];
            blue_str = &string[5..6];
        }

        let red = u8::from_str_radix(red_str, 16).map_err(|_| ())?;
        let green = u8::from_str_radix(green_str, 16).map_err(|_| ())?;
        let blue = u8::from_str_radix(blue_str, 16).map_err(|_| ())?;

        Ok(
            HexColor {
                red,
                green,
                blue
            }
        )
    }
}

impl std::fmt::Display for HexColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "#{:2x}{:2x}{:2x}", self.red, self.green, self.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rgb_test() {
        assert_eq!(RGB::new(1, 2, 3).to_string(), String::from("rgb(1,2,3)"));
        assert_eq!(RGB::new(3, 2, 1).to_string(), String::from("rgb(3,2,1)"));
    }

    #[test]
    fn rgba_test() {
        assert_eq!(RGBA::new(1, 2, 3, 0.1.into()).to_string(), String::from("rgba(1,2,3,0.1000)"));
        assert_eq!(RGBA::new(3, 2, 1, 0.6.into()).to_string(), String::from("rgba(3,2,1,0.6000)"));
    }

    #[test]
    fn hsl_test() {
        assert_eq!(RGBA::new(1, 2, 3).to_string(), String::from("hsl(1,2,3)"));
        assert_eq!(RGBA::new(361, 2, 1).to_string(), String::from("hsl(360,2,1)"));
    }
}