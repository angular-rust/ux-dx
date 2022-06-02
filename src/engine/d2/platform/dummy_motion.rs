use crate::engine::d2::{
    input::{Acceleration, Attitude},
    subsystem::MotionSystem,
    util::Signal1,
};

pub struct DummyMotion {
    pub acceleration: Signal1<Acceleration>,
    pub acceleration_including_gravity: Signal1<Acceleration>,
    pub attitude: Signal1<Attitude>,
}

impl DummyMotion {
    pub fn new() -> Self {
        Self {
            acceleration: Signal1::new(None),
            acceleration_including_gravity: Signal1::new(None),
            attitude: Signal1::new(None),
        }
    }
}

impl MotionSystem for DummyMotion {
    fn acceleration(&self) -> &Signal1<Acceleration> {
        &self.acceleration
    }

    fn acceleration_including_gravity(&self) -> &Signal1<Acceleration> {
        &self.acceleration_including_gravity
    }

    fn attitude(&self) -> &Signal1<Attitude> {
        &self.attitude
    }

    fn is_acceleration_supported(&self) -> bool {
        false
    }

    fn is_attitude_supported(&self) -> bool {
        false
    }
}
