use crate::engine::d2::input::MouseButton;

/// Mouse button codes used internally.
pub struct MouseCodes {}

impl MouseCodes {
    const LEFT: u32 = 0;
    const MIDDLE: u32 = 1;
    const RIGHT: u32 = 2;

    // static
    pub fn to_button(button_code: u32) -> MouseButton {
        match button_code {
            Self::LEFT => MouseButton::Left,
            Self::MIDDLE => MouseButton::Middle,
            Self::RIGHT => MouseButton::Right,
            _ => MouseButton::Unknown { button_code },
        }
    }

    // static
    pub fn to_button_code(button: MouseButton) -> u32 {
        match button {
            MouseButton::Left => Self::LEFT,
            MouseButton::Middle => Self::MIDDLE,
            MouseButton::Right => Self::RIGHT,
            MouseButton::Unknown { button_code } => button_code,
        }
    }
}
