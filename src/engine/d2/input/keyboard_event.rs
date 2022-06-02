use super::Key;

/// Represents an event coming from a physical key press.
///  *
/// NOTE: For performance reasons, KeyboardEvent instances are reused. Use `clone()` to
/// retain a reference to an event.
#[derive(Default, Clone, Debug)]
pub struct KeyboardEvent {
    /// The key that caused this event.
    pub key: Option<Key>,

    /// An incrementing ID unique to every dispatched key event.
    pub id: i32,
}

impl KeyboardEvent {
    pub fn new() -> Self {
        Self { id: 0, key: None }
    }

    pub fn init(&mut self, id: i32, key: Option<Key>) {
        self.id = id;
        self.key = key;
    }
}
