use std::f32::consts::PI;

// use std::f32::consts::{E, FRAC_1_SQRT_2, LN_10, LN_2, LOG10_E, LOG2_E, PI, SQRT_2};

/// Some handy math functions, and inlinable constants.
pub struct Math {}

impl Math {
    pub const SQRT1_2: f32 = std::f32::consts::FRAC_1_SQRT_2; // 0.7071067811865476; // FRAC_1_SQRT_2

    // doesn't specify the size of an int or float, in practice it's 32 bits
    /// The lowest integer value.
    pub const INT_MIN: i32 = -2147483648;

    /// The highest integer value.
    pub const INT_MAX: i32 = 2147483647;

    /// The lowest float value.
    pub const FLOAT_MIN: f32 = std::f32::MIN; //-1.79769313486231e+308;

    /// The highest float value.

    pub const FLOAT_MAX: f32 = std::f32::MAX; //1.79769313486231e+308;

    /// Converts an angle in degrees to radians.
    // static
    #[inline]
    pub fn to_radians(degrees: f32) -> f32 {
        degrees * PI / 180.0
    }

    /// Converts an angle in radians to degrees.
    // static
    #[inline]
    pub fn to_degrees(radians: f32) -> f32 {
        radians * 180.0 / PI
    }

    // // static
    // #[inline]
    // pub fn max<T: f32>(a: T, b: T) -> T {
    //     if a > b {
    //         a
    //     } else {
    //         b
    //     }
    // }

    // // static
    // #[inline]
    // pub fn min<T: f32>(a: T, b: T) -> T {
    //     if a < b {
    //         a
    //     } else {
    //         b
    //     }
    // }

    // // static
    // pub fn clamp<T: f32>(value: T, min: T, max: T) -> T {
    //     if value < min {
    //         min
    //     } else if value > max {
    //         max
    //     } else {
    //         value;
    //     }
    // }

    // static
    pub fn sign(value: f32) -> i32 {
        if value < 0.0 {
            -1
        } else if value > 0.0 {
            1
        } else {
            0
        }
    }
}
