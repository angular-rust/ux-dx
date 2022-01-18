/// All the possible mouse buttons that can be handled. Use Unknown to handle any platform-specific
/// buttons not supported here.

#[derive(Copy, Clone, Debug)]
pub enum MouseButton {
    Left,
    Middle,
    Right,

    /// Used if the environment sends an unknown button.
    Unknown {
        button_code: u32,
    },
}
