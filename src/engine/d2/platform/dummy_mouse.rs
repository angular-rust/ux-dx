use crate::engine::d2::{
    input::{MouseButton, MouseCursor, MouseEvent},
    subsystem::MouseSystem,
    util::Signal1,
};

pub struct DummyMouse {
    pub down: Signal1<MouseEvent>,
    pub move_: Signal1<MouseEvent>,
    pub up: Signal1<MouseEvent>,
    pub scroll: Signal1<f32>,
    cursor: MouseCursor,
}

impl DummyMouse {
    pub fn new() -> Self {
        Self {
            down: Signal1::new(None),
            move_: Signal1::new(None),
            up: Signal1::new(None),
            scroll: Signal1::new(None),
            cursor: MouseCursor::Default,
        }
    }

    pub fn set_cursor(&mut self, cursor: MouseCursor) {
        self.cursor = cursor;
    }
}

impl MouseSystem for DummyMouse {
    fn down_signal(&self) -> &Signal1<MouseEvent> {
        &self.down
    }

    fn move_signal(&self) -> &Signal1<MouseEvent> {
        &self.move_
    }

    fn scroll_signal(&self) -> &Signal1<f32> {
        &self.scroll
    }

    fn up_signal(&self) -> &Signal1<MouseEvent> {
        &self.up
    }

    fn is_supported(&self) -> bool {
        false
    }

    fn x(&self) -> f32 {
        0.0
    }

    fn y(&self) -> f32 {
        0.0
    }

    fn is_down(&self, button: MouseButton) -> bool {
        false
    }

    fn cursor(&self) -> MouseCursor {
        self.cursor
    }
}
