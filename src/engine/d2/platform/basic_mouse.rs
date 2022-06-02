use std::collections::HashMap;

use crate::engine::d2::{
    input::{EventSource, MouseButton, MouseCursor, MouseEvent},
    subsystem::MouseSystem,
    util::Signal1,
};

use super::{BasicPointer, MouseCodes};

pub struct BasicMouse {
    pub down: Signal1<MouseEvent>,
    pub move_: Signal1<MouseEvent>,
    pub up: Signal1<MouseEvent>,
    pub scroll: Signal1<f32>,

    // static
    shared_event: MouseEvent, // = MouseEvent::new();

    pointer: BasicPointer,
    source: EventSource,

    x: f32,
    y: f32,
    cursor: MouseCursor,
    button_states: HashMap<u32, bool>,
}

impl BasicMouse {
    pub fn new(pointer: BasicPointer) -> Self {
        let shared_event = MouseEvent::new();
        Self {
            pointer,
            source: EventSource::Mouse {
                event: shared_event.clone(),
            },

            down: Signal1::new(None),
            move_: Signal1::new(None),
            up: Signal1::new(None),
            scroll: Signal1::new(None),
            x: 0.0,
            y: 0.0,
            cursor: MouseCursor::Default,
            button_states: HashMap::new(),
            shared_event,
        }
    }

    pub fn set_cursor(&mut self, cursor: MouseCursor) {
        // See subclasses for implementation
        self.cursor = cursor;
    }

    pub fn submit_down(&mut self, view_x: f32, view_y: f32, button_code: u32) {
        if !self.is_code_down(button_code) {
            self.button_states.insert(button_code, true);

            // Init the MouseEvent, and let the Pointer system handle it before emitting our signal
            self.prepare(view_x, view_y, Some(MouseCodes::to_button(button_code)));
            self.pointer
                .submit_down(view_x, view_y, self.source.clone());
            self.down.emit(self.shared_event.clone());
        }
    }

    pub fn submit_move(&mut self, view_x: f32, view_y: f32) {
        self.prepare(view_x, view_y, None);
        self.pointer
            .submit_move(view_x, view_y, self.source.clone());
        self.move_.emit(self.shared_event.clone());
    }

    pub fn submit_up(&mut self, view_x: f32, view_y: f32, button_code: u32) {
        if self.is_code_down(button_code) {
            self.button_states.remove(&button_code);

            self.prepare(view_x, view_y, Some(MouseCodes::to_button(button_code)));
            self.pointer.submit_up(view_x, view_y, self.source.clone());
            self.up.emit(self.shared_event.clone());
        }
    }

    // Returns true if the scroll signal was handled
    pub fn submit_scroll(&mut self, view_x: f32, view_y: f32, velocity: f32) -> bool {
        self.x = view_x;
        self.y = view_y;
        if !self.scroll.inner.has_listeners() {
            return false;
        }
        self.scroll.emit(velocity);

        true
    }

    #[inline]
    fn is_code_down(&self, button_code: u32) -> bool {
        self.button_states.contains_key(&button_code)
    }

    fn prepare(&mut self, view_x: f32, view_y: f32, button: Option<MouseButton>) {
        self.x = view_x;
        self.y = view_y;
        self.shared_event
            .init(self.shared_event.id + 1, view_x, view_y, button);
    }
}

impl MouseSystem for BasicMouse {
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
        true
    }

    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn cursor(&self) -> MouseCursor {
        self.cursor
    }

    fn is_down(&self, button: MouseButton) -> bool {
        self.is_code_down(MouseCodes::to_button_code(button))
    }
}
