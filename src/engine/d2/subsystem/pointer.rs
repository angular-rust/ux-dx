use crate::engine::d2::{input::PointerEvent, util::Signal1};

/// Functions related to the environment's pointing device. On desktop computers, this is a mouse. On
/// touch screens, it's a finger.
pub trait PointerSystem {
    /// True if the environment has a pointing device.
    fn is_supported(&self) -> bool;

    /// Emitted when the pointing device is pressed down (when the mouse button is held or a finger
    /// is pressed to the screen).
    fn down(&self) -> &Signal1<PointerEvent>;

    /// Emitted when the pointing device moves while over the stage.
    fn move_signal(&self) -> &Signal1<PointerEvent>;

    /// Emitted when the pointing device is released (when the mouse button is released or the finger
    /// is lifted from the screen).
    fn up_signal(&self) -> &Signal1<PointerEvent>;

    /// The last recorded X coordinate of the pointer.
    fn x(&self) -> f32;

    /// The last recorded Y coordinate of the pointer.
    fn y(&self) -> f32;

    /// True if the pointer is currently pressed down.
    fn is_down(&self) -> bool;
}
