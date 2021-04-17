use crate::{Context, Euler, Matrix, MatrixEntry, Object, Quaternion};

use glib::translate::*;
use std::fmt;

glib_wrapper! {
    pub struct MatrixStack(Object<ffi::CoglMatrixStack, MatrixStackClass>) @extends Object;

    match fn {
        get_type => || ffi::cogl_matrix_stack_get_gtype(),
    }
}

impl MatrixStack {
    /// Allocates a new `MatrixStack` that can be used to build up
    /// transformations relating to objects in a scenegraph like hierarchy.
    /// (See the description of `MatrixStack` and `MatrixEntry` for
    /// more details of what a matrix stack is best suited for)
    ///
    /// When a `MatrixStack` is first allocated it is conceptually
    /// positioned at the root of your scenegraph hierarchy. As you
    /// traverse your scenegraph then you should call
    /// `MatrixStack::push` whenever you move down a level and
    /// `MatrixStack::pop` whenever you move back up a level towards
    /// the root.
    ///
    /// Once you have allocated a `MatrixStack` you can get a reference
    /// to the current transformation for the current position in the
    /// hierarchy by calling `MatrixStack::get_entry`.
    ///
    /// Once you have allocated a `MatrixStack` you can apply operations
    /// such as rotate, scale and translate to modify the current transform
    /// for the current position in the hierarchy by calling
    /// `MatrixStack::rotate`, `MatrixStack::scale` and
    /// `MatrixStack::translate`.
    /// ## `ctx`
    /// A `Context`
    ///
    /// # Returns
    ///
    /// A newly allocated `MatrixStack`
    pub fn new(ctx: &Context) -> MatrixStack {
        unsafe { from_glib_full(ffi::cogl_matrix_stack_new(ctx.to_glib_none().0)) }
    }

    /// Replaces the current matrix with a perspective matrix for a given
    /// viewing frustum defined by 4 side clip planes that all cross
    /// through the origin and 2 near and far clip planes.
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
    pub fn frustum(&self, left: f32, right: f32, bottom: f32, top: f32, z_near: f32, z_far: f32) {
        unsafe {
            ffi::cogl_matrix_stack_frustum(
                self.to_glib_none().0,
                left,
                right,
                bottom,
                top,
                z_near,
                z_far,
            );
        }
    }

    /// Resolves the current `self` transform into a `Matrix` by
    /// combining the operations that have been applied to build up the
    /// current transform.
    ///
    /// There are two possible ways that this function may return its
    /// result depending on whether the stack is able to directly point
    /// to an internal `Matrix` or whether the result needs to be
    /// composed of multiple operations.
    ///
    /// If an internal matrix contains the required result then this
    /// function will directly return a pointer to that matrix, otherwise
    /// if the function returns `None` then `matrix` will be initialized
    /// to match the current transform of `self`.
    ///
    /// `<note>``matrix` will be left untouched if a direct pointer is
    /// returned.`</note>`
    /// ## `matrix`
    /// The potential destination for the current matrix
    ///
    /// # Returns
    ///
    /// A direct pointer to the current transform or `None`
    ///  and in that case `matrix` will be initialized with
    ///  the value of the current transform.
    pub fn get(&self) -> (Matrix, Matrix) {
        unsafe {
            let mut matrix = Matrix::uninitialized();
            let ret = from_glib_full(ffi::cogl_matrix_stack_get(
                self.to_glib_none().0,
                matrix.to_glib_none_mut().0,
            ));
            (ret, matrix)
        }
    }

    /// Gets a reference to the current transform represented by a
    /// `MatrixEntry` pointer.
    ///
    /// `<note>`The transform represented by a `MatrixEntry` is
    /// immutable.`</note>`
    ///
    /// `<note>``MatrixEntry`<!-- -->s are reference counted using
    /// `MatrixEntry::ref` and `MatrixEntry::unref` and you
    /// should call `MatrixEntry::unref` when you are finished with
    /// and entry you get via `MatrixStack::get_entry`.`</note>`
    ///
    /// # Returns
    ///
    /// A pointer to the `MatrixEntry`
    ///  representing the current matrix stack transform.
    pub fn get_entry(&self) -> Option<MatrixEntry> {
        unsafe { from_glib_none(ffi::cogl_matrix_stack_get_entry(self.to_glib_none().0)) }
    }

    /// Gets the inverse transform of the current matrix and uses it to
    /// initialize a new `Matrix`.
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
            let ret = ffi::cogl_matrix_stack_get_inverse(
                self.to_glib_none().0,
                inverse.to_glib_none_mut().0,
            );
            (ret == crate::TRUE, inverse)
        }
    }

    /// Resets the current matrix to the identity matrix.
    pub fn load_identity(&self) {
        unsafe {
            ffi::cogl_matrix_stack_load_identity(self.to_glib_none().0);
        }
    }

    /// Multiplies the current matrix by the given matrix.
    /// ## `matrix`
    /// the matrix to multiply with the current model-view
    pub fn multiply(&self, matrix: &Matrix) {
        unsafe {
            ffi::cogl_matrix_stack_multiply(self.to_glib_none().0, matrix.to_glib_none().0);
        }
    }

    /// Replaces the current matrix with an orthographic projection matrix.
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
    pub fn orthographic(&self, x_1: f32, y_1: f32, x_2: f32, y_2: f32, near: f32, far: f32) {
        unsafe {
            ffi::cogl_matrix_stack_orthographic(
                self.to_glib_none().0,
                x_1,
                y_1,
                x_2,
                y_2,
                near,
                far,
            );
        }
    }

    /// Replaces the current matrix with a perspective matrix based on the
    /// provided values.
    ///
    /// `<note>`You should be careful not to have too great a `z_far` / `z_near`
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
    pub fn perspective(&self, fov_y: f32, aspect: f32, z_near: f32, z_far: f32) {
        unsafe {
            ffi::cogl_matrix_stack_perspective(self.to_glib_none().0, fov_y, aspect, z_near, z_far);
        }
    }

    /// Restores the previous transform that was last saved by calling
    /// `MatrixStack::push`.
    ///
    /// This is usually called while traversing a scenegraph whenever you
    /// return up one level in the graph towards the root node.
    pub fn pop(&self) {
        unsafe {
            ffi::cogl_matrix_stack_pop(self.to_glib_none().0);
        }
    }

    /// Saves the current transform and starts a new transform that derives
    /// from the current transform.
    ///
    /// This is usually called while traversing a scenegraph whenever you
    /// traverse one level deeper. `MatrixStack::pop` can then be
    /// called when going back up one layer to restore the previous
    /// transform of an ancestor.
    pub fn push(&self) {
        unsafe {
            ffi::cogl_matrix_stack_push(self.to_glib_none().0);
        }
    }

    /// Multiplies the current matrix by one that rotates the around the
    /// axis-vector specified by `x`, `y` and `z`. The rotation follows the
    /// right-hand thumb rule so for example rotating by 10 degrees about
    /// the axis-vector (0, 0, 1) causes a small counter-clockwise
    /// rotation.
    /// ## `angle`
    /// Angle in degrees to rotate.
    /// ## `x`
    /// X-component of vertex to rotate around.
    /// ## `y`
    /// Y-component of vertex to rotate around.
    /// ## `z`
    /// Z-component of vertex to rotate around.
    pub fn rotate(&self, angle: f32, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_matrix_stack_rotate(self.to_glib_none().0, angle, x, y, z);
        }
    }

    /// Multiplies the current matrix by one that rotates according to the
    /// rotation described by `euler`.
    ///
    /// ## `euler`
    /// A `Euler`
    pub fn rotate_euler(&self, euler: &Euler) {
        unsafe {
            ffi::cogl_matrix_stack_rotate_euler(self.to_glib_none().0, euler.to_glib_none().0);
        }
    }

    /// Multiplies the current matrix by one that rotates according to the
    /// rotation described by `quaternion`.
    /// ## `quaternion`
    /// A `Quaternion`
    pub fn rotate_quaternion(&self, quaternion: &Quaternion) {
        unsafe {
            ffi::cogl_matrix_stack_rotate_quaternion(
                self.to_glib_none().0,
                quaternion.to_glib_none().0,
            );
        }
    }

    /// Multiplies the current matrix by one that scales the x, y and z
    /// axes by the given values.
    /// ## `x`
    /// Amount to scale along the x-axis
    /// ## `y`
    /// Amount to scale along the y-axis
    /// ## `z`
    /// Amount to scale along the z-axis
    pub fn scale(&self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_matrix_stack_scale(self.to_glib_none().0, x, y, z);
        }
    }

    /// Replaces the current `self` matrix value with the value of `matrix`.
    /// This effectively discards any other operations that were applied
    /// since the last time `MatrixStack::push` was called or since
    /// the stack was initialized.
    /// ## `matrix`
    /// A `Matrix` replace the current matrix value with
    pub fn set(&self, matrix: &Matrix) {
        unsafe {
            ffi::cogl_matrix_stack_set(self.to_glib_none().0, matrix.to_glib_none().0);
        }
    }

    /// Multiplies the current matrix by one that translates along all
    /// three axes according to the given values.
    /// ## `x`
    /// Distance to translate along the x-axis
    /// ## `y`
    /// Distance to translate along the y-axis
    /// ## `z`
    /// Distance to translate along the z-axis
    pub fn translate(&self, x: f32, y: f32, z: f32) {
        unsafe {
            ffi::cogl_matrix_stack_translate(self.to_glib_none().0, x, y, z);
        }
    }
}

impl fmt::Display for MatrixStack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "MatrixStack")
    }
}
