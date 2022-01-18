use cgmath::Vector4;

#[repr(C)]
pub struct DirectionalLight {
    pub direction: Vector4<f32>,
    pub illuminance: Vector4<f32>, // in lx = lm / m^2
}

#[repr(C)]
pub struct PointLight {
    pub position: Vector4<f32>,
    // luminos in flux (in lm)
    pub luminos: Vector4<f32>,
}

#[repr(C)]
pub struct SpotLight {
    pub position: Vector4<f32>,
    // luminos in flux (in lm)
    pub luminos: Vector4<f32>, // seems it chould be vec3 for aligning
    pub cuttoff: f32,
}

impl From<DirectionalLight> for Light {
    fn from(value: DirectionalLight) -> Self {
        Light::Directional(value)
    }
}

impl From<PointLight> for Light {
    fn from(value: PointLight) -> Self {
        Light::Point(value)
    }
}

impl From<SpotLight> for Light {
    fn from(value: SpotLight) -> Self {
        Light::Spot(value)
    }
}

pub enum Light {
    Directional(DirectionalLight),
    Point(PointLight),
    Spot(SpotLight),
}

#[derive(Default)]
pub struct LightManager {
    pub directional_lights: Vec<DirectionalLight>,
    pub point_lights: Vec<PointLight>,
    pub spot_lights: Vec<SpotLight>,
}

impl LightManager {
    pub fn add_light<T: Into<Light>>(&mut self, light: T) {
        use Light::*;
        match light.into() {
            Directional(light) => {
                self.directional_lights.push(light);
            }
            Point(light) => {
                self.point_lights.push(light);
            }
            Spot(light) => {
                self.spot_lights.push(light);
            }
        }
    }
}
