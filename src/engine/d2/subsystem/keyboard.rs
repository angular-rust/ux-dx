use crate::engine::d2::{
    input::{Key, KeyboardEvent},
    util::{Signal0, Signal1},
};

/// Functions related to the environment's physical keyboard.
pub trait KeyboardSystem {
    /// Whether the environment has a physical keyboard. Phones and tablets will generally return
    /// false here.
    fn is_supported(&self) -> bool;

    /// Emitted when a key is pressed down.
    fn down_signal(&self) -> &Signal1<KeyboardEvent>;

    /// Emitted when a key is released.
    fn up_signal(&self) -> &Signal1<KeyboardEvent>;

    /// Emitted when a hardware back button is pressed. If no listeners are connected to this signal
    /// when the back button is pressed, the platform's default action will be taken (which is
    /// usually to close the app). Only supported on Android.
    fn back_button(&self) -> &Signal0;

    /// @returns True if the given key is currently being held down.
    fn is_down(&self, key: Key) -> bool;
}
