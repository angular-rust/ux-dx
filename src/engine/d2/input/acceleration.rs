/// A 3D vector that represents the linear acceleration being applied to the device.
#[derive(Default, Clone, Copy, Debug)]
pub struct Acceleration {
    /// The acceleration on the X-axis, in m/s^2.
    pub x: f32,

    /// The acceleration on the Y-axis, in m/s^2.
    pub y: f32,

    /// The acceleration on the Z-axis, in m/s^2.
    pub z: f32,
}

impl Acceleration {
    fn new() -> Self {
        Default::default()
    }

    fn init(&mut self, x: f32, y: f32, z: f32) {
        self.x = x;
        self.y = y;
        self.z = z;
    }
}
