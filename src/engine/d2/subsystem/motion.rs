use crate::engine::d2::{
    input::{Acceleration, Attitude},
    util::Signal1,
};

/// Functions related to the device's motion sensors.
pub trait MotionSystem {
    /// Whether device acceleration events are supported. If true, the acceleration and/or
    /// accelerationIncludingGravity signals will be emitted.
    fn is_acceleration_supported(&self) -> bool;

    /// Periodically emits the device's current linear acceleration, excluding the pull of gravity.
    /// This will only be emitted if the device has a gyroscope.
    fn acceleration(&self) -> &Signal1<Acceleration>;

    /// Periodically emits the devices's current linear acceleration, including the pull of gravity.
    fn acceleration_including_gravity(&self) -> &Signal1<Acceleration>;

    /// Whether device orientation (attitude) events are supported.
    fn is_attitude_supported(&self) -> bool;

    /// Periodically emits the device's current attitude.
    fn attitude(&self) -> &Signal1<Attitude>;
}
