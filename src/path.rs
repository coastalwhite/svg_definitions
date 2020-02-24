//! This module provides an nicer and easier way to interact with SVG Path Definition Strings.
//!
//! # Note
//! In the [crate::prelude](../prelude/index.html) the name for
//! [PathDefinitionString](struct.PathDefinitionString.html) is [PathString](../prelude/index.html)
//!
//! # Examples
//! ## 1) Triangle
//! ```
//! use svg_definitions::prelude::*;
//!
//! let path_definition_string = PathString::new()
//!     .move_to((0.0, 0.0))
//!     .line_to((10.0, 0.0))
//!     .line_to((0.0, 10.0))
//!     .line_to((0.0, 0.0))
//!     .close_path();
//! ```
//!
//! ## 2) Relative Triangle
//! ```
//! use svg_definitions::prelude::*;
//!
//! let path_definition_string = PathString::new()
//!     .move_to((0.0, 0.0))
//!     .r_line_to((10.0, 0.0))
//!     .r_line_to((-10.0, 10.0))
//!     .r_line_to((0.0, -10.0))
//!     .close_path();
//! ```

use std::clone::Clone;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct PathDefinitionString {
    inner_string: String,
}

type Point2D = (f64, f64);

impl PathDefinitionString {
    /// Creates a new empty instance of a PathDefinitionString
    ///
    /// # Note
    /// Eventhough, one can input f64's the actual output string will always output number with 2 decimals points.
    pub fn new() -> PathDefinitionString {
        PathDefinitionString {
            inner_string: String::from(""),
        }
    }

    /// Compares input string with PathDefinitionString and returns true if both are equal
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .line_to((6.0, 6.0))
    ///     .close_path();
    ///
    /// // Both of these statements are equivalent
    /// let eq_1 = path_definition_string.to_string() == String::from("M 3.00 3.00 L 6.00 6.00 Z");
    /// # assert!(eq_1);
    /// let eq_2 = path_definition_string.is_str("M 3.00 3.00 L 6.00 6.00 Z");
    /// # assert!(eq_2);
    /// # assert_eq!(eq_1, eq_2);
    /// ```
    pub fn is_str(&self, eq: &str) -> bool {
        self.inner_string.trim_start() == eq
    }

    /// Appends a move to a certain point to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0));
    ///
    /// // Will output "M 3.00 3.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00"));
    /// ```
    pub fn move_to(self, (x, y): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!("{} M {:.2} {:.2}", self.inner_string, x, y),
        }
    }

    /// Appends a line to a certain point to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .line_to((10.0, 10.0));
    ///
    /// // Will output "M 3.00 3.00 L 10.00 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 L 10.00 10.00"));
    /// ```
    pub fn line_to(self, (x, y): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!("{} L {:.2} {:.2}", self.inner_string, x, y),
        }
    }

    /// Appends a horizontal line to a certain x-value to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .horizontal_line_to(10.0);
    ///
    /// // Will output "M 3.00 3.00 H 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 H 10.00"));
    /// ```
    pub fn horizontal_line_to(self, x: f64) -> Self {
        PathDefinitionString {
            inner_string: format!("{} H {:.2}", self.inner_string, x),
        }
    }

    /// Appends a vertical line to a certain y-value to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .vertical_line_to(10.0);
    ///
    /// // Will output "M 3.00 3.00 V 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 V 10.00"));
    /// ```
    pub fn vertical_line_to(self, y: f64) -> Self {
        PathDefinitionString {
            inner_string: format!("{} V {:.2}", self.inner_string, y),
        }
    }

    /// Appends a line to a certain point relative to where the last action ended to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .r_line_to((7.0, 7.0));
    ///
    /// // Will output "M 3.00 3.00 l 7.00 7.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 l 7.00 7.00"));
    /// ```
    pub fn r_line_to(self, (dx, dy): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!("{} l {:.2} {:.2}", self.inner_string, dx, dy),
        }
    }

    /// Appends a horizontal line to a certain x-value relative to where the last action ended to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .r_horizontal_line_to(7.0);
    ///
    /// // Will output "M 3.00 3.00 h 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 h 7.00"));
    /// ```
    pub fn r_horizontal_line_to(self, dx: f64) -> Self {
        PathDefinitionString {
            inner_string: format!("{} h {:.2}", self.inner_string, dx),
        }
    }

    /// Appends a vertical line to a certain y-value relative to where the last action ended to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .r_vertical_line_to(7.0);
    ///
    /// // Will output "M 3.00 3.00 v 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 v 7.00"));
    /// ```
    pub fn r_vertical_line_to(self, dy: f64) -> Self {
        PathDefinitionString {
            inner_string: format!("{} v {:.2}", self.inner_string, dy),
        }
    }

    /// Appends a curve to a certain point to the [PathDefinitionString], using control point 1 & 2
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Bezier_Curves)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .curve_to((10.0, 10.0), (15.0, 20.0), (20.0, 25.0));
    ///
    /// // Will output "M 3.00 3.00 C 15.00 20.00, 20.00 25.00, 10.00 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 C 15.00 20.00, 20.00 25.00, 10.00 10.00"));
    /// ```
    pub fn curve_to(self, (x, y): Point2D, (cx1, cy1): Point2D, (cx2, cy2): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!(
                "{} C {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2}",
                self.inner_string, cx1, cy1, cx2, cy2, x, y
            ),
        }
    }

    /// Appends a curve to a certain point relative to where the last action ended to the [PathDefinitionString], using control point 1 & 2
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Bezier_Curves)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .r_curve_to((7.0, 7.0), (12.0, 17.0), (17.0, 22.0));
    ///
    /// // Will output "M 3.00 3.00 c 12.00 17.00, 17.00 22.00, 7.00 7.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 c 12.00 17.00, 17.00 22.00, 7.00 7.00"));
    /// ```
    pub fn r_curve_to(
        self,
        (dx, dy): Point2D,
        (cdx1, cdy1): Point2D,
        (cdx2, cdy2): Point2D,
    ) -> Self {
        PathDefinitionString {
            inner_string: format!(
                "{} c {:.2} {:.2}, {:.2} {:.2}, {:.2} {:.2}",
                self.inner_string, cdx1, cdy1, cdx2, cdy2, dx, dy
            ),
        }
    }

    /// Appends a smooth curve (following a normal curve) to a certain point to the [PathDefinitionString], using control point 2
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Bezier_Curves)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .curve_to((10.0, 10.0), (15.0, 20.0), (20.0, 25.0))
    ///     .smooth_curve_to((20.0, 20.0), (-5.0, -10.0));
    ///
    /// // Will output "M 3.00 3.00 C 15.00 20.00, 20.00 25.00, 10.00 10.00 S -5.00 -10.00, 20.00 20.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 C 15.00 20.00, 20.00 25.00, 10.00 10.00 S -5.00 -10.00, 20.00 20.00"));
    /// ```
    pub fn smooth_curve_to(self, (x, y): Point2D, (cx2, cy2): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!(
                "{} S {:.2} {:.2}, {:.2} {:.2}",
                self.inner_string, cx2, cy2, x, y
            ),
        }
    }

    /// Appends a smooth curve (following a normal curve) to a certain point
    /// relative to where the last action ended to the [PathDefinitionString], using control point 2
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Bezier_Curves)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .r_curve_to((10.0, 10.0), (15.0, 20.0), (20.0, 25.0))
    ///     .r_smooth_curve_to((20.0, 20.0), (-5.0, -10.0));
    ///
    /// // Will output "M 3.00 3.00 c 15.00 20.00, 20.00 25.00, 10.00 10.00 s -5.00 -10.00, 20.00 20.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 c 15.00 20.00, 20.00 25.00, 10.00 10.00 s -5.00 -10.00, 20.00 20.00"));
    /// ```
    pub fn r_smooth_curve_to(self, (dx, dy): Point2D, (cdx2, cdy2): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!(
                "{} s {:.2} {:.2}, {:.2} {:.2}",
                self.inner_string, cdx2, cdy2, dx, dy
            ),
        }
    }

    /// Appends a quadratic curve to a certain point
    /// to the [PathDefinitionString], using control point 1
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Bezier_Curves)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .quad_curve_to((10.0, 10.0), (15.0, 20.0));
    ///
    /// // Will output "M 3.00 3.00 Q 15.00 20.00, 10.00 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 Q 15.00 20.00, 10.00 10.00"));
    /// ```
    pub fn quad_curve_to(self, (x, y): Point2D, (cx1, cy1): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!(
                "{} Q {:.2} {:.2}, {:.2} {:.2}",
                self.inner_string, cx1, cy1, x, y
            ),
        }
    }

    /// Appends a quadratic curve to a certain point
    /// relative to where the last action ended to the [PathDefinitionString], using control point 1
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Bezier_Curves)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .r_quad_curve_to((10.0, 10.0), (15.0, 20.0));
    ///
    /// // Will output "M 3.00 3.00 q 15.00 20.00, 10.00 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 q 15.00 20.00, 10.00 10.00"));
    /// ```
    pub fn r_quad_curve_to(self, (dx, dy): Point2D, (cdx1, cdy1): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!(
                "{} q {:.2} {:.2}, {:.2} {:.2}",
                self.inner_string, cdx1, cdy1, dx, dy
            ),
        }
    }

    /// Appends a quadratic curve (following a quadratic curve) to a certain point
    /// to the [PathDefinitionString],
    /// smoothing out the curve using the previous quadratic curve
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Bezier_Curves)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .quad_curve_to((10.0, 10.0), (15.0, 20.0))
    ///     .quad_string_to((20.0, 20.0));
    ///
    /// // Will output "M 3.00 3.00 Q 15.00 20.00, 10.00 10.00 T 20.00 20.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 Q 15.00 20.00, 10.00 10.00 T 20.00 20.00"));
    /// ```
    pub fn quad_string_to(self, (x, y): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!("{} T {:.2} {:.2}", self.inner_string, x, y),
        }
    }

    /// Appends a quadratic curve (following a quadratic curve) to a certain point
    /// relative to where the last action ended to the [PathDefinitionString],
    /// smoothing out the curve using the previous quadratic curve
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Bezier_Curves)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((3.0, 3.0))
    ///     .r_quad_curve_to((10.0, 10.0), (15.0, 20.0))
    ///     .r_quad_string_to((20.0, 20.0));
    ///
    /// // Will output "M 3.00 3.00 q 15.00 20.00, 10.00 10.00 t 20.00 20.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 3.00 3.00 q 15.00 20.00, 10.00 10.00 t 20.00 20.00"));
    /// ```
    pub fn r_quad_string_to(self, (dx, dy): Point2D) -> Self {
        PathDefinitionString {
            inner_string: format!("{} t {:.2} {:.2}", self.inner_string, dx, dy),
        }
    }

    /// Appends an arc to a certain point to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Arcs)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((5.0, 5.0))
    ///     .arc_to((10.0, 10.0), (4.5, 8.0), 3.14, true, false);
    ///
    /// // Will output "M 5.00 5.00 A 4.50 8.00 3.14 1 0 10.00 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 5.00 5.00 A 4.50 8.00 3.14 1 0 10.00 10.00"));
    /// ```
    pub fn arc_to(
        self,
        (x, y): Point2D,
        (rx, ry): (f64, f64),
        x_axis_rotation: f64,
        large_arc_flag: bool,
        sweep_flag: bool,
    ) -> Self {
        let large_arc_flag = if large_arc_flag { '1' } else { '0' };

        let sweep_flag = if sweep_flag { '1' } else { '0' };

        PathDefinitionString {
            inner_string: format!(
                "{} A {:.2} {:.2} {:.2} {} {} {:.2} {:.2}",
                self.inner_string, rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, x, y
            ),
        }
    }

    /// Appends an arc to a certain point relative to where the last action ended to the [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Arcs)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((5.0, 5.0))
    ///     .r_arc_to((10.0, 10.0), (4.5, 8.0), 3.14, true, false);
    ///
    /// // Will output "M 5.00 5.00 a 4.50 8.00 3.14 1 0 10.00 10.00"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 5.00 5.00 a 4.50 8.00 3.14 1 0 10.00 10.00"));
    /// ```
    pub fn r_arc_to(
        self,
        (dx, dy): Point2D,
        (rx, ry): (f64, f64),
        x_axis_rotation: f64,
        large_arc_flag: bool,
        sweep_flag: bool,
    ) -> Self {
        let large_arc_flag = if large_arc_flag { '1' } else { '0' };

        let sweep_flag = if sweep_flag { '1' } else { '0' };

        PathDefinitionString {
            inner_string: format!(
                "{} a {:.2} {:.2} {:.2} {} {} {:.2} {:.2}",
                self.inner_string, rx, ry, x_axis_rotation, large_arc_flag, sweep_flag, dx, dy
            ),
        }
    }

    /// Closes a [PathDefinitionString]
    ///
    /// # Note / Arguments
    /// For further information: [Look here](https://developer.mozilla.org/en-US/docs/Web/SVG/Tutorial/Paths#Line_commands)
    ///
    /// # Examples
    /// ```
    /// use svg_definitions::prelude::*;
    ///
    /// let path_definition_string = PathString::new()
    ///     .move_to((0.0, 0.0))
    ///     .line_to((10.0, 10.0))
    ///     .line_to((10.0, 0.0))
    ///     .close_path();
    ///
    /// // Will output "M 0.00 0.00 L 10.00 10.00 L 10.00 0.00 Z"
    /// println!("{}", path_definition_string);
    /// # assert!(path_definition_string.is_str("M 0.00 0.00 L 10.00 10.00 L 10.00 0.00 Z"));
    /// ```
    pub fn close_path(self) -> Self {
        PathDefinitionString {
            inner_string: format!("{} Z", self.inner_string),
        }
    }
}

impl fmt::Display for PathDefinitionString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.inner_string.trim_start())
    }
}

impl Clone for PathDefinitionString {
    fn clone(&self) -> PathDefinitionString {
        PathDefinitionString {
            inner_string: self.inner_string.clone(),
        }
    }
}

impl Hash for PathDefinitionString {
    fn hash<T: Hasher>(&self, state: &mut T) {
        self.inner_string.hash(state)
    }
}

#[cfg(test)]
mod tests {
    use super::PathDefinitionString;

    #[test]
    fn test_curves() {
        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .curve_to((10.1, 10.9), (30.5, 15.2), (20.7, 5.8))
            .close_path()
            .is_str("M 5.00 5.00 C 30.50 15.20, 20.70 5.80, 10.10 10.90 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .r_curve_to((10.1, 10.9), (30.5, 15.2), (20.7, 5.8))
            .close_path()
            .is_str("M 5.00 5.00 c 30.50 15.20, 20.70 5.80, 10.10 10.90 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .smooth_curve_to((10.1, 10.9), (20.7, 5.8))
            .close_path()
            .is_str("M 5.00 5.00 S 20.70 5.80, 10.10 10.90 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .r_smooth_curve_to((10.1, 10.9), (20.7, 5.8))
            .close_path()
            .is_str("M 5.00 5.00 s 20.70 5.80, 10.10 10.90 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .quad_curve_to((10.1, 10.9), (20.7, 5.8))
            .close_path()
            .is_str("M 5.00 5.00 Q 20.70 5.80, 10.10 10.90 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .r_quad_curve_to((10.1, 10.9), (20.7, 5.8))
            .close_path()
            .is_str("M 5.00 5.00 q 20.70 5.80, 10.10 10.90 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .quad_string_to((10.1, 10.9))
            .close_path()
            .is_str("M 5.00 5.00 T 10.10 10.90 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .r_quad_string_to((10.1, 10.9))
            .close_path()
            .is_str("M 5.00 5.00 t 10.10 10.90 Z"));
    }

    #[test]
    fn test_lines() {
        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .line_to((10.0, 10.0))
            .close_path()
            .is_str("M 5.00 5.00 L 10.00 10.00 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .r_line_to((5.0, 5.0))
            .close_path()
            .is_str("M 5.00 5.00 l 5.00 5.00 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .horizontal_line_to(10.0)
            .close_path()
            .is_str("M 5.00 5.00 H 10.00 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .r_horizontal_line_to(5.0)
            .close_path()
            .is_str("M 5.00 5.00 h 5.00 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .vertical_line_to(10.0)
            .close_path()
            .is_str("M 5.00 5.00 V 10.00 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .r_vertical_line_to(5.0)
            .close_path()
            .is_str("M 5.00 5.00 v 5.00 Z"));
    }

    #[test]
    fn test_arc() {
        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .arc_to((10.0, 10.0), (4.5, 8.0), 3.14, true, false)
            .close_path()
            .is_str("M 5.00 5.00 A 4.50 8.00 3.14 1 0 10.00 10.00 Z"));

        assert!(PathDefinitionString::new()
            .move_to((5.0, 5.0))
            .r_arc_to((10.0, 10.0), (4.5, 8.0), 3.14, true, false)
            .close_path()
            .is_str("M 5.00 5.00 a 4.50 8.00 3.14 1 0 10.00 10.00 Z"));
    }
}
