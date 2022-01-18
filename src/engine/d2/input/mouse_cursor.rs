#[derive(Copy, Clone, Debug)]
pub enum MouseCursor {
    /// The default system cursor, typically an arrow.
    Default,

    /// The system cursor used when hovering over buttons and links, typically a hand.
    Button,

    /// An invisible cursor.
    None,
    // Image(texture :Texture, anchorX: i32, anchorY: i32);
}
