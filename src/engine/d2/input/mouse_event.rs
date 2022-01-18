use super::MouseButton;
/// Represents an event coming from a mouse.
///  *
/// NOTE: For performance reasons, MouseEvent instances are reused. Use `clone()` to
/// retain a reference to an event.
#[derive(Default, Clone, Debug)]

pub struct MouseEvent {
    /// The X position of the mouse, in view (stage) coordinates.
    pub view_x: f32,

    /// The Y position of the mouse, in view (stage) coordinates.
    pub view_y: f32,

    /// The mouse button that caused this event, or None for movement events.
    pub button: Option<MouseButton>,

    /// An incrementing ID unique to every dispatched mouse event.
    pub id: i32,
}

impl MouseEvent {
    pub fn new() -> Self {
        Self {
            id: 0,
            view_x: 0.0,
            view_y: 0.0,
            button: None,
        }
    }

    pub fn init(&mut self, id: i32, view_x: f32, view_y: f32, button: Option<MouseButton>) {
        self.id = id;
        self.view_x = view_x;
        self.view_y = view_y;
        self.button = button;
    }
}
