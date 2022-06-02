use crate::engine::d2::input::Key;

/// Key codes.
pub struct KeyCodes {}

impl KeyCodes {
    pub const A: i32 = 65;
    pub const B: i32 = 66;
    pub const C: i32 = 67;
    pub const D: i32 = 68;
    pub const E: i32 = 69;
    pub const F: i32 = 70;
    pub const G: i32 = 71;
    pub const H: i32 = 72;
    pub const I: i32 = 73;
    pub const J: i32 = 74;
    pub const K: i32 = 75;
    pub const L: i32 = 76;
    pub const M: i32 = 77;
    pub const N: i32 = 78;
    pub const O: i32 = 79;
    pub const P: i32 = 80;
    pub const Q: i32 = 81;
    pub const R: i32 = 82;
    pub const S: i32 = 83;
    pub const T: i32 = 84;
    pub const U: i32 = 85;
    pub const V: i32 = 86;
    pub const W: i32 = 87;
    pub const X: i32 = 88;
    pub const Y: i32 = 89;
    pub const Z: i32 = 90;

    pub const NUMBER_0: i32 = 48;
    pub const NUMBER_1: i32 = 49;
    pub const NUMBER_2: i32 = 50;
    pub const NUMBER_3: i32 = 51;
    pub const NUMBER_4: i32 = 52;
    pub const NUMBER_5: i32 = 53;
    pub const NUMBER_6: i32 = 54;
    pub const NUMBER_7: i32 = 55;
    pub const NUMBER_8: i32 = 56;
    pub const NUMBER_9: i32 = 57;

    pub const NUMPAD_0: i32 = 96;
    pub const NUMPAD_1: i32 = 97;
    pub const NUMPAD_2: i32 = 98;
    pub const NUMPAD_3: i32 = 99;
    pub const NUMPAD_4: i32 = 100;
    pub const NUMPAD_5: i32 = 101;
    pub const NUMPAD_6: i32 = 102;
    pub const NUMPAD_7: i32 = 103;
    pub const NUMPAD_8: i32 = 104;
    pub const NUMPAD_9: i32 = 105;

    pub const NUMPAD_ADD: i32 = 107;
    pub const NUMPAD_DECIMAL: i32 = 110;
    pub const NUMPAD_DIVIDE: i32 = 111;
    pub const NUMPAD_ENTER: i32 = 108;
    pub const NUMPAD_MULTIPLY: i32 = 106;
    pub const NUMPAD_SUBTRACT: i32 = 109;

    pub const F1: i32 = 112;
    pub const F2: i32 = 113;
    pub const F3: i32 = 114;
    pub const F4: i32 = 115;
    pub const F5: i32 = 116;
    pub const F6: i32 = 117;
    pub const F7: i32 = 118;
    pub const F8: i32 = 119;
    pub const F9: i32 = 120;
    pub const F10: i32 = 121;
    pub const F11: i32 = 122;
    pub const F12: i32 = 123;
    pub const F13: i32 = 124;
    pub const F14: i32 = 125;
    pub const F15: i32 = 126;

    pub const LEFT: i32 = 37;
    pub const UP: i32 = 38;
    pub const RIGHT: i32 = 39;
    pub const DOWN: i32 = 40;

    pub const ALT: i32 = 18;
    pub const BACKQUOTE: i32 = 192;
    pub const BACKSLASH: i32 = 220;
    pub const BACKSPACE: i32 = 8;
    pub const CAPS_LOCK: i32 = 20;
    pub const COMMA: i32 = 188;
    pub const COMMAND: i32 = 15;
    pub const CONTROL: i32 = 17;
    pub const DELETE: i32 = 46;
    pub const END: i32 = 35;
    pub const ENTER: i32 = 13;
    pub const EQUALS: i32 = 187;
    pub const ESCAPE: i32 = 27;
    pub const HOME: i32 = 36;
    pub const INSERT: i32 = 45;
    pub const LEFT_BRACKET: i32 = 219;
    pub const MINUS: i32 = 189;
    pub const PAGE_DOWN: i32 = 34;
    pub const PAGE_UP: i32 = 33;
    pub const PERIOD: i32 = 190;
    pub const QUOTE: i32 = 222;
    pub const RIGHT_BRACKET: i32 = 221;
    pub const SEMICOLON: i32 = 186;
    pub const SHIFT: i32 = 16;
    pub const SLASH: i32 = 191;
    pub const SPACE: i32 = 32;
    pub const TAB: i32 = 9;

    // Android keys (AIR only)
    pub const BACK: i32 = 0x01000016;
    pub const MENU: i32 = 0x01000012;
    pub const SEARCH: i32 = 0x0100001f;

    // static
    pub fn to_key(keycode: i32) -> Key {
        match keycode {
            Self::A => Key::A,
            Self::B => Key::B,
            Self::C => Key::C,
            Self::D => Key::D,
            Self::E => Key::E,
            Self::F => Key::F,
            Self::G => Key::G,
            Self::H => Key::H,
            Self::I => Key::I,
            Self::J => Key::J,
            Self::K => Key::K,
            Self::L => Key::L,
            Self::M => Key::M,
            Self::N => Key::N,
            Self::O => Key::O,
            Self::P => Key::P,
            Self::Q => Key::Q,
            Self::R => Key::R,
            Self::S => Key::S,
            Self::T => Key::T,
            Self::U => Key::U,
            Self::V => Key::V,
            Self::W => Key::W,
            Self::X => Key::X,
            Self::Y => Key::Y,
            Self::Z => Key::Z,

            Self::NUMBER_0 => Key::Number0,
            Self::NUMBER_1 => Key::Number1,
            Self::NUMBER_2 => Key::Number2,
            Self::NUMBER_3 => Key::Number3,
            Self::NUMBER_4 => Key::Number4,
            Self::NUMBER_5 => Key::Number5,
            Self::NUMBER_6 => Key::Number6,
            Self::NUMBER_7 => Key::Number7,
            Self::NUMBER_8 => Key::Number8,
            Self::NUMBER_9 => Key::Number9,

            Self::NUMPAD_0 => Key::Numpad0,
            Self::NUMPAD_1 => Key::Numpad1,
            Self::NUMPAD_2 => Key::Numpad2,
            Self::NUMPAD_3 => Key::Numpad3,
            Self::NUMPAD_4 => Key::Numpad4,
            Self::NUMPAD_5 => Key::Numpad5,
            Self::NUMPAD_6 => Key::Numpad6,
            Self::NUMPAD_7 => Key::Numpad7,
            Self::NUMPAD_8 => Key::Numpad8,
            Self::NUMPAD_9 => Key::Numpad9,

            Self::NUMPAD_ADD => Key::NumpadAdd,
            Self::NUMPAD_DECIMAL => Key::NumpadDecimal,
            Self::NUMPAD_DIVIDE => Key::NumpadDivide,
            Self::NUMPAD_ENTER => Key::NumpadEnter,
            Self::NUMPAD_MULTIPLY => Key::NumpadMultiply,
            Self::NUMPAD_SUBTRACT => Key::NumpadSubtract,

            Self::F1 => Key::F1,
            Self::F2 => Key::F2,
            Self::F3 => Key::F3,
            Self::F4 => Key::F4,
            Self::F5 => Key::F5,
            Self::F6 => Key::F6,
            Self::F7 => Key::F7,
            Self::F8 => Key::F8,
            Self::F9 => Key::F9,
            Self::F10 => Key::F10,
            Self::F11 => Key::F11,
            Self::F12 => Key::F12,

            Self::LEFT => Key::Left,
            Self::UP => Key::Up,
            Self::RIGHT => Key::Right,
            Self::DOWN => Key::Down,

            Self::ALT => Key::Alt,
            Self::BACKQUOTE => Key::Backquote,
            Self::BACKSLASH => Key::Backslash,
            Self::BACKSPACE => Key::Backspace,
            Self::CAPS_LOCK => Key::CapsLock,
            Self::COMMA => Key::Comma,
            Self::COMMAND => Key::Command,
            Self::CONTROL => Key::Control,
            Self::DELETE => Key::Delete,
            Self::END => Key::End,
            Self::ENTER => Key::Enter,
            Self::EQUALS => Key::Equals,
            Self::ESCAPE => Key::Escape,
            Self::HOME => Key::Home,
            Self::INSERT => Key::Insert,
            Self::LEFT_BRACKET => Key::LeftBracket,
            Self::MINUS => Key::Minus,
            Self::PAGE_DOWN => Key::PageDown,
            Self::PAGE_UP => Key::PageUp,
            Self::PERIOD => Key::Period,
            Self::QUOTE => Key::Quote,
            Self::RIGHT_BRACKET => Key::RightBracket,
            Self::SEMICOLON => Key::Semicolon,
            Self::SHIFT => Key::Shift,
            Self::SLASH => Key::Slash,
            Self::SPACE => Key::Space,
            Self::TAB => Key::Tab,

            Self::MENU => Key::Menu,
            Self::SEARCH => Key::Search,
            _ => Key::Unknown { keycode },
        }
    }

    // static
    pub fn to_key_code(key: Key) -> i32 {
        match key {
            Key::A => Self::A,
            Key::B => Self::B,
            Key::C => Self::C,
            Key::D => Self::D,
            Key::E => Self::E,
            Key::F => Self::F,
            Key::G => Self::G,
            Key::H => Self::H,
            Key::I => Self::I,
            Key::J => Self::J,
            Key::K => Self::K,
            Key::L => Self::L,
            Key::M => Self::M,
            Key::N => Self::N,
            Key::O => Self::O,
            Key::P => Self::P,
            Key::Q => Self::Q,
            Key::R => Self::R,
            Key::S => Self::S,
            Key::T => Self::T,
            Key::U => Self::U,
            Key::V => Self::V,
            Key::W => Self::W,
            Key::X => Self::X,
            Key::Y => Self::Y,
            Key::Z => Self::Z,

            Key::Number0 => Self::NUMBER_0,
            Key::Number1 => Self::NUMBER_1,
            Key::Number2 => Self::NUMBER_2,
            Key::Number3 => Self::NUMBER_3,
            Key::Number4 => Self::NUMBER_4,
            Key::Number5 => Self::NUMBER_5,
            Key::Number6 => Self::NUMBER_6,
            Key::Number7 => Self::NUMBER_7,
            Key::Number8 => Self::NUMBER_8,
            Key::Number9 => Self::NUMBER_9,

            Key::Numpad0 => Self::NUMPAD_0,
            Key::Numpad1 => Self::NUMPAD_1,
            Key::Numpad2 => Self::NUMPAD_2,
            Key::Numpad3 => Self::NUMPAD_3,
            Key::Numpad4 => Self::NUMPAD_4,
            Key::Numpad5 => Self::NUMPAD_5,
            Key::Numpad6 => Self::NUMPAD_6,
            Key::Numpad7 => Self::NUMPAD_7,
            Key::Numpad8 => Self::NUMPAD_8,
            Key::Numpad9 => Self::NUMPAD_9,

            Key::NumpadAdd => Self::NUMPAD_ADD,
            Key::NumpadDecimal => Self::NUMPAD_DECIMAL,
            Key::NumpadDivide => Self::NUMPAD_DIVIDE,
            Key::NumpadEnter => Self::NUMPAD_ENTER,
            Key::NumpadMultiply => Self::NUMPAD_MULTIPLY,
            Key::NumpadSubtract => Self::NUMPAD_SUBTRACT,

            Key::F1 => Self::F1,
            Key::F2 => Self::F2,
            Key::F3 => Self::F3,
            Key::F4 => Self::F4,
            Key::F5 => Self::F5,
            Key::F6 => Self::F6,
            Key::F7 => Self::F7,
            Key::F8 => Self::F8,
            Key::F9 => Self::F9,
            Key::F10 => Self::F10,
            Key::F11 => Self::F11,
            Key::F12 => Self::F12,
            Key::F13 => Self::F13,
            Key::F14 => Self::F14,
            Key::F15 => Self::F15,

            Key::Left => Self::LEFT,
            Key::Up => Self::UP,
            Key::Right => Self::RIGHT,
            Key::Down => Self::DOWN,

            Key::Alt => Self::ALT,
            Key::Backquote => Self::BACKQUOTE,
            Key::Backslash => Self::BACKSLASH,
            Key::Backspace => Self::BACKSPACE,
            Key::CapsLock => Self::CAPS_LOCK,
            Key::Comma => Self::COMMA,
            Key::Command => Self::COMMAND,
            Key::Control => Self::CONTROL,
            Key::Delete => Self::DELETE,
            Key::End => Self::END,
            Key::Enter => Self::ENTER,
            Key::Equals => Self::EQUALS,
            Key::Escape => Self::ESCAPE,
            Key::Home => Self::HOME,
            Key::Insert => Self::INSERT,
            Key::LeftBracket => Self::LEFT_BRACKET,
            Key::Minus => Self::MINUS,
            Key::PageDown => Self::PAGE_DOWN,
            Key::PageUp => Self::PAGE_UP,
            Key::Period => Self::PERIOD,
            Key::Quote => Self::QUOTE,
            Key::RightBracket => Self::RIGHT_BRACKET,
            Key::Semicolon => Self::SEMICOLON,
            Key::Shift => Self::SHIFT,
            Key::Slash => Self::SLASH,
            Key::Space => Self::SPACE,
            Key::Tab => Self::TAB,

            Key::Menu => Self::MENU,
            Key::Search => Self::SEARCH,

            Key::Unknown { keycode } => keycode,
        }
    }
}
