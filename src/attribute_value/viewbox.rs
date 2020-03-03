use std::hash::{Hash, Hasher};

use crate::attribute_value::AttributeValue;

#[doc(hidden)]
#[derive(Debug, Clone)]
pub struct ViewBoxProps {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

// Implementation of ViewBoxProps
impl ViewBoxProps {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> ViewBoxProps {
        ViewBoxProps {
            x,
            y,
            width,
            height,
        }
    }
}

/// Shorthand to create [AttributeValue::ViewBox]
impl From<(i32, i32, i32, i32)> for AttributeValue {
    fn from(view_box: (i32, i32, i32, i32)) -> AttributeValue {
        AttributeValue::new_viewbox(view_box.0, view_box.1, view_box.2, view_box.3)
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
