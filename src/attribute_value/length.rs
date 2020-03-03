use crate::util::prec_num;
use std::hash::{Hash, Hasher};

#[derive(Hash, Clone)]
pub enum LengthUnit {
    EM,
    EX,
    PX,
    IN,
    CM,
    MM,
    PT,
    PC,
    PERCENTAGE
}

impl ToString for LengthUnit {
    fn to_string(&self) -> String {
        use LengthUnit::*;

        String::from(match self {
            EM => "em",
            EX => "ex",
            PX => "px",
            IN => "in",
            CM => "cm",
            MM => "mm",
            PT => "pt",
            PC => "pc",
            PERCENTAGE => "%",
        })
    }
}

#[derive(Clone)]
pub struct LengthProps {
    unit: LengthUnit,
    value: f32,
}

impl LengthProps {
    pub fn new(unit: LengthUnit, value: f32) -> LengthProps {
        LengthProps {
            unit,
            value
        }
    }
}

impl ToString for LengthProps {
    fn to_string(&self) -> String {
        format!("{}{}", prec_num(self.value), self.unit.to_string())
    }
}

impl Hash for LengthProps {
    fn hash<H: Hasher>(&self, state: &mut H) {
        prec_num(self.value).hash(state);
        self.unit.hash(state);
    } 
}

/// Shorthand to create LengthProps
/// Will use unit PX
impl From<i32> for LengthProps {
    fn from(number: i32) -> Self {
        LengthProps::new(LengthUnit::PX, number as f32)
    }
}

/// Shorthand to create LengthProps
/// Will use unit %
impl From<f32> for LengthProps {
    fn from(number: f32) -> Self {
        LengthProps::new(LengthUnit::PERCENTAGE, number)
    }
}