use crate::engine::d2::{
    input::{MouseButton, MouseCursor, MouseEvent},
    util::Signal1,
};

/// Functions related to the environment's mouse.
pub trait MouseSystem {
    /// True if the environment has a mouse.
    fn is_supported(&self) -> bool;

    /// Emitted when a mouse button is pressed down.
    fn down_signal(&self) -> &Signal1<MouseEvent>;

    /// Emitted when the mouse cursor is moved while over the stage.
    fn move_signal(&self) -> &Signal1<MouseEvent>;

    /// Emitted when a mouse button is released.
    fn up_signal(&self) -> &Signal1<MouseEvent>;

    /// A velocity emitted when the mouse wheel or trackpad is scrolled. A positive value is an
    /// upward scroll, negative is a downward scroll. Typically, each scroll wheel "click" equates to
    /// 1 velocity.
    fn scroll_signal(&self) -> &Signal1<f32>;

    /// The last recorded X coordinate of the mouse.
    fn x(&self) -> f32;

    /// The last recorded Y coordinate of the mouse.
    fn y(&self) -> f32;

    /// The style of the mouse cursor.
    fn cursor(&self) -> MouseCursor;

    /// @returns True if the given button is currently being held down.
    fn is_down(&self, button: MouseButton) -> bool;
}
