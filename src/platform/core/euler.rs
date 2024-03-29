use super::Matrix;
use std::f32::consts::FRAC_PI_2;
// use super::Quaternion;

// @short_description: Functions for initializing and manipulating
// euler angles.
//
// Euler angles are a simple representation of a 3 dimensional
// rotation; comprised of 3 ordered heading, pitch and roll rotations.
// An important thing to understand is that the axis of rotation
// belong to the object being rotated and so they also rotate as each
// of the heading, pitch and roll rotations are applied.
//
// One way to consider euler angles is to imagine controlling an
// aeroplane, where you first choose a heading (Such as flying south
// east), then you set the pitch (such as 30 degrees to take off) and
// then you might set a roll, by dipping the left, wing as you prepare
// to turn.
//
// They have some advantages and limitations that it helps to be
// aware of:
//
// Advantages:
// <itemizedlist>
// - 
// Easy to understand and use, compared to quaternions and matrices,
// so may be a good choice for a user interface.
// </listitem>
// - 
// Efficient storage, needing only 3 components any rotation can be
// represented.
// Actually the #Euler type isn't optimized for size because
// we may cache the equivalent #Quaternion along with a euler
// rotation, but it would be trivial for an application to track the
// components of euler rotations in a packed float array if optimizing
// for size was important. The values could be passed to  only when
// manipulation is necessary.
// </listitem>
// </itemizedlist>
//
// Disadvantages:
// <itemizedlist>
// - 
// Aliasing: it's possible to represent some rotations with multiple
// different heading, pitch and roll rotations.
// </listitem>
// - 
// They can suffer from a problem called Gimbal Lock. A good
// explanation of this can be seen on wikipedia here:
// http://en.wikipedia.org/wiki/Gimbal_lock but basically two
// of the axis of rotation may become aligned and so you loose a
// degree of freedom. For example a pitch of +-90° would mean that
// heading and bank rotate around the same axis.
// </listitem>
// - 
// If you use euler angles to orient something in 3D space and try to
// transition between orientations by interpolating the component
// angles you probably wont get the transitions you expect as they may
// not follow the shortest path between the two orientations.
// </listitem>
// - 
// There's no standard to what order the component axis rotations are
// applied. The most common convention seems to be what we do in
// with heading (y-axis), pitch (x-axis) and then roll (z-axis), but
// other software might apply x-axis, y-axis then z-axis or any other
// order so you need to consider this if you are accepting euler
// rotations from some other software. Other software may also use
// slightly different aeronautical terms, such as "yaw" instead of
// "heading" or "bank" instead of "roll".
// </listitem>
// </itemizedlist>
//
// To minimize the aliasing issue we may refer to "Canonical Euler"
// angles where heading and roll are restricted to +- 180° and pitch is
// restricted to +- 90°. If pitch is +- 90° bank is set to 0°.
//
// Quaternions don't suffer from Gimbal Lock and they can be nicely
// interpolated between, their disadvantage is that they don't have an
// intuitive representation.
//
// A common practice is to accept angles in the intuitive Euler form
// and convert them to quaternions internally to avoid Gimbal Lock and
// handle interpolations. See quaternion_init_from_euler().

// Euler:
// @heading: Angle to rotate around an object's y axis
// @pitch: Angle to rotate around an object's x axis
// @roll: Angle to rotate around an object's z axis
//
// Represents an ordered rotation first of @heading degrees around an
// object's y axis, then @pitch degrees around an object's x axis and
// finally @roll degrees around an object's z axis.
//
// It's important to understand the that axis are associated
// with the object being rotated, so the axis also rotate in sequence
// with the rotations being applied.
//
// The members of a #Euler can be initialized, for example, with
// euler_init() and euler_init_from_quaternion ().
//
// You may also want to look at quaternion_init_from_euler() if
// you want to do interpolation between 3d rotations.
#[derive(Debug, PartialOrd)] // Hash
pub struct Euler {
    // < public >
    heading: f32,
    pitch: f32,
    roll: f32,
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
        self.heading = heading;
        self.pitch = pitch;
        self.roll = roll;
    }

    /// Extracts a euler rotation from the given `matrix` and
    /// initializses `self` with the component x, y and z rotation angles.
    ///
    /// ## `matrix`
    /// A `Matrix` containing a rotation, but no scaling,
    ///  mirroring or skewing.
    pub fn init_from_matrix(&mut self, matrix: &Matrix) {
        //
        // Extracting a canonical Euler angle from a matrix:
        // (where it is assumed the matrix contains no scaling, mirroring or
        //  skewing)
        //
        // A Euler angle is a combination of three rotations around mutually
        // perpendicular axis. For this algorithm they are:
        //
        // Heading: A rotation about the Y axis by an angle H:
        // | cosH  0  sinH|
        // |    0  1     0|
        // |-sinH  0  cosH|
        //
        // Pitch: A rotation around the X axis by an angle P:
        // |1     0      0|
        // |0  cosP  -sinP|
        // |0  sinP   cosP|
        //
        // Roll: A rotation about the Z axis by an angle R:
        // |cosR -sinR  0|
        // |sinR  cosR  0|
        // |   0     0  1|
        //
        // When multiplied as matrices this gives:
        //     | cosHcosR+sinHsinPsinR   sinRcosP  -sinHcosR+cosHsinPsinR|
        // M = |-cosHsinR+sinHsinPcosR   cosRcosP   sinRsinH+cosHsinPcosB|
        //     | sinHcosP               -sinP       cosHcosP             |
        //
        // Given that there are an infinite number of ways to represent
        // a given orientation, the "canonical" Euler angle is any such that:
        //  -180 < H < 180,
        //  -180 < R < 180 and
        //   -90 < P < 90
        //
        // M[3][2] = -sinP lets us immediately solve for P = asin(-M[3][2])
        //   (Note: asin has a range of +-90)
        // This gives cosP
        // This means we can use M[3][1] to calculate sinH:
        //   sinH = M[3][1]/cosP
        // And use M[3][3] to calculate cosH:
        //   cosH = M[3][3]/cosP
        // This lets us calculate H = atan2(sinH,cosH), but we optimise this:
        //   1st note: atan2(x, y) does: atan(x/y) and uses the sign of x and y to
        //   determine the quadrant of the final angle.
        //   2nd note: we know cosP is > 0 (ignoring cosP == 0)
        //   Therefore H = atan2((M[3][1]/cosP) / (M[3][3]/cosP)) can be simplified
        //   by skipping the division by cosP since it won't change the x/y ratio
        //   nor will it change their sign. This gives:
        //     H = atan2(M[3][1], M[3][3])
        // R is computed in the same way as H from M[1][2] and M[2][2] so:
        //     R = atan2(M[1][2], M[2][2])
        // Note: If cosP were == 0 then H and R could not be calculated as above
        // because all the necessary matrix values would == 0. In other words we are
        // pitched vertically and so H and R would now effectively rotate around the
        // same axis - known as "Gimbal lock". In this situation we will set all the
        // rotation on H and set R = 0.
        //   So with P = R = 0 we have cosP = 0, sinR = 0 and cosR = 1
        //   We can substitute those into the above equation for M giving:
        //   |    cosH      0     -sinH|
        //   |sinHsinP      0  cosHsinP|
        //   |       0  -sinP         0|
        //   And calculate H as atan2 (-M[3][2], M[1][1])

        // NB: Matrix provides struct members named according to the
        // [row][column] indexed. So matrix.zx is row 3 column 1.
        let sin_p = -matrix.zy;
        // Determine the Pitch, avoiding domain errors with asin () which
        // might occur due to previous imprecision in manipulating the matrix.

        if sin_p <= -1.0 {
            self.pitch = -FRAC_PI_2;
        } else if sin_p >= 1.0 {
            self.pitch = FRAC_PI_2;
        } else {
            self.pitch = sin_p.asin();
        };

        // If P is too close to 0 then we have hit Gimbal lock */
        if sin_p > 0.999 {
            self.heading = (-matrix.zy).atan2(matrix.xx);
            self.roll = 0.0;
        } else {
            self.heading = matrix.zx.atan2(matrix.zz);
            self.roll = matrix.xy.atan2(matrix.yy);
        }
    }

    // /// Initializes a `self` rotation with the equivalent rotation
    // /// represented by the given `quaternion`.
    // ///
    // /// ## `quaternion`
    // /// A `Euler` with the rotation to initialize with
    // NOT YET IMPLEMENTED
    // pub fn init_from_quaternion(&mut self, quaternion: &Quaternion) {
    //     unsafe {
    //         ffi::euler_init_from_quaternion(
    //             self.to_glib_none_mut().0,
    //             quaternion.to_glib_none().0,
    //         );
    //     }
    // }

    fn equal(v1: &Self, v2: &Self) -> bool {
        let error_margin = std::f32::EPSILON;
        (v1.heading - v2.heading).abs() < error_margin
            && (v1.pitch - v2.pitch).abs() < error_margin
            && (v1.roll - v2.roll).abs() < error_margin
    }
}

impl PartialEq for Euler {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        Euler::equal(self, other)
    }
}

impl Eq for Euler {}
