pub const SIGNS: [i32; 2] = [-1, 1];
pub const BOOLEANS: [bool; 2] = [false, true];
pub const FLOAT_ROUNDING_ERROR: f32 = 0.000001;
pub const PI: f32 = 3.1415927;
pub const HALF_PI: f32 = PI / 2.0;
pub const PI2: f32 = PI * 2.0;
pub const E: f32 = 2.7182818;
pub const SQRT2: f32 = 1.4142136;
pub const SQRT3: f32 = 1.7320508;
pub const RADIANS_TO_DEGREES: f32 = 180.0 / PI;
pub const RAD_DEG: f32 = RADIANS_TO_DEGREES;
pub const DEGREES_TO_RADIANS: f32 = PI / 180.0;
pub const DEG_RAD: f32 = DEGREES_TO_RADIANS;
pub const DOUBLE_DEG_RAD: f64 = 0.017453292519943295;
pub const DOUBLE_RAD_DEG: f64 = 57.29577951308232;
pub const SIN_BITS: i32 = 14; // 16KB. Adjust for accuracy.
pub const SIN_MASK: i32 = !(-1 << SIN_BITS);
pub const SIN_COUNT: i32 = SIN_MASK + 1;


pub fn slope(a: f32) -> f32 {
    1.0 - (a - 0.5).abs() * 2.0
}

pub fn clamp(value: f32, min: f32, max: f32) -> f32 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn zero(f: f32, tolerance: Option<f32>) ->bool {
    f.abs() < tolerance.unwrap_or(FLOAT_ROUNDING_ERROR)
}

/// Mod function that works properly for negative numbers.
pub fn modulo(x: i32, n: i32) -> i32 {
    ((x % n) + n) % n
}