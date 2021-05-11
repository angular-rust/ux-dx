use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct Fixed(Object<ffi::CoglFixed, FixedClass>);

    match fn {
        get_type => || ffi::cogl_fixed_get_type(),
    }
}

impl Fixed {
    // TODO: remade all it here coz
    // typedef int32_t CoglFixed;

    // /// Computes the arc tangent of `self`.
    // ///
    // /// # Returns
    // ///
    // /// the arc tangent of the passed value, in fixed point notation
    // pub fn atan(&self) -> Option<Fixed> {
    //     let a: ffi::CoglFixed = self.to_glib_none().0;
    //     unsafe {
    //         from_glib_none(ffi::cogl_fixed_atan(self.to_glib_none().0))
    //     }
    // }

    // /// Computes the arc tangent of `self` / `b` but uses the sign of both
    // /// arguments to return the angle in right quadrant.
    // /// ## `b`
    // /// the denominator as a `Fixed` number
    // ///
    // /// # Returns
    // ///
    // /// the arc tangent of the passed fraction, in fixed point
    // ///  notation
    // pub fn atan2(&self, b: &Fixed) -> Option<Fixed> {
    //     unsafe {
    //         from_glib_none(ffi::cogl_fixed_atan2(self.to_glib_none().0, b.to_glib_none().0))
    //     }
    // }

    // /// Computes the cosine of `self`.
    // ///
    // /// # Returns
    // ///
    // /// the cosine of the passed angle, in fixed point notation
    // pub fn cos(&self) -> Option<Fixed> {
    //     unsafe {
    //         from_glib_none(ffi::cogl_fixed_cos(self.to_glib_none().0))
    //     }
    // }

    // /// Calculates 2 to the `self` power.
    // ///
    // /// This function is around 11 times faster on x86, and around 22 times faster
    // /// on fpu-less arm than libc pow(2, x).
    // ///
    // /// # Returns
    // ///
    // /// the power of 2 to the passed value
    // pub fn pow2(&self) -> u32 {
    //     unsafe {
    //         ffi::cogl_fixed_pow2(self.to_glib_none().0)
    //     }
    // }

    // /// Computes the sine of `self`.
    // ///
    // /// # Returns
    // ///
    // /// the sine of the passed angle, in fixed point notation
    // pub fn sin(&self) -> Option<Fixed> {
    //     unsafe {
    //         from_glib_none(ffi::cogl_fixed_sin(self.to_glib_none().0))
    //     }
    // }

    // /// Computes the square root of `self`.
    // ///
    // /// # Returns
    // ///
    // /// the square root of the passed value, in floating point
    // ///  notation
    // pub fn sqrt(&self) -> Option<Fixed> {
    //     unsafe {
    //         from_glib_none(ffi::cogl_fixed_sqrt(self.to_glib_none().0))
    //     }
    // }

    // /// Computes the tangent of `self`.
    // ///
    // /// # Returns
    // ///
    // /// the tangent of the passed angle, in fixed point notation
    // pub fn tan(&self) -> Option<Fixed> {
    //     unsafe {
    //         from_glib_none(ffi::cogl_fixed_tan(self.to_glib_none().0))
    //     }
    // }

    // /// Calculates base 2 logarithm.
    // ///
    // /// This function is some 2.5 times faster on x86, and over 12 times faster on
    // /// fpu-less arm, than using libc `log`.
    // /// ## `x`
    // /// value to calculate base 2 logarithm from
    // ///
    // /// # Returns
    // ///
    // /// base 2 logarithm.
    // pub fn log2(x: u32) -> Option<Fixed> {
    //     unsafe {
    //         from_glib_none(ffi::cogl_fixed_log2(x))
    //     }
    // }

    // /// Calculates `x` to the `y` power.
    // /// ## `x`
    // /// base
    // /// ## `y`
    // /// `Fixed` exponent
    // ///
    // /// # Returns
    // ///
    // /// the power of `x` to the `y`
    // pub fn pow(x: u32, y: &Fixed) -> u32 {
    //     unsafe {
    //         ffi::cogl_fixed_pow(x, y.to_glib_none().0)
    //     }
    // }
}

impl fmt::Display for Fixed {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Fixed")
    }
}
