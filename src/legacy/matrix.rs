#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use crate::{Euler, Quaternion};

use glib::translate::*;
use std::boxed::Box as Box_;
use std::mem;

glib_wrapper! {
    #[derive(Debug, PartialOrd, Ord)] // Hash
    pub struct Matrix(Boxed<ffi::CoglMatrix>);

    match fn {
        copy => |ptr| ffi::cogl_matrix_copy(mut_override(ptr)),
        free => |ptr| ffi::cogl_matrix_free(ptr),
        get_type => || ffi::cogl_matrix_get_gtype(),
    }
}

impl Matrix {
    /// Multiplies `self` by the given frustum perspective matrix.
    /// ## `left`
    /// X position of the left clipping plane where it
    ///  intersects the near clipping plane
    /// ## `right`
    /// X position of the right clipping plane where it
    ///  intersects the near clipping plane
    /// ## `bottom`
    /// Y position of the bottom clipping plane where it
    ///  intersects the near clipping plane
    /// ## `top`
    /// Y position of the top clipping plane where it intersects
    ///  the near clipping plane
    /// ## `z_near`
    /// The distance to the near clipping plane (Must be positive)
    /// ## `z_far`
    /// The distance to the far clipping plane (Must be positive)
    pub fn frustum(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        z_near: f32,
        z_far: f32,
    ) {
        unsafe {
            ffi::cogl_matrix_frustum(
                self.to_glib_none_mut().0,
                left,
                right,
                bottom,
                top,
                z_near,
                z_far,
            );
        }
    }

    //TODO:
    // /// Casts `self` to a float array which can be directly passed to OpenGL.
    // ///
    // /// # Returns
    // ///
    // /// a pointer to the float array
    // pub fn get_array(&self) -> &[f32] {
    //     unsafe { ffi::cogl_matrix_get_array(self.to_glib_none().0) };
    // }

    /// Gets the inverse transform of a given matrix and uses it to initialize
    /// a new `Matrix`.
    ///
    /// `<note>`Although the first parameter is annotated as const to indicate
    /// that the transform it represents isn't modified this function may
    /// technically save a copy of the inverse transform within the given
    /// `Matrix` so that subsequent requests for the inverse transform may
    /// avoid costly inversion calculations.`</note>`
    /// ## `inverse`
    /// The destination for a 4x4 inverse transformation matrix
    ///
    /// # Returns
    ///
    /// `true` if the inverse was successfully calculated or `false`
    ///  for degenerate transformations that can't be inverted (in this case the
    ///  `inverse` matrix will simply be initialized with the identity matrix)
    pub fn get_inverse(&self) -> (bool, Matrix) {
        unsafe {
            let mut inverse = Matrix::uninitialized();
            let ret =
                ffi::cogl_matrix_get_inverse(self.to_glib_none().0, inverse.to_glib_none_mut().0);
            (ret == crate::TRUE, inverse)
        }
    }

    /// Initializes `self` with the contents of `array`
    /// ## `array`
    /// A linear array of 16 floats (column-major order)
    pub fn init_from_array(&mut self, array: &[f32]) {
        unsafe {
            ffi::cogl_matrix_init_from_array(self.to_glib_none_mut().0, array.as_ptr());
        }
    }

    /// Initializes `self` from a `Euler` rotation.
    ///
    /// ## `euler`
    /// A `Euler`
    pub fn init_from_euler(&mut self, euler: &Euler) {
        unsafe {
            ffi::cogl_matrix_init_from_euler(self.to_glib_none_mut().0, euler.to_glib_none().0);
        }
    }

    /// Initializes `self` from a `Quaternion` rotation.
    /// ## `quaternion`
    /// A `Quaternion`
    pub fn init_from_quaternion(&mut self, quaternion: &Quaternion) {
        unsafe {
            ffi::cogl_matrix_init_from_quaternion(
                self.to_glib_none_mut().0,
                quaternion.to_glib_none().0,
            );
        }
    }

    /// Resets matrix to the identity matrix:
    ///
    ///
    /// ```text
    ///   .xx=1; .xy=0; .xz=0; .xw=0;
    ///   .yx=0; .yy=1; .yz=0; .yw=0;
    ///   .zx=0; .zy=0; .zz=1; .zw=0;
    ///   .wx=0; .wy=0; .wz=0; .ww=1;
    /// ```
    pub fn init_identity(&mut self) {
        unsafe {
            ffi::cogl_matrix_init_identity(self.to_glib_none_mut().0);
        }
    }

    /// Resets matrix to the (tx, ty, tz) translation matrix:
    ///
    ///
    /// ```text
    ///   .xx=1; .xy=0; .xz=0; .xw=tx;
    ///   .yx=0; .yy=1; .yz=0; .yw=ty;
    ///   .zx=0; .zy=0; .zz=1; .zw=tz;
    ///   .wx=0; .wy=0; .wz=0; .ww=1;
    /// ```
    ///
    /// ## `tx`
    /// x coordinate of the translation vector
    /// ## `ty`
    /// y coordinate of the translation vector
    /// ## `tz`
    /// z coordinate of the translation vector
    pub fn init_translation(&mut self, tx: f32, ty: f32, tz: f32) {
        unsafe {
            ffi::cogl_matrix_init_translation(self.to_glib_none_mut().0, tx, ty, tz);
        }
    }

    /// Determines if the given matrix is an identity matrix.
    ///
    /// # Returns
    ///
    /// `true` if `self` is an identity matrix else `false`
    pub fn is_identity(&self) -> bool {
        unsafe { ffi::cogl_matrix_is_identity(self.to_glib_none().0) == crate::TRUE }
    }

    /// Applies a view transform `self` that positions the camera at
    /// the coordinate (`eye_position_x`, `eye_position_y`, `eye_position_z`)
    /// looking towards an object at the coordinate (`object_x`, `object_y`,
    /// `object_z`). The top of the camera is aligned to the given world up
    /// vector, which is normally simply (0, 1, 0) to map up to the
    /// positive direction of the y axis.
    ///
    /// Because there is a lot of missleading documentation online for
    /// gluLookAt regarding the up vector we want to try and be a bit
    /// clearer here.
    ///
    /// The up vector should simply be relative to your world coordinates
    /// and does not need to change as you move the eye and object
    /// positions. Many online sources may claim that the up vector needs
    /// to be perpendicular to the vector between the eye and object
    /// position (partly because the man page is somewhat missleading) but
    /// that is not necessary for this function.
    ///
    /// `<note>`You should never look directly along the world-up
    /// vector.`</note>`
    ///
    /// `<note>`It is assumed you are using a typical projection matrix where
    /// your origin maps to the center of your viewport.`</note>`
    ///
    /// `<note>`Almost always when you use this function it should be the first
    /// transform applied to a new modelview transform`</note>`
    /// ## `eye_position_x`
    /// The X coordinate to look from
    /// ## `eye_position_y`
    /// The Y coordinate to look from
    /// ## `eye_position_z`
    /// The Z coordinate to look from
    /// ## `object_x`
    /// The X coordinate of the object to look at
    /// ## `object_y`
    /// The Y coordinate of the object to look at
    /// ## `object_z`
    /// The Z coordinate of the object to look at
    /// ## `world_up_x`
    /// The X component of the world's up direction vector
    /// ## `world_up_y`
    /// The Y component of the world's up direction vector
    /// ## `world_up_z`
    /// The Z component of the world's up direction vector
    pub fn look_at(
        &mut self,
        eye_position_x: f32,
        eye_position_y: f32,
        eye_position_z: f32,
        object_x: f32,
        object_y: f32,
        object_z: f32,
        world_up_x: f32,
        world_up_y: f32,
        world_up_z: f32,
    ) {
        unsafe {
            ffi::cogl_matrix_look_at(
                self.to_glib_none_mut().0,
                eye_position_x,
                eye_position_y,
                eye_position_z,
                object_x,
                object_y,
                object_z,
                world_up_x,
                world_up_y,
                world_up_z,
            );
        }
    }

    /// Multiplies the two supplied matrices together and stores
    /// the resulting matrix inside `self`.
    ///
    /// `<note>`It is possible to multiply the `a` matrix in-place, so
    /// `self` can be equal to `a` but can't be equal to `b`.`</note>`
    /// ## `a`
    /// A 4x4 transformation matrix
    /// ## `b`
    /// A 4x4 transformation matrix
    pub fn multiply(&mut self, a: &Matrix, b: &Matrix) {
        unsafe {
            ffi::cogl_matrix_multiply(
                self.to_glib_none_mut().0,
                a.to_glib_none().0,
                b.to_glib_none().0,
            );
        }
    }

    /// Multiplies `self` by a parallel projection matrix.
    /// ## `x_1`
    /// The x coordinate for the first vertical clipping plane
    /// ## `y_1`
    /// The y coordinate for the first horizontal clipping plane
    /// ## `x_2`
    /// The x coordinate for the second vertical clipping plane
    /// ## `y_2`
    /// The y coordinate for the second horizontal clipping plane
    /// ## `near`
    /// The `<emphasis>`distance`</emphasis>` to the near clipping
    ///  plane (will be `<emphasis>`negative`</emphasis>` if the plane is
    ///  behind the viewer)
    /// ## `far`
    /// The `<emphasis>`distance`</emphasis>` to the far clipping
    ///  plane (will be `<emphasis>`negative`</emphasis>` if the plane is
    ///  behind the viewer)
    pub fn orthographic(&mut self, x_1: f32, y_1: f32, x_2: f32, y_2: f32, near: f32, far: f32) {
        unsafe {
            ffi::cogl_matrix_orthographic(self.to_glib_none_mut().0, x_1, y_1, x_2, y_2, near, far);
        }
    }

    /// Multiplies `self` by the described perspective matrix
    ///
    /// `<note>`You should be careful not to have to great a `z_far` / `z_near`
    /// ratio since that will reduce the effectiveness of depth testing
    /// since there wont be enough precision to identify the depth of
    /// objects near to each other.`</note>`
    /// ## `fov_y`
    /// Vertical field of view angle in degrees.
    /// ## `aspect`
    /// The (width over height) aspect ratio for display
    /// ## `z_near`
    /// The distance to the near clipping plane (Must be positive,
    ///  and must not be 0)
    /// ## `z_far`
    /// The distance to the far clipping plane (Must be positive)
    pub fn perspective(&mut self, fov_y: f32, aspect: f32, z_near: f32, z_far: f32) {
        unsafe {
            ffi::cogl_matrix_perspective(self.to_glib_none_mut().0, fov_y, aspect, z_near, z_far);
        }
    }

    //pub fn project_points(&self, n_components: i32, stride_in: usize, points_in: /*Unimplemented*/Option<Fundamental: Pointer>, stride_out: usize, points_out: /*Unimplemented*/Option<Fundamental: Pointer>, n_points: i32) {
    //    unsafe { TODO: call cogl_sys:cogl_matrix_project_points() }
    //}

    /// Multiplies `self` with a rotation matrix that applies a rotation
    /// of `angle` degrees around the specified 3D vector.
    /// ## `angle`
    /// The angle you want to rotate in degrees
    /// ## `x`
    /// X component of your rotation vector
    /// ## `y`
    /// Y component of your rotation vector
    /// ## `z`
    /// Z component of your rotation vector
    pub fn rotate(&mut self, angle: f32, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_matrix_rotate(self.to_glib_none_mut().0, angle, x, y, z);
        }
    }

    /// Multiplies `self` with a rotation transformation described by the
    /// given `Euler`.
    ///
    /// ## `euler`
    /// A euler describing a rotation
    pub fn rotate_euler(&mut self, euler: &Euler) {
        unsafe {
            ffi::cogl_matrix_rotate_euler(self.to_glib_none_mut().0, euler.to_glib_none().0);
        }
    }

    /// Multiplies `self` with a rotation transformation described by the
    /// given `Quaternion`.
    ///
    /// ## `quaternion`
    /// A quaternion describing a rotation
    pub fn rotate_quaternion(&mut self, quaternion: &Quaternion) {
        unsafe {
            ffi::cogl_matrix_rotate_quaternion(
                self.to_glib_none_mut().0,
                quaternion.to_glib_none().0,
            );
        }
    }

    /// Multiplies `self` with a transform matrix that scales along the X,
    /// Y and Z axis.
    /// ## `sx`
    /// The X scale factor
    /// ## `sy`
    /// The Y scale factor
    /// ## `sz`
    /// The Z scale factor
    pub fn scale(&mut self, sx: f32, sy: f32, sz: f32) {
        unsafe {
            ffi::cogl_matrix_scale(self.to_glib_none_mut().0, sx, sy, sz);
        }
    }

    /// Transforms a point whos position is given and returned as four float
    /// components.
    /// ## `x`
    /// The X component of your points position
    /// ## `y`
    /// The Y component of your points position
    /// ## `z`
    /// The Z component of your points position
    /// ## `w`
    /// The W component of your points position
    pub fn transform_point(&self, x: &mut f32, y: &mut f32, z: &mut f32, w: &mut f32) {
        unsafe {
            ffi::cogl_matrix_transform_point(self.to_glib_none().0, x, y, z, w);
        }
    }

    //pub fn transform_points(&self, n_components: i32, stride_in: usize, points_in: /*Unimplemented*/Option<Fundamental: Pointer>, stride_out: usize, points_out: /*Unimplemented*/Option<Fundamental: Pointer>, n_points: i32) {
    //    unsafe { TODO: call cogl_sys:cogl_matrix_transform_points() }
    //}

    /// Multiplies `self` with a transform matrix that translates along
    /// the X, Y and Z axis.
    /// ## `x`
    /// The X translation you want to apply
    /// ## `y`
    /// The Y translation you want to apply
    /// ## `z`
    /// The Z translation you want to apply
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_matrix_translate(self.to_glib_none_mut().0, x, y, z);
        }
    }

    /// Replaces `self` with its transpose. Ie, every element (i,j) in the
    /// new matrix is taken from element (j,i) in the old matrix.
    pub fn transpose(&mut self) {
        unsafe {
            ffi::cogl_matrix_transpose(self.to_glib_none_mut().0);
        }
    }

    /// Multiplies `self` by a view transform that maps the 2D coordinates
    /// (0,0) top left and (`width_2d`,`height_2d`) bottom right the full viewport
    /// size. Geometry at a depth of 0 will now lie on this 2D plane.
    ///
    /// Note: this doesn't multiply the matrix by any projection matrix,
    /// but it assumes you have a perspective projection as defined by
    /// passing the corresponding arguments to `Matrix::frustum`.
    ///
    /// Toolkits such as Clutter that mix 2D and 3D drawing can use this to
    /// create a 2D coordinate system within a 3D perspective projected
    /// view frustum.
    /// ## `left`
    /// coord of left vertical clipping plane
    /// ## `right`
    /// coord of right vertical clipping plane
    /// ## `bottom`
    /// coord of bottom horizontal clipping plane
    /// ## `top`
    /// coord of top horizontal clipping plane
    /// ## `z_near`
    /// The distance to the near clip plane. Never pass 0 and always pass
    ///  a positive number.
    /// ## `z_2d`
    /// The distance to the 2D plane. (Should always be positive and
    ///  be between `z_near` and the z_far value that was passed to
    ///  `Matrix::frustum`)
    /// ## `width_2d`
    /// The width of the 2D coordinate system
    /// ## `height_2d`
    /// The height of the 2D coordinate system
    pub fn view_2d_in_frustum(
        &mut self,
        left: f32,
        right: f32,
        bottom: f32,
        top: f32,
        z_near: f32,
        z_2d: f32,
        width_2d: f32,
        height_2d: f32,
    ) {
        unsafe {
            ffi::cogl_matrix_view_2d_in_frustum(
                self.to_glib_none_mut().0,
                left,
                right,
                bottom,
                top,
                z_near,
                z_2d,
                width_2d,
                height_2d,
            );
        }
    }

    /// Multiplies `self` by a view transform that maps the 2D coordinates
    /// (0,0) top left and (`width_2d`,`height_2d`) bottom right the full viewport
    /// size. Geometry at a depth of 0 will now lie on this 2D plane.
    ///
    /// Note: this doesn't multiply the matrix by any projection matrix,
    /// but it assumes you have a perspective projection as defined by
    /// passing the corresponding arguments to `Matrix::perspective`.
    ///
    /// Toolkits such as Clutter that mix 2D and 3D drawing can use this to
    /// create a 2D coordinate system within a 3D perspective projected
    /// view frustum.
    /// ## `fov_y`
    /// A field of view angle for the Y axis
    /// ## `aspect`
    /// The ratio of width to height determining the field of view angle
    ///  for the x axis.
    /// ## `z_near`
    /// The distance to the near clip plane. Never pass 0 and always pass
    ///  a positive number.
    /// ## `z_2d`
    /// The distance to the 2D plane. (Should always be positive and
    ///  be between `z_near` and the z_far value that was passed to
    ///  `Matrix::frustum`)
    /// ## `width_2d`
    /// The width of the 2D coordinate system
    /// ## `height_2d`
    /// The height of the 2D coordinate system
    pub fn view_2d_in_perspective(
        &mut self,
        fov_y: f32,
        aspect: f32,
        z_near: f32,
        z_2d: f32,
        width_2d: f32,
        height_2d: f32,
    ) {
        unsafe {
            ffi::cogl_matrix_view_2d_in_perspective(
                self.to_glib_none_mut().0,
                fov_y,
                aspect,
                z_near,
                z_2d,
                width_2d,
                height_2d,
            );
        }
    }

    fn equal(v1: &Self, v2: &Self) -> bool {
        let a = Box_::into_raw(Box::new(v1)) as *mut _;
        let b = Box_::into_raw(Box::new(v2)) as *mut _;
        unsafe { ffi::cogl_matrix_equal(a, b) == crate::TRUE }
    }
}

#[doc(hidden)]
impl Uninitialized for Matrix {
    #[inline]
    unsafe fn uninitialized() -> Self {
        mem::zeroed()
    }
}

impl PartialEq for Matrix {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Matrix::equal(self, other)
    }
}

impl Eq for Matrix {}
