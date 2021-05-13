#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::{Euler, Quaternion};

use std::boxed::Box as Box_;
use std::mem;

// * SECTION:cogl-matrix
// * @short_description: Functions for initializing and manipulating 4x4 matrices
// *
// * Matrices are used in  to describe affine model-view transforms, texture
// * transforms, and projective transforms. This exposes a utility API that can
// * be used for direct manipulation of these matrices.

// /*
//  * These identify different kinds of 4x4 transformation matrices and we use
//  * this information to find fast-paths when available.
//  */
//  enum MatrixType {
//     COGL_MATRIX_TYPE_GENERAL,	/**< general 4x4 matrix */
//     COGL_MATRIX_TYPE_IDENTITY,	/**< identity matrix */
//     COGL_MATRIX_TYPE_3D_NO_ROT,	/**< orthogonal projection and others... */
//     COGL_MATRIX_TYPE_PERSPECTIVE,	/**< perspective projection matrix */
//     COGL_MATRIX_TYPE_2D,		/**< 2-D transformation */
//     COGL_MATRIX_TYPE_2D_NO_ROT,	/**< 2-D scale & translate only */
//     COGL_MATRIX_TYPE_3D,		/**< 3-D transformation */
//     COGL_MATRIX_N_TYPES
//  } ;

// * Matrix:
// *
// * A Matrix holds a 4x4 transform matrix. This is a single precision,
// * column-major matrix which means it is compatible with what OpenGL expects.
// *
// * A Matrix can represent transforms such as, rotations, scaling,
// * translation, sheering, and linear projections. You can combine these
// * transforms by multiplying multiple matrices in the order you want them
// * applied.
// *
// * The transformation of a vertex (x, y, z, w) by a Matrix is given by:
// *
// * |[
// *   x_new = xx * x + xy * y + xz * z + xw * w
// *   y_new = yx * x + yy * y + yz * z + yw * w
// *   z_new = zx * x + zy * y + zz * z + zw * w
// *   w_new = wx * x + wy * y + wz * z + ww * w
// * ]|
// *
// * Where w is normally 1
// *
// * <note>You must consider the members of the Matrix structure read only,
// * and all matrix modifications must be done via the matrix API. This
// * allows  to annotate the matrices internally. Violation of this will give
// * undefined results. If you need to initialize a matrix with a constant other
// * than the identity matrix you can use matrix_init_from_array().</note>
#[derive(Default, Debug, Clone, PartialOrd)] // Hash
pub struct Matrix {
    // column 0
    xx: f32,
    yx: f32,
    zx: f32,
    wx: f32,

    // column 1
    xy: f32,
    yy: f32,
    zy: f32,
    wy: f32,

    // column 2
    xz: f32,
    yz: f32,
    zz: f32,
    wz: f32,

    // column 3
    xw: f32,
    yw: f32,
    zw: f32,
    ww: f32,
    // < private >

    // Note: we may want to extend this later with private flags
    // and a cache of the inverse transform matrix.

    // float          COGL_PRIVATE (inv)[16];
    // unsigned long  COGL_PRIVATE (type);
    // unsigned long  COGL_PRIVATE (flags);
    // unsigned long  COGL_PRIVATE (_padding3);
}

#[inline]
fn idx(row: usize, col: usize) -> usize {
    col * 4 + row
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
        let mut m: [f32; 16] = [0.0; 16];

        let x = (2.0 * z_near) / (right - left);
        let y = (2.0 * z_near) / (top - bottom);
        let a = (right + left) / (right - left);
        let b = (top + bottom) / (top - bottom);
        let c = -(z_far + z_near) / (z_far - z_near);
        let d = -(2.0 * z_far * z_near) / (z_far - z_near); /* error? */

        m[idx(0, 0)] = x;
        m[idx(0, 1)] = 0.0;
        m[idx(0, 2)] = a;
        m[idx(0, 3)] = 0.0;
        m[idx(1, 0)] = 0.0;
        m[idx(1, 1)] = y;
        m[idx(1, 2)] = b;
        m[idx(1, 3)] = 0.0;
        m[idx(2, 0)] = 0.0;
        m[idx(2, 1)] = 0.0;
        m[idx(2, 2)] = c;
        m[idx(2, 3)] = d;
        m[idx(3, 0)] = 0.0;
        m[idx(3, 1)] = 0.0;
        m[idx(3, 2)] = -1.0;
        m[idx(3, 3)] = 0.0;

        // matrix_multiply_array_with_flags (matrix, m, MAT_FLAG_PERSPECTIVE);
        unimplemented!()
    }

    //TODO:
    // /// Casts `self` to a float array which can be directly passed to OpenGL.
    // ///
    // /// # Returns
    // ///
    // /// a pointer to the float array
    // pub fn get_array(&self) -> &[f32] {
    //     unimplemented!()
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
        // if (_matrix_update_inverse ((Matrix *)matrix))
        // {
        // matrix_init_from_array (inverse, matrix->inv);
        // return true;
        // }
        // else
        // {
        // matrix_init_identity (inverse);
        // return false;
        // }
        unimplemented!()
    }

    /// Initializes `self` with the contents of `array`
    /// ## `array`
    /// A linear array of 16 floats (column-major order)
    pub fn init_from_array(&mut self, array: &[f32]) {
        // memcpy (matrix, array, 16 * sizeof (float));
        // matrix->flags = (MAT_FLAG_GENERAL | MAT_DIRTY_ALL);
    }

    /// Initializes `self` from a `Euler` rotation.
    ///
    /// ## `euler`
    /// A `Euler`
    pub fn init_from_euler(&mut self, euler: &Euler) {
        // /* Convert angles to radians */
        // float heading_rad = euler->heading / 180.0f * G_PI;
        // float pitch_rad = euler->pitch / 180.0f * G_PI;
        // float roll_rad = euler->roll / 180.0f * G_PI;
        // /* Pre-calculate the sin and cos */
        // float sin_heading = sinf (heading_rad);
        // float cos_heading = cosf (heading_rad);
        // float sin_pitch = sinf (pitch_rad);
        // float cos_pitch = cosf (pitch_rad);
        // float sin_roll = sinf (roll_rad);
        // float cos_roll = cosf (roll_rad);

        // /* These calculations are based on the following website but they
        // * use a different order for the rotations so it has been modified
        // * slightly.
        // * http://www.euclideanspace.com/maths/geometry/
        // *        rotations/conversions/eulerToMatrix/index.htm
        // */
        // /* Heading rotation x=0, y=1, z=0 gives:
        // *
        // * [ ch   0   sh   0 ]
        // * [ 0    1   0    0 ]
        // * [ -sh  0   ch   0 ]
        // * [ 0    0   0    1 ]
        // *
        // * Pitch rotation x=1, y=0, z=0 gives:
        // * [ 1    0   0    0 ]
        // * [ 0    cp  -sp  0 ]
        // * [ 0    sp  cp   0 ]
        // * [ 0    0   0    1 ]
        // *
        // * Roll rotation x=0, y=0, z=1 gives:
        // * [ cr   -sr 0    0 ]
        // * [ sr   cr  0    0 ]
        // * [ 0    0   1    0 ]
        // * [ 0    0   0    1 ]
        // *
        // * Heading matrix * pitch matrix =
        // * [ ch   sh*sp    cp*sh   0  ]
        // * [ 0    cp       -sp     0  ]
        // * [ -sh  ch*sp    ch*cp   0  ]
        // * [ 0    0        0       1  ]
        // *
        // * That matrix * roll matrix =
        // * [ ch*cr + sh*sp*sr   sh*sp*cr - ch*sr       sh*cp       0 ]
        // * [     cp*sr                cp*cr             -sp        0 ]
        // * [ ch*sp*sr - sh*cr   sh*sr + ch*sp*cr       ch*cp       0 ]
        // * [       0                    0                0         1 ]
        // */
        // matrix->xx = cos_heading * cos_roll + sin_heading * sin_pitch * sin_roll;
        // matrix->yx = cos_pitch * sin_roll;
        // matrix->zx = cos_heading * sin_pitch * sin_roll - sin_heading * cos_roll;
        // matrix->wx = 0.0f;

        // matrix->xy = sin_heading * sin_pitch * cos_roll - cos_heading * sin_roll;
        // matrix->yy = cos_pitch * cos_roll;
        // matrix->zy = sin_heading * sin_roll + cos_heading * sin_pitch * cos_roll;
        // matrix->wy = 0.0f;

        // matrix->xz = sin_heading * cos_pitch;
        // matrix->yz = -sin_pitch;
        // matrix->zz = cos_heading * cos_pitch;
        // matrix->wz = 0;

        // matrix->xw = 0;
        // matrix->yw = 0;
        // matrix->zw = 0;
        // matrix->ww = 1;

        // matrix->flags = (MAT_FLAG_GENERAL | MAT_DIRTY_ALL);
        unimplemented!()
    }

    /// Initializes `self` from a `Quaternion` rotation.
    /// ## `quaternion`
    /// A `Quaternion`
    pub fn init_from_quaternion(&mut self, quaternion: &Quaternion) {
        // float qnorm = _COGL_QUATERNION_NORM (quaternion);
        // float s = (qnorm > 0.0f) ? (2.0f / qnorm) : 0.0f;
        // float xs = quaternion->x * s;
        // float ys = quaternion->y * s;
        // float zs = quaternion->z * s;
        // float wx = quaternion->w * xs;
        // float wy = quaternion->w * ys;
        // float wz = quaternion->w * zs;
        // float xx = quaternion->x * xs;
        // float xy = quaternion->x * ys;
        // float xz = quaternion->x * zs;
        // float yy = quaternion->y * ys;
        // float yz = quaternion->y * zs;
        // float zz = quaternion->z * zs;

        // matrix->xx = 1.0f - (yy + zz);
        // matrix->yx = xy + wz;
        // matrix->zx = xz - wy;
        // matrix->xy = xy - wz;
        // matrix->yy = 1.0f - (xx + zz);
        // matrix->zy = yz + wx;
        // matrix->xz = xz + wy;
        // matrix->yz = yz - wx;
        // matrix->zz = 1.0f - (xx + yy);
        // matrix->xw = matrix->yw = matrix->zw = 0.0f;
        // matrix->wx = matrix->wy = matrix->wz = 0.0f;
        // matrix->ww = 1.0f;

        // matrix->flags = (MAT_FLAG_GENERAL | MAT_DIRTY_ALL);
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
        // memcpy (matrix, identity, 16 * sizeof (float));

        // matrix->type = COGL_MATRIX_TYPE_IDENTITY;
        // matrix->flags = MAT_DIRTY_INVERSE;
        unimplemented!()
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
        // memcpy (matrix, identity, 16 * sizeof (float));

        // matrix->xw = tx;
        // matrix->yw = ty;
        // matrix->zw = tz;

        // matrix->type = COGL_MATRIX_TYPE_3D;
        // matrix->flags = MAT_FLAG_TRANSLATION | MAT_DIRTY_INVERSE;
    }

    /// Determines if the given matrix is an identity matrix.
    ///
    /// # Returns
    ///
    /// `true` if `self` is an identity matrix else `false`
    pub fn is_identity(&self) -> bool {
        // if (!(matrix->flags & MAT_DIRTY_TYPE) &&
        //     matrix->type == COGL_MATRIX_TYPE_IDENTITY)
        // return true;
        // else
        // return memcmp (matrix, identity, sizeof (float) * 16) == 0;
        unimplemented!()
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
        // Matrix tmp;
        // float forward[3];
        // float side[3];
        // float up[3];

        // /* Get a unit viewing direction vector */
        // vector3_init (forward,
        //                     object_x - eye_position_x,
        //                     object_y - eye_position_y,
        //                     object_z - eye_position_z);
        // vector3_normalize (forward);

        // vector3_init (up, world_up_x, world_up_y, world_up_z);

        // /* Take the sideways direction as being perpendicular to the viewing
        // * direction and the word up vector. */
        // vector3_cross_product (side, forward, up);
        // vector3_normalize (side);

        // /* Now we have unit sideways and forward-direction vectors calculate
        // * a new mutually perpendicular up vector. */
        // vector3_cross_product (up, side, forward);

        // tmp.xx = side[0];
        // tmp.yx = side[1];
        // tmp.zx = side[2];
        // tmp.wx = 0;

        // tmp.xy = up[0];
        // tmp.yy = up[1];
        // tmp.zy = up[2];
        // tmp.wy = 0;

        // tmp.xz = -forward[0];
        // tmp.yz = -forward[1];
        // tmp.zz = -forward[2];
        // tmp.wz = 0;

        // tmp.xw = 0;
        // tmp.yw = 0;
        // tmp.zw = 0;
        // tmp.ww = 1;

        // tmp.flags = (MAT_FLAG_GENERAL_3D | MAT_DIRTY_TYPE | MAT_DIRTY_INVERSE);

        // matrix_translate (&tmp, -eye_position_x, -eye_position_y, -eye_position_z);

        // matrix_multiply (matrix, matrix, &tmp);
        unimplemented!()
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
        // result->flags = (a->flags |
        //     b->flags |
        //     MAT_DIRTY_TYPE |
        //     MAT_DIRTY_INVERSE);

        // if (TEST_MAT_FLAGS(result, MAT_FLAGS_3D))
        // matrix_multiply3x4 ((float *)result, (float *)a, (float *)b);
        // else
        // matrix_multiply4x4 ((float *)result, (float *)a, (float *)b);
        unimplemented!()
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
        // float m[16];

        // M (0,0) = 2.0f / (x_2 - x_1);
        // M (0,1) = 0.0f;
        // M (0,2) = 0.0f;
        // M (0,3) = -(x_2 + x_1) / (x_2 - x_1);

        // M (1,0) = 0.0f;
        // M (1,1) = 2.0f / (y_1 - y_2);
        // M (1,2) = 0.0f;
        // M (1,3) = -(y_1 + y_2) / (y_1 - y_2);

        // M (2,0) = 0.0f;
        // M (2,1) = 0.0f;
        // M (2,2) = -2.0f / (far - near);
        // M (2,3) = -(far + near) / (far - near);

        // M (3,0) = 0.0f;
        // M (3,1) = 0.0f;
        // M (3,2) = 0.0f;
        // M (3,3) = 1.0f;

        // matrix_multiply_array_with_flags (matrix, m,
        //                                     (MAT_FLAG_GENERAL_SCALE |
        //                                     MAT_FLAG_TRANSLATION));
        unimplemented!()
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
        // float ymax = z_near * tan (fov_y * G_PI / 360.0);

        // matrix_frustum (matrix,
        //                     -ymax * aspect,  /* left */
        //                     ymax * aspect,   /* right */
        //                     -ymax,           /* bottom */
        //                     ymax,            /* top */
        //                     z_near,
        //                     z_far);
        // _COGL_MATRIX_DEBUG_PRINT (matrix);
        unimplemented!()
    }

    pub fn project_points(
        &self,
        n_components: i32,
        stride_in: usize,
        points_in: &[u8],
        stride_out: usize,
        points_out: &[u8],
        n_points: i32,
    ) {
        // if (n_components == 2)
        //     _matrix_project_points_f2 (matrix,
        //                                     stride_in, points_in,
        //                                     stride_out, points_out,
        //                                     n_points);
        // else if (n_components == 3)
        //     _matrix_project_points_f3 (matrix,
        //                                     stride_in, points_in,
        //                                     stride_out, points_out,
        //                                     n_points);
        // else
        //     {
        //     _COGL_RETURN_IF_FAIL (n_components == 4);

        //     _matrix_project_points_f4 (matrix,
        //                                     stride_in, points_in,
        //                                     stride_out, points_out,
        //                                     n_points);
        //     }
    }

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
        // float xx, yy, zz, xy, yz, zx, xs, ys, zs, one_c, s, c;
        // float m[16];
        // Bool optimized;

        // s = sinf (angle * DEG2RAD);
        // c = cosf (angle * DEG2RAD);

        // memcpy (m, identity, 16 * sizeof (float));
        // optimized = false;

        // #define M(row,col)  m[col*4+row]

        // if (x == 0.0f)
        //     {
        //     if (y == 0.0f)
        //         {
        //         if (z != 0.0f)
        //             {
        //             optimized = true;
        //             /* rotate only around z-axis */
        //             M (0,0) = c;
        //             M (1,1) = c;
        //             if (z < 0.0f)
        //                 {
        //                 M (0,1) = s;
        //                 M (1,0) = -s;
        //                 }
        //             else
        //                 {
        //                 M (0,1) = -s;
        //                 M (1,0) = s;
        //                 }
        //             }
        //         }
        //     else if (z == 0.0f)
        //         {
        //         optimized = true;
        //         /* rotate only around y-axis */
        //         M (0,0) = c;
        //         M (2,2) = c;
        //         if (y < 0.0f)
        //             {
        //             M (0,2) = -s;
        //             M (2,0) = s;
        //             }
        //         else
        //             {
        //             M (0,2) = s;
        //             M (2,0) = -s;
        //             }
        //         }
        //     }
        // else if (y == 0.0f)
        //     {
        //     if (z == 0.0f)
        //         {
        //         optimized = true;
        //         /* rotate only around x-axis */
        //         M (1,1) = c;
        //         M (2,2) = c;
        //         if (x < 0.0f)
        //             {
        //             M (1,2) = s;
        //             M (2,1) = -s;
        //             }
        //         else
        //             {
        //             M (1,2) = -s;
        //             M (2,1) = s;
        //             }
        //         }
        //     }

        // if (!optimized)
        //     {
        //     const float mag = sqrtf (x * x + y * y + z * z);

        //     if (mag <= 1.0e-4)
        //         {
        //         /* no rotation, leave mat as-is */
        //         return;
        //         }

        //     x /= mag;
        //     y /= mag;
        //     z /= mag;

        //     /*
        //     *     Arbitrary axis rotation matrix.
        //     *
        //     *  This is composed of 5 matrices, Rz, Ry, T, Ry', Rz', multiplied
        //     *  like so:  Rz * Ry * T * Ry' * Rz'.  T is the final rotation
        //     *  (which is about the X-axis), and the two composite transforms
        //     *  Ry' * Rz' and Rz * Ry are (respectively) the rotations necessary
        //     *  from the arbitrary axis to the X-axis then back.  They are
        //     *  all elementary rotations.
        //     *
        //     *  Rz' is a rotation about the Z-axis, to bring the axis vector
        //     *  into the x-z plane.  Then Ry' is applied, rotating about the
        //     *  Y-axis to bring the axis vector parallel with the X-axis.  The
        //     *  rotation about the X-axis is then performed.  Ry and Rz are
        //     *  simply the respective inverse transforms to bring the arbitrary
        //     *  axis back to it's original orientation.  The first transforms
        //     *  Rz' and Ry' are considered inverses, since the data from the
        //     *  arbitrary axis gives you info on how to get to it, not how
        //     *  to get away from it, and an inverse must be applied.
        //     *
        //     *  The basic calculation used is to recognize that the arbitrary
        //     *  axis vector (x, y, z), since it is of unit length, actually
        //     *  represents the sines and cosines of the angles to rotate the
        //     *  X-axis to the same orientation, with theta being the angle about
        //     *  Z and phi the angle about Y (in the order described above)
        //     *  as follows:
        //     *
        //     *  cos ( theta ) = x / sqrt ( 1 - z^2 )
        //     *  sin ( theta ) = y / sqrt ( 1 - z^2 )
        //     *
        //     *  cos ( phi ) = sqrt ( 1 - z^2 )
        //     *  sin ( phi ) = z
        //     *
        //     *  Note that cos ( phi ) can further be inserted to the above
        //     *  formulas:
        //     *
        //     *  cos ( theta ) = x / cos ( phi )
        //     *  sin ( theta ) = y / sin ( phi )
        //     *
        //     *  ...etc.  Because of those relations and the standard trigonometric
        //     *  relations, it is pssible to reduce the transforms down to what
        //     *  is used below.  It may be that any primary axis chosen will give the
        //     *  same results (modulo a sign convention) using thie method.
        //     *
        //     *  Particularly nice is to notice that all divisions that might
        //     *  have caused trouble when parallel to certain planes or
        //     *  axis go away with care paid to reducing the expressions.
        //     *  After checking, it does perform correctly under all cases, since
        //     *  in all the cases of division where the denominator would have
        //     *  been zero, the numerator would have been zero as well, giving
        //     *  the expected result.
        //     */
        //     xx = x * x;
        //     yy = y * y;
        //     zz = z * z;
        //     xy = x * y;
        //     yz = y * z;
        //     zx = z * x;
        //     xs = x * s;
        //     ys = y * s;
        //     zs = z * s;
        //     one_c = 1.0f - c;

        //     /* We already hold the identity-matrix so we can skip some statements */
        //     M (0,0) = (one_c * xx) + c;
        //     M (0,1) = (one_c * xy) - zs;
        //     M (0,2) = (one_c * zx) + ys;
        //     /*    M (0,3) = 0.0f; */
        //     M (1,0) = (one_c * xy) + zs;
        //     M (1,1) = (one_c * yy) + c;
        //     M (1,2) = (one_c * yz) - xs;
        //     /*    M (1,3) = 0.0f; */
        //     M (2,0) = (one_c * zx) - ys;
        //     M (2,1) = (one_c * yz) + xs;
        //     M (2,2) = (one_c * zz) + c;
        //     /*    M (2,3) = 0.0f; */
        //     /*
        //         M (3,0) = 0.0f;
        //         M (3,1) = 0.0f;
        //         M (3,2) = 0.0f;
        //         M (3,3) = 1.0f;
        //         */
        //     }
        // #undef M

        // matrix_multiply_array_with_flags (matrix, m, MAT_FLAG_ROTATION);
        unimplemented!()
    }

    /// Multiplies `self` with a rotation transformation described by the
    /// given `Euler`.
    ///
    /// ## `euler`
    /// A euler describing a rotation
    pub fn rotate_euler(&mut self, euler: &Euler) {
        // Matrix rotation_transform;

        // matrix_init_from_euler (&rotation_transform, euler);
        // matrix_multiply (matrix, matrix, &rotation_transform);
        unimplemented!()
    }

    /// Multiplies `self` with a rotation transformation described by the
    /// given `Quaternion`.
    ///
    /// ## `quaternion`
    /// A quaternion describing a rotation
    pub fn rotate_quaternion(&mut self, quaternion: &Quaternion) {
        // Matrix rotation_transform;

        // matrix_init_from_quaternion (&rotation_transform, quaternion);
        // matrix_multiply (matrix, matrix, &rotation_transform);
        unimplemented!()
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
        // float *m = (float *)matrix;
        // m[0] *= x;   m[4] *= y;   m[8]  *= z;
        // m[1] *= x;   m[5] *= y;   m[9]  *= z;
        // m[2] *= x;   m[6] *= y;   m[10] *= z;
        // m[3] *= x;   m[7] *= y;   m[11] *= z;

        // if (fabsf (x - y) < 1e-8 && fabsf (x - z) < 1e-8)
        //   matrix->flags |= MAT_FLAG_UNIFORM_SCALE;
        // else
        //   matrix->flags |= MAT_FLAG_GENERAL_SCALE;

        // matrix->flags |= (MAT_DIRTY_TYPE | MAT_DIRTY_INVERSE);
        unimplemented!()
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
        // float _x = *x, _y = *y, _z = *z, _w = *w;

        // *x = matrix->xx * _x + matrix->xy * _y + matrix->xz * _z + matrix->xw * _w;
        // *y = matrix->yx * _x + matrix->yy * _y + matrix->yz * _z + matrix->yw * _w;
        // *z = matrix->zx * _x + matrix->zy * _y + matrix->zz * _z + matrix->zw * _w;
        // *w = matrix->wx * _x + matrix->wy * _y + matrix->wz * _z + matrix->ww * _w;
        unimplemented!()
    }

    pub fn transform_points(
        &self,
        n_components: i32,
        stride_in: usize,
        points_in: &[u8],
        stride_out: usize,
        points_out: &[u8],
        n_points: i32,
    ) {
        // /* The results of transforming always have three components... */
        // _COGL_RETURN_IF_FAIL (stride_out >= sizeof (Point3f));

        // if (n_components == 2)
        //     _matrix_transform_points_f2 (matrix,
        //                                     stride_in, points_in,
        //                                     stride_out, points_out,
        //                                     n_points);
        // else
        // {
        // _COGL_RETURN_IF_FAIL (n_components == 3);

        // _matrix_transform_points_f3 (matrix,
        //                                     stride_in, points_in,
        //                                     stride_out, points_out,
        //                                     n_points);
        // }
        unimplemented!()
    }

    /// Multiplies `self` with a transform matrix that translates along
    /// the X, Y and Z axis.
    /// ## `x`
    /// The X translation you want to apply
    /// ## `y`
    /// The Y translation you want to apply
    /// ## `z`
    /// The Z translation you want to apply
    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        // float *m = (float *)matrix;
        // m[12] = m[0] * x + m[4] * y + m[8]  * z + m[12];
        // m[13] = m[1] * x + m[5] * y + m[9]  * z + m[13];
        // m[14] = m[2] * x + m[6] * y + m[10] * z + m[14];
        // m[15] = m[3] * x + m[7] * y + m[11] * z + m[15];

        // matrix->flags |= (MAT_FLAG_TRANSLATION |
        //                   MAT_DIRTY_TYPE |
        //                   MAT_DIRTY_INVERSE);
        unimplemented!()
    }

    /// Replaces `self` with its transpose. Ie, every element (i,j) in the
    /// new matrix is taken from element (j,i) in the old matrix.
    pub fn transpose(&mut self) {
        // float new_values[16];

        // /* We don't need to do anything if the matrix is the identity matrix */
        // if (!(matrix->flags & MAT_DIRTY_TYPE) &&
        //     matrix->type == COGL_MATRIX_TYPE_IDENTITY)
        //     return;

        // _matrix_util_transposef (new_values, matrix_get_array (matrix));

        // matrix_init_from_array (matrix, new_values);
        unimplemented!()
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
        // float left_2d_plane = left / z_near * z_2d;
        // float right_2d_plane = right / z_near * z_2d;
        // float bottom_2d_plane = bottom / z_near * z_2d;
        // float top_2d_plane = top / z_near * z_2d;

        // float width_2d_start = right_2d_plane - left_2d_plane;
        // float height_2d_start = top_2d_plane - bottom_2d_plane;

        // /* Factors to scale from framebuffer geometry to frustum
        // * cross-section geometry. */
        // float width_scale = width_2d_start / width_2d;
        // float height_scale = height_2d_start / height_2d;

        // matrix_translate (matrix,
        //                         left_2d_plane, top_2d_plane, -z_2d);

        // matrix_scale (matrix, width_scale, -height_scale, width_scale);
        unimplemented!()
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
        // float top = z_near * tan (fov_y * G_PI / 360.0);
        // matrix_view_2d_in_frustum (matrix,
        //                                 -top * aspect,
        //                                 top * aspect,
        //                                 -top,
        //                                 top,
        //                                 z_near,
        //                                 z_2d,
        //                                 width_2d,
        //                                 height_2d);
        unimplemented!()
    }

    fn equal(v1: &Self, v2: &Self) -> bool {
        // const Matrix *a = v1;
        // const Matrix *b = v2;

        // _COGL_RETURN_VAL_IF_FAIL (v1 != NULL, false);
        // _COGL_RETURN_VAL_IF_FAIL (v2 != NULL, false);

        // /* We want to avoid having a fuzzy _equal() function (e.g. that uses
        // * an arbitrary epsilon value) since this function noteably conforms
        // * to the prototype suitable for use with g_hash_table_new() and a
        // * fuzzy hash function isn't really appropriate for comparing hash
        // * table keys since it's possible that you could end up fetching
        // * different values if you end up with multiple similar keys in use
        // * at the same time. If you consider that fuzzyness allows cases
        // * such as A == B == C but A != C then you could also end up loosing
        // * values in a hash table.
        // *
        // * We do at least use the == operator to compare values though so
        // * that -0 is considered equal to 0.
        // */
        // /* XXX: We don't compare the flags, inverse matrix or padding */
        // if (a->xx == b->xx &&
        //     a->xy == b->xy &&
        //     a->xz == b->xz &&
        //     a->xw == b->xw &&
        //     a->yx == b->yx &&
        //     a->yy == b->yy &&
        //     a->yz == b->yz &&
        //     a->yw == b->yw &&
        //     a->zx == b->zx &&
        //     a->zy == b->zy &&
        //     a->zz == b->zz &&
        //     a->zw == b->zw &&
        //     a->wx == b->wx &&
        //     a->wy == b->wy &&
        //     a->wz == b->wz &&
        //     a->ww == b->ww)
        //     return true;
        // else
        //     return false;
        unimplemented!()
    }
}

impl PartialEq for Matrix {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Matrix::equal(self, other)
    }
}

impl Eq for Matrix {}
