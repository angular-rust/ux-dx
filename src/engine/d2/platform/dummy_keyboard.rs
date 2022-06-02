use crate::engine::d2::{
    input::{Key, KeyboardEvent},
    subsystem::KeyboardSystem,
    util::{Signal0, Signal1},
};

pub struct DummyKeyboard {
    pub down: Signal1<KeyboardEvent>,
    pub up: Signal1<KeyboardEvent>,
    pub back_button: Signal0,
}

impl DummyKeyboard {
    pub fn new() -> Self {
        Self {
            down: Signal1::new(None),
            up: Signal1::new(None),
            back_button: Signal0::new(None),
        }
    }
}

impl KeyboardSystem for DummyKeyboard {
    fn back_button(&self) -> &Signal0 {
        &self.back_button
    }

    fn down_signal(&self) -> &Signal1<KeyboardEvent> {
        &self.down
    }

    fn up_signal(&self) -> &Signal1<KeyboardEvent> {
        &self.up
    }

    fn is_supported(&self) -> bool {
        false
    }

    fn is_down(&self, key: Key) -> bool {
        false
    }
}
