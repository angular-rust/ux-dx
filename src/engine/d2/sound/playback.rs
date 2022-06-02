use std::rc::Rc;

use crate::engine::d2::{
    animation::AnimatedFloat,
    util::{Disposable, Value},
};

use super::Sound;

/// Represents a currently playing sound.
pub trait Playback: Disposable {
    /// The volume of the sound being played, between 0 and 1 (inclusive).
    fn volume(&self) -> AnimatedFloat;

    fn set_volume(&self, val: AnimatedFloat);

    /// Whether the playback is currently paused. Playbacks are automatically paused while the app is
    /// hidden, such as when minimized or placed in a background browser tab.
    fn paused(&self) -> bool;

    fn set_paused(&self, val: bool);

    /// Whether the playback has finished playing or has been disposed. Looping playbacks will never
    /// complete naturally, and are complete only after being disposed.
    ///  *
    /// In environments that don't support audio, this will be true.
    ///  *
    /// Do not set this value! To pause the playback, set `paused`. To stop it completely, call
    /// `dispose()`.
    fn complete(&self) -> Value<bool>;

    fn set_complete(&self, val: Value<bool>);

    /// The current playback position in seconds.
    fn position(&self) -> f32;

    fn set_position(&self, pos: f32);

    /// The sound being played.
    fn sound(&self) -> Rc<dyn Sound>;
}
