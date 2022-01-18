use std::{cell::RefCell, f32::consts::PI};

use super::{AnimatedFloat, Behavior};

struct SineProps {
    /// The speed, in seconds, it takes to animate between the starting value and the ending value (or the other way around)
    pub speed: AnimatedFloat,
    /// The number of times to animate between the starting value and the end value
    pub cycles: f32,
    /// The number of times to animate
    count: f32,
    /// The total distance between the start and end values
    distance: f32,
    /// The middle of the start and end position, stored for quicker math
    center: f32,
}
/// Controls an AnimatedFloat using a Sine wave, typically endlessly.
/// Useful for flashing a notification, or creating a throbbing animation effect without using a Script.
pub struct Sine {
    props: RefCell<SineProps>,
    // /// The end value
    // pub end: f32,
    // /// The starting value
    // pub start: f32,
}

impl Sine {
    /// Stores the half value of PI for quicker calculations.
    pub const HALF_PI: f32 = { 0.5 * PI };

    /// @param start The starting value for the animated float.
    /// @param end The last value for the animated float.
    /// @param speed The speed (in seconds) it takes to go from the start value to the end value.
    /// @param cycles The number of animation cycles to go through. A value of 0 will cycle forever,
    ///   whereas a value of 1 will go from the start position, to the end position, and back to start.
    /// @param offset The number of seconds to offset the animation. Useful for offseting the animation for a series of sine behaviors.
    // speed:f32 = 1, cycles:f32 = 0, offset:f32 = 0
    pub fn new(start: f32, end: f32, speed: f32, cycles: f32, offset: f32) -> Self {
        let distance = (start - end) * 0.5;

        let props = RefCell::new(SineProps {
            speed: AnimatedFloat::new(speed, None),
            cycles,
            count: Self::HALF_PI + offset * (PI / speed), // Start at the start value plus the seconds to offset.
            distance,
            center: end + distance,
        });

        Self {
            props,
            // start,
            // end,
        }
    }
}

impl Behavior for Sine {
    fn update(&self, dt: f32) -> f32 {
        let mut props = self.props.borrow_mut();

        props.speed.update(dt);
        props.count += dt * (PI / props.speed.get());
        if self.is_complete() {
            return props.center + PI * props.distance;
        }

        props.center + props.count.sin() * props.distance
    }

    fn is_complete(&self) -> bool {
        let props = self.props.borrow();
        props.cycles > 0.0 && ((props.count - Self::HALF_PI) / PI) * 0.5 >= props.cycles
    }
}
