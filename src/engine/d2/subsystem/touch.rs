use crate::engine::d2::{input::TouchPoint, util::Signal1};

/// Functions related to the environment's touch screen.
pub trait TouchSystem {
    /// True if the environment has a touch screen.
    fn is_supported(&self) -> bool;

    /// The maximum number of touch points that can be detected at once.
    fn max_points(&self) -> i32;

    /// Emits a new TouchPoint when a finger presses down on the screen.
    fn down_signal(&self) -> &Signal1<TouchPoint>;

    /// Emits the modified TouchPoint when a finger changes position.
    fn move_signal(&self) -> &Signal1<TouchPoint>;

    /// Emits the removed TouchPoint when a finger is raised from the screen.
    fn up_signal(&self) -> &Signal1<TouchPoint>;

    /// The touch points currently pressed to the screen.
    fn points(&self) -> Vec<TouchPoint>;
}
