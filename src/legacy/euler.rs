use crate::Matrix;
// use crate::Quaternion;

use glib::translate::*;
use std::boxed::Box as Box_;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord)] // Hash
    pub struct Euler(Boxed<ffi::CoglEuler>);

    match fn {
        copy => |ptr| ffi::cogl_euler_copy(mut_override(ptr)),
        free => |ptr| ffi::cogl_euler_free(ptr),
        get_type => || ffi::cogl_euler_get_gtype(),
    }
}

impl Euler {
    /// Initializes `self` to represent a rotation of `x_angle` degrees
    /// around the x axis, then `y_angle` degrees around the y_axis and
    /// `z_angle` degrees around the z axis.
    ///
    /// ## `heading`
    /// Angle to rotate around an object's y axis
    /// ## `pitch`
    /// Angle to rotate around an object's x axis
    /// ## `roll`
    /// Angle to rotate around an object's z axis
    pub fn init(&mut self, heading: f32, pitch: f32, roll: f32) {
        unsafe {
            ffi::cogl_euler_init(self.to_glib_none_mut().0, heading, pitch, roll);
        }
    }

    /// Extracts a euler rotation from the given `matrix` and
    /// initializses `self` with the component x, y and z rotation angles.
    ///
    /// ## `matrix`
    /// A `Matrix` containing a rotation, but no scaling,
    ///  mirroring or skewing.
    pub fn init_from_matrix(&mut self, matrix: &Matrix) {
        unsafe {
            ffi::cogl_euler_init_from_matrix(self.to_glib_none_mut().0, matrix.to_glib_none().0);
        }
    }

    // /// Initializes a `self` rotation with the equivalent rotation
    // /// represented by the given `quaternion`.
    // ///
    // /// ## `quaternion`
    // /// A `Euler` with the rotation to initialize with
    // pub fn init_from_quaternion(&mut self, quaternion: &Quaternion) {
    //     unsafe {
    //         ffi::cogl_euler_init_from_quaternion(
    //             self.to_glib_none_mut().0,
    //             quaternion.to_glib_none().0,
    //         );
    //     }
    // }

    fn equal(v1: &Self, v2: &Self) -> bool {
        let a = Box_::into_raw(Box::new(v1)) as *mut _;
        let b = Box_::into_raw(Box::new(v2)) as *mut _;
        unsafe { ffi::cogl_euler_equal(a, b) == crate::TRUE }
    }
}

impl PartialEq for Euler {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Euler::equal(self, other)
    }
}

impl Eq for Euler {}
