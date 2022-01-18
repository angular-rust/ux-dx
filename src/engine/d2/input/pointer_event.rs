use std::rc::Rc;

use crate::engine::d2::display::Sprite;

use super::{MouseEvent, TouchPoint};

#[derive(Clone, Debug)]
pub enum EventSource {
    Mouse { event: MouseEvent },
    Touch { point: TouchPoint },
}

/// Represents an event coming from a pointing device, such as a mouse or finger.
///  *
/// NOTE: For performance reasons, PointerEvent instances are reused. Use `clone()` to
/// retain a reference to an event.

#[derive(Default, Clone, Debug)]
pub struct PointerEvent {
    /// The X position of the pointing device, in view (stage) coordinates.
    pub view_x: f32,

    /// The Y position of the pointing device, in view (stage) coordinates.
    pub view_y: f32,

    /// The deepest sprite lying under the pointer that caused the event, if any. The hit sprite does
    /// not necessarily have a pointer event listener connected to it. This event starts at the hit
    /// sprite, and propagates upwards to its parents.
    pub hit: Option<Sprite>,

    /// The source that this event originated from. This can be used to determine if the event came
    /// from a mouse or a touch.
    pub source: Option<EventSource>,

    /// An incrementing ID unique to every dispatched pointer event.
    pub id: i32,

    pub stopped: bool,
}

impl PointerEvent {
    pub fn new() -> Self {
        Self {
            id: 0,
            view_x: 0.0,
            view_y: 0.0,
            hit: None,
            source: None,
            stopped: false,
        }
    }

    /// Prevents this PointerEvent from propagating up to parent sprites and the top-level Pointer
    /// signal. Other listeners for this event on the current sprite will still fire.
    #[inline]
    pub fn stop_propagation(&mut self) {
        self.stopped = true;
    }

    pub fn init(&mut self, id: i32, view_x: f32, view_y: f32, hit: Option<Sprite>, source: Option<EventSource>) {
        self.id = id;
        self.view_x = view_x;
        self.view_y = view_y;
        self.hit = hit;
        self.source = source;
        self.stopped = false;
    }
}
