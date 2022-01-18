use std::rc::Rc;

use super::EventSource;

/// Represents a touch screen contact point, such as a finger. It is possible to retain a reference
/// to a TouchPoint, and track changes to it over time.
#[derive(Default, Clone, Debug)]
pub struct TouchPoint {
    /// The X position of the touch, in view (stage) coordinates. This value is modified when the
    /// point moves.
    pub view_x: f32,

    /// The Y position of the touch, in view (stage) coordinates. This value is modified when the
    /// point moves.
    pub view_y: f32,

    /// An identifier unique to this touch.
    pub id: i32,
    // Cached to avoid lots of allocation
    pub source: Option<Box<EventSource>>,
}

impl TouchPoint {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            // source: None, //EventSource::Touch(self)
            ..Default::default()
        }
    }

    pub fn init(&mut self, view_x: f32, view_y: f32) {
        self.view_x = view_x;
        self.view_y = view_y;
    }
}

impl PartialEq for TouchPoint {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
