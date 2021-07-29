use super::{Angle, Vector};
use cgmath::prelude::*;
use cgmath::{Matrix4, Rad, Vector3};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform(pub(crate) Matrix4<f32>);

impl Transform {
    pub fn identity() -> Self {
        Self(Matrix4::identity())
    }

    pub fn zero() -> Self {
        Self(Matrix4::zero())
    }

    pub fn translate(&self, vector: impl Into<Vector>) -> Self {
        let vector = vector.into();
        Self(Matrix4::from_translation(Vector3::new(vector.x, vector.y, 0.0)) * self.0)
    }

    pub fn rotate(&self, angle: impl Into<Angle>) -> Self {
        let angle = angle.into();

        Self(Matrix4::from_angle_z(Rad(angle.radians_value())) * self.0)
    }

    pub fn scale(&self, vector: impl Into<Vector>) -> Self {
        let vector = vector.into();

        Self(Matrix4::from_nonuniform_scale(vector.x as f32, vector.y, 1.0) * self.0)
    }

    pub fn inverse(&self) -> Self {
        let m = self.0.invert().unwrap();
        Self(m)
    }

    pub fn apply(&self, transform: impl Into<Transform>) -> Self {
        let transform = transform.into();
        Self(transform.0 * self.0)
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
