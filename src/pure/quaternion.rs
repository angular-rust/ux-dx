#![allow(
    clippy::too_many_arguments,
    clippy::let_and_return,
    clippy::from_over_into
)]

use super::{Euler, Matrix};
use std::boxed::Box as Box_;
use std::mem;
// * SECTION:cogl-quaternion
// * @short_description: Functions for initializing and manipulating
// * quaternions.
// *
// * Quaternions have become a standard form for representing 3D
// * rotations and have some nice properties when compared with other
// * representation such as (roll,pitch,yaw) Euler angles. They can be
// * used to interpolate between different rotations and they don't
// * suffer from a problem called
// * <ulink url="http://en.wikipedia.org/wiki/Gimbal_lock">"Gimbal lock"</ulink>
// * where two of the axis of rotation may become aligned and you loose a
// * degree of freedom.
// * Quaternion:
// * @w: based on the angle of rotation it is cos(𝜃/2)
// * @x: based on the angle of rotation and x component of the axis of
// *     rotation it is sin(𝜃/2)*axis.x
// * @y: based on the angle of rotation and y component of the axis of
// *     rotation it is sin(𝜃/2)*axis.y
// * @z: based on the angle of rotation and z component of the axis of
// *     rotation it is sin(𝜃/2)*axis.z
// *
// * A quaternion is comprised of a scalar component and a 3D vector
// * component. The scalar component is normally referred to as w and the
// * vector might either be referred to as v or a (for axis) or expanded
// * with the individual components: (x, y, z) A full quaternion would
// * then be written as <literal>[w (x, y, z)]</literal>.
// *
// * Quaternions can be considered to represent an axis and angle
// * pair although sadly these numbers are buried somewhat under some
// * maths...
// *
// * For the curious you can see here that a given axis (a) and angle (𝜃)
// * pair are represented in a quaternion as follows:
// * |[
// * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
// * ]|
// *
// * Unit Quaternions:
// * When using Quaternions to represent spatial orientations for 3D
// * graphics it's always assumed you have a unit quaternion. The
// * magnitude of a quaternion is defined as:
// * |[
// * sqrt (w² + x² + y² + z²)
// * ]|
// * and a unit quaternion satisfies this equation:
// * |[
// * w² + x² + y² + z² = 1
// * ]|
// *
// * Thankfully most of the time we don't actually have to worry about
// * the maths that goes on behind the scenes but if you are curious to
// * learn more here are some external references:
// *
// * <itemizedlist>
// * <listitem>
// * <ulink url="http://mathworld.wolfram.com/Quaternion.html"/>
// * </listitem>
// * <listitem>
// * <ulink url="http://www.gamedev.net/reference/articles/article1095.asp"/>
// * </listitem>
// * <listitem>
// * <ulink url="http://www.cprogramming.com/tutorial/3d/quaternions.html"/>
// * </listitem>
// * <listitem>
// * <ulink url="http://www.isner.com/tutorials/quatSpells/quaternion_spells_12.htm"/>
// * </listitem>
// * <listitem>
// * 3D Maths Primer for Graphics and Game Development ISBN-10: 1556229119
// * </listitem>
// * <listitem>
// * <ulink url="http://www.cs.caltech.edu/courses/cs171/quatut.pdf"/>
// * </listitem>
// * <listitem>
// * <ulink url="http://www.j3d.org/matrix_faq/matrfaq_latest.html#Q56"/>
// * </listitem>
// * </itemizedlist>
// *
#[derive(Debug, PartialOrd, Ord)] // Hash
pub struct Quaternion {
    //< public >
// float w;

// float x;
// float y;
// float z;

// /*< private >*/
// float padding0;
// float padding1;
// float padding2;
// float padding3;
}

impl Quaternion {
    ///
    /// ## `b`
    /// A `Quaternion`
    pub fn dot_product(&self, b: &Quaternion) -> f32 {
        // return a->w * b->w + a->x * b->x + a->y * b->y + a->z * b->z;
        unimplemented!()
    }

    ///
    pub fn get_rotation_angle(&self) -> f32 {
        // /* NB: We are using quaternions to represent an axis (a), angle (𝜃) pair
        // * in this form:
        // * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
        // */
        // /* FIXME: clamp [-1, 1] */
        // return 2.0f * acosf (quaternion->w) * _COGL_QUATERNION_RADIANS_TO_DEGREES;
        unimplemented!()
    }

    ///
    /// ## `vector3`
    /// an allocated 3-float array
    pub fn get_rotation_axis(&self) -> f32 {
        // float sin_half_angle_sqr;
        // float one_over_sin_angle_over_2;

        // /* NB: We are using quaternions to represent an axis (a), angle (𝜃) pair
        // * in this form:
        // * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
        // */
        // /* NB: sin²(𝜃) + cos²(𝜃) = 1 */
        // sin_half_angle_sqr = 1.0f - quaternion->w * quaternion->w;

        // if (sin_half_angle_sqr <= 0.0f)
        //     {
        //     /* Either an identity quaternion or numerical imprecision.
        //     * Either way we return an arbitrary vector. */
        //     vector3[0] = 1;
        //     vector3[1] = 0;
        //     vector3[2] = 0;
        //     return;
        //     }

        // /* Calculate 1 / sin(𝜃/2) */
        // one_over_sin_angle_over_2 = 1.0f / sqrtf (sin_half_angle_sqr);

        // vector3[0] = quaternion->x * one_over_sin_angle_over_2;
        // vector3[1] = quaternion->y * one_over_sin_angle_over_2;
        // vector3[2] = quaternion->z * one_over_sin_angle_over_2;
        unimplemented!()
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
        // float axis[3] = { x, y, z};
        // quaternion_init_from_angle_vector (quaternion, angle, axis);
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
        // /* NB: We are using quaternions to represent an axis (a), angle (𝜃) pair
        // * in this form:
        // * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
        // */
        // float axis[3];
        // float half_angle;
        // float sin_half_angle;

        // /* XXX: Should we make vector3_normalize have separate in and
        // * out args? */
        // axis[0] = axis3f_in[0];
        // axis[1] = axis3f_in[1];
        // axis[2] = axis3f_in[2];
        // vector3_normalize (axis);

        // half_angle = angle * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f;
        // sin_half_angle = sinf (half_angle);

        // quaternion->w = cosf (half_angle);

        // quaternion->x = axis[0] * sin_half_angle;
        // quaternion->y = axis[1] * sin_half_angle;
        // quaternion->z = axis[2] * sin_half_angle;

        // quaternion_normalize (quaternion);
        unimplemented!()
    }

    /// Initializes a [w (x, y,z)] quaternion directly from an array of 4
    /// floats: [w,x,y,z].
    ///
    /// ## `array`
    /// An array of 4 floats w,(x,y,z)
    pub fn init_from_array(&mut self, array: &[f32]) {
        // quaternion->w = array[0];
        // quaternion->x = array[1];
        // quaternion->y = array[2];
        // quaternion->z = array[3];
        unimplemented!()
    }

    ///
    /// ## `euler`
    /// A `Euler` with which to initialize the quaternion
    pub fn init_from_euler(&mut self, euler: &Euler) {
        // /* NB: We are using quaternions to represent an axis (a), angle (𝜃) pair
        // * in this form:
        // * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
        // */
        // float sin_heading =
        // sinf (euler->heading * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f);
        // float sin_pitch =
        // sinf (euler->pitch * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f);
        // float sin_roll =
        // sinf (euler->roll * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f);
        // float cos_heading =
        // cosf (euler->heading * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f);
        // float cos_pitch =
        // cosf (euler->pitch * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f);
        // float cos_roll =
        // cosf (euler->roll * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f);

        // quaternion->w =
        // cos_heading * cos_pitch * cos_roll +
        // sin_heading * sin_pitch * sin_roll;

        // quaternion->x =
        // cos_heading * sin_pitch * cos_roll +
        // sin_heading * cos_pitch * sin_roll;
        // quaternion->y =
        // sin_heading * cos_pitch * cos_roll -
        // cos_heading * sin_pitch * sin_roll;
        // quaternion->z =
        // cos_heading * cos_pitch * sin_roll -
        // sin_heading * sin_pitch * cos_roll;
        unimplemented!()
    }

    /// Initializes a quaternion from a rotation matrix.
    /// ## `matrix`
    /// A rotation matrix with which to initialize the quaternion
    pub fn init_from_matrix(&mut self, matrix: &Matrix) {
        // /* Algorithm devised by Ken Shoemake, Ref:
        // * http://campar.in.tum.de/twiki/pub/Chair/DwarfTutorial/quatut.pdf
        // */
        // /* 3D maths literature refers to the diagonal of a matrix as the
        // * "trace" of a matrix... */
        // float trace = matrix->xx + matrix->yy + matrix->zz;
        // float root;

        // if (trace > 0.0f)
        //     {
        //     root = sqrtf (trace + 1);
        //     quaternion->w = root * 0.5f;
        //     root = 0.5f / root;
        //     quaternion->x = (matrix->zy - matrix->yz) * root;
        //     quaternion->y = (matrix->xz - matrix->zx) * root;
        //     quaternion->z = (matrix->yx - matrix->xy) * root;
        //     }
        // else
        //     {
        // #define X 0
        // #define Y 1
        // #define Z 2
        // #define W 3
        //     int h = X;
        //     if (matrix->yy > matrix->xx)
        //         h = Y;
        //     if (matrix->zz > COGL_MATRIX_READ (matrix, h, h))
        //         h = Z;
        //     switch (h)
        //         {
        // #define CASE_MACRO(i, j, k, I, J, K) \
        //         case I: \
        //         root = sqrtf ((COGL_MATRIX_READ (matrix, I, I) - \
        //                         (COGL_MATRIX_READ (matrix, J, J) + \
        //                         COGL_MATRIX_READ (matrix, K, K))) + \
        //                         COGL_MATRIX_READ (matrix, W, W)); \
        //         quaternion->i = root * 0.5f;\
        //         root = 0.5f / root;\
        //         quaternion->j = (COGL_MATRIX_READ (matrix, I, J) + \
        //                         COGL_MATRIX_READ (matrix, J, I)) * root; \
        //         quaternion->k = (COGL_MATRIX_READ (matrix, K, I) + \
        //                         COGL_MATRIX_READ (matrix, I, K)) * root; \
        //         quaternion->w = (COGL_MATRIX_READ (matrix, K, J) - \
        //                         COGL_MATRIX_READ (matrix, J, K)) * root;\
        //         break
        //         CASE_MACRO (x, y, z, X, Y, Z);
        //         CASE_MACRO (y, z, x, Y, Z, X);
        //         CASE_MACRO (z, x, y, Z, X, Y);
        // #undef CASE_MACRO
        // #undef X
        // #undef Y
        // #undef Z
        //         }
        //     }

        // if (matrix->ww != 1.0f)
        //     {
        //     float s = 1.0 / sqrtf (matrix->ww);
        //     quaternion->w *= s;
        //     quaternion->x *= s;
        //     quaternion->y *= s;
        //     quaternion->z *= s;
        //     }
        unimplemented!()
    }

    ///
    /// ## `src`
    /// A `Quaternion` with which to initialize `self`
    pub fn init_from_quaternion(&mut self, src: &mut Quaternion) {
        // memcpy (quaternion, src, sizeof (float) * 4);
        unimplemented!()
    }

    /// XXX: check which direction this rotates
    ///
    /// ## `angle`
    /// The angle to rotate around the x axis
    pub fn init_from_x_rotation(&mut self, angle: f32) {
        // /* NB: We are using quaternions to represent an axis (a), angle (𝜃) pair
        // * in this form:
        // * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
        // */
        // float half_angle = angle * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f;

        // quaternion->w = cosf (half_angle);

        // quaternion->x = sinf (half_angle);
        // quaternion->y = 0.0f;
        // quaternion->z = 0.0f;
        unimplemented!()
    }

    ///
    /// ## `angle`
    /// The angle to rotate around the y axis
    pub fn init_from_y_rotation(&mut self, angle: f32) {
        // /* NB: We are using quaternions to represent an axis (a), angle (𝜃) pair
        // * in this form:
        // * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
        // */
        // float half_angle = angle * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f;

        // quaternion->w = cosf (half_angle);

        // quaternion->x = 0.0f;
        // quaternion->y = sinf (half_angle);
        // quaternion->z = 0.0f;
        unimplemented!()
    }

    ///
    /// ## `angle`
    /// The angle to rotate around the z axis
    pub fn init_from_z_rotation(&mut self, angle: f32) {
        // /* NB: We are using quaternions to represent an axis (a), angle (𝜃) pair
        // * in this form:
        // * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
        // */
        // float half_angle = angle * _COGL_QUATERNION_DEGREES_TO_RADIANS * 0.5f;

        // quaternion->w = cosf (half_angle);

        // quaternion->x = 0.0f;
        // quaternion->y = 0.0f;
        // quaternion->z = sinf (half_angle);
        unimplemented!()
    }

    /// Initializes the quaternion with the canonical quaternion identity
    /// [1 (0, 0, 0)] which represents no rotation. Multiplying a
    /// quaternion with this identity leaves the quaternion unchanged.
    ///
    /// You might also want to consider using
    /// `get_static_identity_quaternion`.
    ///
    pub fn init_identity(&mut self) {
        // quaternion->w = 1.0;

        // quaternion->x = 0.0;
        // quaternion->y = 0.0;
        // quaternion->z = 0.0;
        unimplemented!()
    }

    ///
    pub fn invert(&mut self) {
        // quaternion->x = -quaternion->x;
        // quaternion->y = -quaternion->y;
        // quaternion->z = -quaternion->z;
        unimplemented!()
    }

    /// This combines the rotations of two quaternions into `self`. The
    /// operation is not commutative so the order is important because AxB
    /// != BxA.  follows the standard convention for quaternions here
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
        // float w = a->w;
        // float x = a->x;
        // float y = a->y;
        // float z = a->z;

        // _COGL_RETURN_IF_FAIL (b != result);

        // result->w = w * b->w - x * b->x - y * b->y - z * b->z;

        // result->x = w * b->x + x * b->w + y * b->z - z * b->y;
        // result->y = w * b->y + y * b->w + z * b->x - x * b->z;
        // result->z = w * b->z + z * b->w + x * b->y - y * b->x;
        unimplemented!()
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
        // float cos_difference;
        // float qb_w;
        // float qb_x;
        // float qb_y;
        // float qb_z;
        // float fa;
        // float fb;

        // _COGL_RETURN_IF_FAIL (t >=0 && t <= 1.0f);

        // if (t == 0)
        //     {
        //     *result = *a;
        //     return;
        //     }
        // else if (t == 1)
        //     {
        //     *result = *b;
        //     return;
        //     }

        // /* compute the cosine of the angle between the two given quaternions */
        // cos_difference = quaternion_dot_product (a, b);

        // /* If negative, use -b. Two quaternions q and -q represent the same angle but
        // * may produce a different slerp. We choose b or -b to rotate using the acute
        // * angle.
        // */
        // if (cos_difference < 0.0f)
        //     {
        //     qb_w = -b->w;
        //     qb_x = -b->x;
        //     qb_y = -b->y;
        //     qb_z = -b->z;
        //     cos_difference = -cos_difference;
        //     }
        // else
        //     {
        //     qb_w = b->w;
        //     qb_x = b->x;
        //     qb_y = b->y;
        //     qb_z = b->z;
        //     }

        // /* If we have two unit quaternions the dot should be <= 1.0 */
        // g_assert (cos_difference < 1.1f);

        // fa = 1.0f - t;
        // fb = t;

        // result->x = fa * a->x + fb * qb_x;
        // result->y = fa * a->y + fb * qb_y;
        // result->z = fa * a->z + fb * qb_z;
        // result->w = fa * a->w + fb * qb_w;

        // quaternion_normalize (result);
        unimplemented!()
    }

    ///
    pub fn normalize(&mut self) {
        // float slen = _COGL_QUATERNION_NORM (quaternion);
        // float factor = 1.0f / sqrtf (slen);

        // quaternion->x *= factor;
        // quaternion->y *= factor;
        // quaternion->z *= factor;

        // quaternion->w *= factor;

        unimplemented!()
    }

    ///
    /// ## `exponent`
    /// the exponent
    pub fn pow(&mut self, exponent: f32) {
        // float half_angle;
        // float new_half_angle;
        // float factor;

        // /* Try and identify and nop identity quaternions to avoid
        // * dividing by zero */
        // if (fabs (quaternion->w) > 0.9999f)
        //     return;

        // /* NB: We are using quaternions to represent an axis (a), angle (𝜃) pair
        // * in this form:
        // * [w=cos(𝜃/2) ( x=sin(𝜃/2)*a.x, y=sin(𝜃/2)*a.y, z=sin(𝜃/2)*a.x )]
        // */
        // /* FIXME: clamp [-1, 1] */
        // /* Extract 𝜃/2 from w */
        // half_angle = acosf (quaternion->w);

        // /* Compute the new 𝜃/2 */
        // new_half_angle = half_angle * exponent;

        // /* Compute the new w value */
        // quaternion->w = cosf (new_half_angle);

        // /* And new xyz values */
        // factor = sinf (new_half_angle) / sinf (half_angle);
        // quaternion->x *= factor;
        // quaternion->y *= factor;
        // quaternion->z *= factor;
        unimplemented!()
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
        // float cos_difference;
        // float qb_w;
        // float qb_x;
        // float qb_y;
        // float qb_z;
        // float fa;
        // float fb;

        // _COGL_RETURN_IF_FAIL (t >=0 && t <= 1.0f);

        // if (t == 0)
        //     {
        //     *result = *a;
        //     return;
        //     }
        // else if (t == 1)
        //     {
        //     *result = *b;
        //     return;
        //     }

        // /* compute the cosine of the angle between the two given quaternions */
        // cos_difference = quaternion_dot_product (a, b);

        // /* If negative, use -b. Two quaternions q and -q represent the same angle but
        // * may produce a different slerp. We choose b or -b to rotate using the acute
        // * angle.
        // */
        // if (cos_difference < 0.0f)
        //     {
        //     qb_w = -b->w;
        //     qb_x = -b->x;
        //     qb_y = -b->y;
        //     qb_z = -b->z;
        //     cos_difference = -cos_difference;
        //     }
        // else
        //     {
        //     qb_w = b->w;
        //     qb_x = b->x;
        //     qb_y = b->y;
        //     qb_z = b->z;
        //     }

        // /* If we have two unit quaternions the dot should be <= 1.0 */
        // g_assert (cos_difference < 1.1f);

        // /* Determine the interpolation factors for each quaternion, simply using
        // * linear interpolation for quaternions that are nearly exactly the same.
        // * (this will avoid divisions by zero)
        // */
        // if (cos_difference > 0.9999f)
        //     {
        //     fa = 1.0f - t;
        //     fb = t;

        //     /* XXX: should we also normalize() at the end in this case? */
        //     }
        // else
        //     {
        //     /* Calculate the sin of the angle between the two quaternions using the
        //     * trig identity: sin²(𝜃) + cos²(𝜃) = 1
        //     */
        //     float sin_difference =  sqrtf (1.0f - cos_difference * cos_difference);

        //     float difference = atan2f (sin_difference, cos_difference);
        //     float one_over_sin_difference = 1.0f / sin_difference;
        //     fa = sinf ((1.0f - t) * difference) * one_over_sin_difference;
        //     fb = sinf (t * difference) * one_over_sin_difference;
        //     }

        // /* Finally interpolate the two quaternions */
        // result->x = fa * a->x + fb * qb_x;
        // result->y = fa * a->y + fb * qb_y;
        // result->z = fa * a->z + fb * qb_z;
        // result->w = fa * a->w + fb * qb_w;
        unimplemented!()
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
        // Quaternion slerp0;
        // Quaternion slerp1;

        // quaternion_slerp (&slerp0, a, b, t);
        // quaternion_slerp (&slerp1, prev, next, t);
        // quaternion_slerp (result, &slerp0, &slerp1, 2.0f * t * (1.0f - t));
        unimplemented!()
    }

    fn equal(v1: &Self, v2: &Self) -> bool {
        // const Quaternion *a = v1;
        // const Quaternion *b = v2;

        // _COGL_RETURN_VAL_IF_FAIL (v1 != NULL, false);
        // _COGL_RETURN_VAL_IF_FAIL (v2 != NULL, false);

        // if (v1 == v2)
        //     return true;

        // return (a->w == b->w &&
        //         a->x == b->x &&
        //         a->y == b->y &&
        //         a->z == b->z);
        unimplemented!()
    }
}

impl PartialEq for Quaternion {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Quaternion::equal(self, other)
    }
}

impl Eq for Quaternion {}
