//! Camera implementations

use cgmath::Point3;

use crate::utils::clamp;

mod arcball;
pub use self::arcball::*;

mod first_person;
pub use self::first_person::*;

pub enum CameraBehaviour {
    FirstPerson,
    Spectator,
    Flight,
    Orbit,
}

// Spherical coordinate system
#[derive(Clone, Copy)]
pub struct Camera {
    theta: f32, // polar angle
    phi: f32,   // azimuthal angle
    r: f32,     // radial distance (distance to origin)
}

impl Camera {
    pub fn position(&self) -> Point3<f32> {
        Point3::new(
            self.r * self.phi.sin() * self.theta.sin(),
            self.r * self.phi.cos(),
            self.r * self.phi.sin() * self.theta.cos(),
        )
    }
}

impl Camera {
    pub fn rotate(&mut self, theta: f32, phi: f32) {
        self.theta += theta;
        let phi = self.phi + phi;
        self.phi = clamp(phi, 10.0_f32.to_radians(), 170.0_f32.to_radians());
    }

    pub fn forward(&mut self, r: f32) {
        self.r -= r;
    }
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
            theta: 0.0_f32.to_radians(),
            phi: 45.0_f32.to_radians(),
            r: 3.0,
        }
    }
}
