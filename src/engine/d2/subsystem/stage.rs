use crate::engine::d2::{
    display::Orientation,
    util::{Signal0, Value},
};

/// Functions related to the environment's display viewport.
pub trait StageSystem {
    /// The width of the stage viewport, in pixels.
    fn width(&self) -> i32;

    /// The height of the stage viewport, in pixels.
    fn height(&self) -> i32;

    /// The current screen orientation, or a wrapped None value if the environment doesn't support
    /// multiple orientations.
    fn orientation(&self) -> Value<Orientation>;

    /// True if the stage is currently fullscreen.
    fn fullscreen(&self) -> Value<bool>;

    /// Whether the stage may change its fullscreen state. False if the stage is fullscreen and can't
    /// go into windowed mode, or vise versa.
    fn is_fullscreen_supported(&self) -> bool;

    /// Emitted after the stage size changes, such as when the window is resized or the device is
    /// rotated.
    fn resize_signal(&self) -> Signal0;

    /// Request to lock the orientation, so that rotating the device will not adjust the screen. Has
    /// no effect if the environment doesn't support orientation locking.
    /// @param orient The orientation to lock to.
    fn lock_orientation(&self, orient: Orientation);

    /// Request to unlock the orientation, so that rotating the device will adjust the screen. Has no
    /// effect if the environment doesn't support orientation locking.
    fn unlock_orientation(&self);

    /// Request that the stage be resized to a certain size.
    fn request_resize(&self, width: i32, height: i32);

    /// Request that fullscreen be enabled or disabled. No effect if changing fullscreen is not
    /// supported.
    //  enable :bool = true
    fn request_fullscreen(&self, enable: bool);
}
