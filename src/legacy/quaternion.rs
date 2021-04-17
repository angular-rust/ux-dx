#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::{Euler, Matrix};

use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord)] // Hash
    pub struct Quaternion(Boxed<ffi::CoglQuaternion>);

    match fn {
        copy => |ptr| ffi::cogl_quaternion_copy(mut_override(ptr)),
        free => |ptr| ffi::cogl_quaternion_free(ptr),
        get_type => || ffi::cogl_quaternion_get_gtype(),
    }
}

impl Quaternion {
    ///
    /// ## `b`
    /// A `Quaternion`
    pub fn dot_product(&self, b: &Quaternion) -> f32 {
        unsafe { ffi::cogl_quaternion_dot_product(self.to_glib_none().0, b.to_glib_none().0) }
    }

    ///
    pub fn get_rotation_angle(&self) -> f32 {
        unsafe { ffi::cogl_quaternion_get_rotation_angle(self.to_glib_none().0) }
    }

    ///
    /// ## `vector3`
    /// an allocated 3-float array
    pub fn get_rotation_axis(&self) -> f32 {
        unsafe {
            let mut vector3 = mem::MaybeUninit::uninit();
            ffi::cogl_quaternion_get_rotation_axis(self.to_glib_none().0, vector3.as_mut_ptr());
            vector3.assume_init()
        }
    }

    /// Initializes a quaternion that rotates `angle` degrees around the
    /// axis vector (`x`, `y`, `z`). The axis vector does not need to be
    /// normalized.
    ///
    /// ## `angle`
    /// The angle you want to rotate around the given axis
    /// ## `x`
    /// The x component of your axis vector about which you want to
    /// rotate.
    /// ## `y`
    /// The y component of your axis vector about which you want to
    /// rotate.
    /// ## `z`
    /// The z component of your axis vector about which you want to
    /// rotate.
    pub fn init(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_quaternion_init(self.to_glib_none_mut().0, angle, x, y, z);
        }
    }

    /// Initializes a quaternion that rotates `angle` degrees around the
    /// given `axis` vector. The axis vector does not need to be
    /// normalized.
    ///
    /// ## `angle`
    /// The angle to rotate around `axis3f`
    /// ## `axis3f`
    /// your 3 component axis vector about which you want to rotate.
    pub fn init_from_angle_vector(&mut self, angle: f32, axis3f: &[f32; 3]) {
        unsafe {
            ffi::cogl_quaternion_init_from_angle_vector(
                self.to_glib_none_mut().0,
                angle,
                axis3f.to_glib_none().0,
            );
        }
    }

    /// Initializes a [w (x, y,z)] quaternion directly from an array of 4
    /// floats: [w,x,y,z].
    ///
    /// ## `array`
    /// An array of 4 floats w,(x,y,z)
    pub fn init_from_array(&mut self, array: &[f32]) {
        unsafe {
            ffi::cogl_quaternion_init_from_array(self.to_glib_none_mut().0, array.as_ptr());
        }
    }

    ///
    /// ## `euler`
    /// A `Euler` with which to initialize the quaternion
    pub fn init_from_euler(&mut self, euler: &Euler) {
        unsafe {
            ffi::cogl_quaternion_init_from_euler(self.to_glib_none_mut().0, euler.to_glib_none().0);
        }
    }

    /// Initializes a quaternion from a rotation matrix.
    /// ## `matrix`
    /// A rotation matrix with which to initialize the quaternion
    pub fn init_from_matrix(&mut self, matrix: &Matrix) {
        unsafe {
            ffi::cogl_quaternion_init_from_matrix(
                self.to_glib_none_mut().0,
                matrix.to_glib_none().0,
            );
        }
    }

    ///
    /// ## `src`
    /// A `Quaternion` with which to initialize `self`
    pub fn init_from_quaternion(&mut self, src: &mut Quaternion) {
        unsafe {
            ffi::cogl_quaternion_init_from_quaternion(
                self.to_glib_none_mut().0,
                src.to_glib_none_mut().0,
            );
        }
    }

    /// XXX: check which direction this rotates
    ///
    /// ## `angle`
    /// The angle to rotate around the x axis
    pub fn init_from_x_rotation(&mut self, angle: f32) {
        unsafe {
            ffi::cogl_quaternion_init_from_x_rotation(self.to_glib_none_mut().0, angle);
        }
    }

    ///
    /// ## `angle`
    /// The angle to rotate around the y axis
    pub fn init_from_y_rotation(&mut self, angle: f32) {
        unsafe {
            ffi::cogl_quaternion_init_from_y_rotation(self.to_glib_none_mut().0, angle);
        }
    }

    ///
    /// ## `angle`
    /// The angle to rotate around the z axis
    pub fn init_from_z_rotation(&mut self, angle: f32) {
        unsafe {
            ffi::cogl_quaternion_init_from_z_rotation(self.to_glib_none_mut().0, angle);
        }
    }

    /// Initializes the quaternion with the canonical quaternion identity
    /// [1 (0, 0, 0)] which represents no rotation. Multiplying a
    /// quaternion with this identity leaves the quaternion unchanged.
    ///
    /// You might also want to consider using
    /// `get_static_identity_quaternion`.
    ///
    pub fn init_identity(&mut self) {
        unsafe {
            ffi::cogl_quaternion_init_identity(self.to_glib_none_mut().0);
        }
    }

    ///
    pub fn invert(&mut self) {
        unsafe {
            ffi::cogl_quaternion_invert(self.to_glib_none_mut().0);
        }
    }

    /// This combines the rotations of two quaternions into `self`. The
    /// operation is not commutative so the order is important because AxB
    /// != BxA. Cogl follows the standard convention for quaternions here
    /// so the rotations are applied `right` to `left`. This is similar to the
    /// combining of matrices.
    ///
    /// `<note>`It is possible to multiply the `a` quaternion in-place, so
    /// `self` can be equal to `a` but can't be equal to `b`.`</note>`
    ///
    /// ## `left`
    /// The second `Quaternion` rotation to apply
    /// ## `right`
    /// The first `Quaternion` rotation to apply
    pub fn multiply(&mut self, left: &Quaternion, right: &Quaternion) {
        unsafe {
            ffi::cogl_quaternion_multiply(
                self.to_glib_none_mut().0,
                left.to_glib_none().0,
                right.to_glib_none().0,
            );
        }
    }

    /// Performs a normalized linear interpolation between two quaternions.
    /// That is it does a linear interpolation of the quaternion components
    /// and then normalizes the result. This will follow the shortest arc
    /// between the two orientations (just like the `slerp` function) but
    /// will not progress at a constant speed. Unlike `slerp` nlerp is
    /// commutative which is useful if you are blending animations
    /// together. (I.e. nlerp (tmp, a, b) followed by nlerp (result, tmp,
    /// d) is the same as nlerp (tmp, a, d) followed by nlerp (result, tmp,
    /// b)). Finally nlerp is cheaper than slerp so it can be a good choice
    /// if you don't need the constant speed property of the `slerp` function.
    ///
    /// Notable properties:
    /// `<itemizedlist>`
    /// `<listitem>`
    /// commutative: Yes
    /// `</listitem>`
    /// `<listitem>`
    /// constant velocity: No
    /// `</listitem>`
    /// `<listitem>`
    /// torque minimal (travels along the surface of the 4-sphere): Yes
    /// `</listitem>`
    /// `<listitem>`
    /// faster than `Quaternion::slerp`
    /// `</listitem>`
    /// `</itemizedlist>`
    /// ## `a`
    /// The first `Quaternion`
    /// ## `b`
    /// The second `Quaternion`
    /// ## `t`
    /// The factor in the range [0,1] used to interpolate between
    /// quaterion `a` and `b`.
    pub fn nlerp(&mut self, a: &Quaternion, b: &Quaternion, t: f32) {
        unsafe {
            ffi::cogl_quaternion_nlerp(
                self.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                t,
            );
        }
    }

    ///
    pub fn normalize(&mut self) {
        unsafe {
            ffi::cogl_quaternion_normalize(self.to_glib_none_mut().0);
        }
    }

    ///
    /// ## `exponent`
    /// the exponent
    pub fn pow(&mut self, exponent: f32) {
        unsafe {
            ffi::cogl_quaternion_pow(self.to_glib_none_mut().0, exponent);
        }
    }

    /// Performs a spherical linear interpolation between two quaternions.
    ///
    /// Noteable properties:
    /// `<itemizedlist>`
    /// `<listitem>`
    /// commutative: No
    /// `</listitem>`
    /// `<listitem>`
    /// constant velocity: Yes
    /// `</listitem>`
    /// `<listitem>`
    /// torque minimal (travels along the surface of the 4-sphere): Yes
    /// `</listitem>`
    /// `<listitem>`
    /// more expensive than `Quaternion::nlerp`
    /// `</listitem>`
    /// `</itemizedlist>`
    /// ## `a`
    /// The first `Quaternion`
    /// ## `b`
    /// The second `Quaternion`
    /// ## `t`
    /// The factor in the range [0,1] used to interpolate between
    /// quaternion `a` and `b`.
    pub fn slerp(&mut self, a: &Quaternion, b: &Quaternion, t: f32) {
        unsafe {
            ffi::cogl_quaternion_slerp(
                self.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                t,
            );
        }
    }

    ///
    /// ## `prev`
    /// A `Quaternion` used before `a`
    /// ## `a`
    /// The first `Quaternion`
    /// ## `b`
    /// The second `Quaternion`
    /// ## `next`
    /// A `Quaternion` that will be used after `b`
    /// ## `t`
    /// The factor in the range [0,1] used to interpolate between
    /// quaternion `a` and `b`.
    pub fn squad(
        &mut self,
        prev: &Quaternion,
        a: &Quaternion,
        b: &Quaternion,
        next: &Quaternion,
        t: f32,
    ) {
        unsafe {
            ffi::cogl_quaternion_squad(
                self.to_glib_none_mut().0,
                prev.to_glib_none().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
                next.to_glib_none().0,
                t,
            );
        }
    }

    fn equal(v1: &Self, v2: &Self) -> bool {
        let a = Box_::into_raw(Box::new(v1)) as *mut _;
        let b = Box_::into_raw(Box::new(v2)) as *mut _;
        unsafe { ffi::cogl_quaternion_equal(a, b) == crate::TRUE }
    }
}

impl PartialEq for Quaternion {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Quaternion::equal(self, other)
    }
}

impl Eq for Quaternion {}
