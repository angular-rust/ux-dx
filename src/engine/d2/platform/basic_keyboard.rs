use std::collections::HashMap;

use crate::engine::d2::{
    input::{Key, KeyboardEvent},
    subsystem::KeyboardSystem,
    util::{Signal0, Signal1},
};

use super::KeyCodes;

pub struct BasicKeyboard {
    pub down: Signal1<KeyboardEvent>,
    pub up: Signal1<KeyboardEvent>,
    pub back_button: Signal0,

    // static
    shared_event: KeyboardEvent, // = KeyboardEvent::new();

    key_states: HashMap<i32, bool>,
}

impl BasicKeyboard {
    pub fn new() -> Self {
        Self {
            down: Signal1::new(None),
            up: Signal1::new(None),
            back_button: Signal0::new(None),
            key_states: HashMap::new(),
            shared_event: KeyboardEvent::new(),
        }
    }

    #[inline]
    fn is_code_down(&self, key_code: i32) -> bool {
        self.key_states.contains_key(&key_code)
    }

    /// Called by the platform to handle a down event.
    /// @return Whether default action should be prevented.
    pub fn submit_down(&mut self, key_code: i32) -> bool {
        if key_code == KeyCodes::BACK {
            if self.back_button.inner.has_listeners() {
                self.back_button.emit();
                return true;
            }
            return false; // No preventDefault
        }

        if !self.is_code_down(key_code) {
            self.key_states.insert(key_code, true);
            self.shared_event
                .init(self.shared_event.id + 1, Some(KeyCodes::to_key(key_code)));
            self.down.emit(self.shared_event.clone());
        }

        true
    }

    /// Called by the platform to handle an up event.
    pub fn submit_up(&mut self, key_code: i32) {
        if self.is_code_down(key_code) {
            self.key_states.remove(&key_code);
            self.shared_event
                .init(self.shared_event.id + 1, Some(KeyCodes::to_key(key_code)));
            self.up.emit(self.shared_event.clone());
        }
    }
}

impl KeyboardSystem for BasicKeyboard {
    fn back_button(&self) -> &Signal0 {
        &self.back_button
    }

    fn down_signal(&self) -> &Signal1<KeyboardEvent> {
        &self.down
    }

    fn up_signal(&self) -> &Signal1<KeyboardEvent> {
        &self.up
    }

    fn is_down(&self, key: Key) -> bool {
        self.is_code_down(KeyCodes::to_key_code(key))
    }

    fn is_supported(&self) -> bool {
        true
    }
}
