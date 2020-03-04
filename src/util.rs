pub mod internal_ratio;

/// formats number to 2 points of precision
pub fn prec_num(num: f32) -> String {
    format!("{:.2}", num)
}

/// round to the number
pub fn round_num(num: f32) -> i32 {
    num.round() as i32
}