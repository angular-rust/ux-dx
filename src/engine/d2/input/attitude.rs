/// Three angles that represent the device's attitude, one around each axis.
#[derive(Default, Clone, Copy, Debug)]
pub struct Attitude {
    /// The angle in degrees around the X-axis; that is, how far the device is pitched forward or
    /// backward.
    pub pitch: f32,

    /// The angle in degrees around the Y-axis; that is, how far the device is rolled left or right.
    pub roll: f32,

    /// The angle in degrees around the Z-axis.
    pub azimuth: f32,
}

impl Attitude {
    fn new() -> Self {
        Default::default()
    }

    fn init(&mut self, pitch: f32, roll: f32, azimuth: f32) {
        self.pitch = pitch;
        self.roll = roll;
        self.azimuth = azimuth;
    }
}
