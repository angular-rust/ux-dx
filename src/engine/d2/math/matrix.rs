use cgmath::Point2;
use std::fmt;

/// A 2D transform matrix.
///  *
// ```
// [ m00 m01 m02 ]
// [ m10 m11 m12 ]
// [  0   0   1  ]
// ```

#[derive(Default, Copy, Clone, Debug)]
pub struct Matrix {
    pub m00: f32,
    pub m10: f32,
    pub m01: f32,
    pub m11: f32,
    pub m02: f32,
    pub m12: f32,
}

impl Matrix {
    pub fn new() -> Self {
        Self::identity()
    }

    pub fn set(&mut self, m00: f32, m10: f32, m01: f32, m11: f32, m02: f32, m12: f32) {
        self.m00 = m00;
        self.m01 = m01;
        self.m02 = m02;
        self.m10 = m10;
        self.m11 = m11;
        self.m12 = m12;
    }

    /// Sets this matrix to the identity matrix.
    pub fn identity() -> Self {
        let mut instance = Self::default();
        instance.set(1.0, 0.0, 0.0, 1.0, 0.0, 0.0);

        instance
    }

    /// Set this matrix to a translation, scale, and rotation, in that order.
    pub fn compose(&mut self, x: f32, y: f32, scale_x: f32, scale_y: f32, rotation: f32) {
        let sin = rotation.sin();
        let cos = rotation.cos();
        self.set(cos * scale_x, sin * scale_x, -sin * scale_y, cos * scale_y, x, y);
    }

    pub fn translate(&mut self, x: f32, y: f32) {
        self.m02 += self.m00 * x + self.m01 * y;
        self.m12 += self.m11 * y + self.m10 * x;
    }

    pub fn scale(&mut self, x: f32, y: f32) {
        self.m00 *= x;
        self.m10 *= x;
        self.m01 *= y;
        self.m11 *= y;
    }

    pub fn rotate(&mut self, rotation: f32) {
        let sin = rotation.sin();
        let cos = rotation.cos();

        let t00 = self.m00 * cos + self.m01 * sin;
        let t01 = -self.m00 * sin + self.m01 * cos;
        self.m00 = t00;
        self.m01 = t01;

        let t10 = self.m11 * sin + self.m10 * cos;
        let t11 = self.m11 * cos - self.m10 * sin;
        self.m10 = t10;
        self.m11 = t11;
    }

    /// @return Whether the matrix was inverted.
    pub fn invert(&mut self) -> bool {
        let det = self.determinant();
        if det == 0.0 {
            return false;
        }
        self.set(
            self.m11 / det,
            -self.m01 / det,
            -self.m10 / det,
            self.m00 / det,
            (self.m01 * self.m12 - self.m11 * self.m02) / det,
            (self.m10 * self.m02 - self.m00 * self.m12) / det,
        );

        true
    }

    pub fn transform(&self, x: f32, y: f32) -> Point2<f32> {
        Point2 {
            x: x * self.m00 + y * self.m01 + self.m02,
            y: x * self.m10 + y * self.m11 + self.m12,
        }
    }

    pub fn transform_array(&self, points: Vec<f32>, length: usize, result: &mut Vec<f32>) {
        let mut idx = 0;
        while idx < length {
            let x = points[idx];
            let y = points[idx + 1];
            result[idx] = x * self.m00 + y * self.m01 + self.m02;
            idx += 1;
            result[idx] = x * self.m10 + y * self.m11 + self.m12;
            idx += 1;
        }
    }

    /// Calculate the determinant of this matrix.
    pub fn determinant(&self) -> f32 {
        self.m00 * self.m11 - self.m01 * self.m10
    }

    /// Transforms a point by the inverse of this matrix, or return false if this matrix is not
    /// invertible.
    pub fn inverse_transform(&self, x: f32, y: f32) -> Option<Point2<f32>> {
        let det = self.determinant();
        if det == 0.0 {
            return None;
        }

        let x = x - self.m02;
        let y = y - self.m12;

        Some(Point2 {
            x: (x * self.m11 - y * self.m01) / det,
            y: (y * self.m00 - x * self.m10) / det,
        })
    }

    /// Multiply two matrices and return the result.
    //  static
    pub fn multiply(lhs: Matrix, rhs: Matrix) -> Matrix {
        let mut result = Matrix::new();

        // First row
        let mut a = lhs.m00 * rhs.m00 + lhs.m01 * rhs.m10;
        let mut b = lhs.m00 * rhs.m01 + lhs.m01 * rhs.m11;
        let mut c = lhs.m00 * rhs.m02 + lhs.m01 * rhs.m12 + lhs.m02;
        result.m00 = a;
        result.m01 = b;
        result.m02 = c;

        // Second row
        a = lhs.m10 * rhs.m00 + lhs.m11 * rhs.m10;
        b = lhs.m10 * rhs.m01 + lhs.m11 * rhs.m11;
        c = lhs.m10 * rhs.m02 + lhs.m11 * rhs.m12 + lhs.m12;
        result.m10 = a;
        result.m11 = b;
        result.m12 = c;

        result
    }
}

impl fmt::Display for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} {} \\ {} {} {}", self.m00, self.m01, self.m02, self.m10, self.m11, self.m12)
    }
}
