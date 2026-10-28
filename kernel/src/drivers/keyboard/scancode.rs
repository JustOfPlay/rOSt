

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Key {
    // Letters
    A, B, C, D, E, F, G, H, I, J, K, L, M,
    N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    // Number row
    Num0, Num1, Num2, Num3, Num4,
    Num5, Num6, Num7, Num8, Num9,

    // Common keys
    Escape, Enter, Space, Tab, Backspace,
    Minus, Equal, LeftBracket, RightBracket,
    Backslash, Semicolon, Apostrophe, Grave,
    Comma, Period, Slash, CapsLock,

    // Modifier keys
    LeftShift, RightShift,
    LeftCtrl, RightCtrl,
    LeftAlt, RightAlt,

    // Function keys
    F1, F2, F3, F4, F5, F6,
    F7, F8, F9, F10, F11, F12,

    // Navigation and special keys
    ArrowUp, ArrowDown, ArrowLeft, ArrowRight,
    Home, End, Insert, Delete, PageUp, PageDown,
    NumLock, ScrollLock, KeypadEnter, KeypadSlash,
    PrintScreen, Pause, Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyState {
    Pressed,
    Released,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    pub key: Key,
    pub state: KeyState,
}

impl core::fmt::Display for Key {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl core::fmt::Display for KeyState {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub struct ScancodeDecoder {
    extended: bool,
}

impl ScancodeDecoder {
    pub const fn new() -> Self {
        Self { extended: false }
    }

    pub fn decode(&mut self, byte: u8) -> Option<KeyEvent> {
        if byte == 0xE0 {
            self.extended = true;
            return None;
        }

        if byte == 0xE1 {
            self.extended = false;
            return Some(KeyEvent {
                key: Key::Pause,
                state: KeyState::Pressed,
            });
        }

        self.extended = false;

        let state = if byte & 0x80 != 0 {
            KeyState::Released
        } else {
            KeyState::Pressed
        };

        let code = byte & 0x7F;

        let key = if self.extended {
            match code {
                0x1C => Key::KeypadEnter,
                0x1D => Key::RightCtrl,
                0x35 => Key::KeypadSlash,
                0x38 => Key::RightAlt,
                0x47 => Key::Home,
                0x48 => Key::ArrowUp,
                0x49 => Key::PageUp,
                0x4B => Key::ArrowLeft,
                0x4D => Key::ArrowRight,
                0x4F => Key::End,
                0x50 => Key::ArrowDown,
                0x51 => Key::PageDown,
                0x52 => Key::Insert,
                0x53 => Key::Delete,
                0x5B => Key::Unknown, 
                0x5C => Key::Unknown,
                _ => Key::Unknown,
            }
        } else {
            match code {
                0x02 => Key::Num1,
                0x03 => Key::Num2,
                0x04 => Key::Num3,
                0x05 => Key::Num4,
                0x06 => Key::Num5,
                0x07 => Key::Num6,
                0x08 => Key::Num7,
                0x09 => Key::Num8,
                0x0A => Key::Num9,
                0x0B => Key::Num0,

                0x10 => Key::Q,
                0x11 => Key::W,
                0x12 => Key::E,
                0x13 => Key::R,
                0x14 => Key::T,
                0x15 => Key::Y,
                0x16 => Key::U,
                0x17 => Key::I,
                0x18 => Key::O,
                0x19 => Key::P,

                0x1E => Key::A,
                0x1F => Key::S,
                0x20 => Key::D,
                0x21 => Key::F,
                0x22 => Key::G,
                0x23 => Key::H,
                0x24 => Key::J,
                0x25 => Key::K,
                0x26 => Key::L,

                0x2C => Key::Z,
                0x2D => Key::X,
                0x2E => Key::C,
                0x2F => Key::V,
                0x30 => Key::B,
                0x31 => Key::N,
                0x32 => Key::M,

                0x01 => Key::Escape,
                0x0E => Key::Backspace,
                0x0F => Key::Tab,
                0x1C => Key::Enter,
                0x2A => Key::LeftShift,
                0x36 => Key::RightShift,
                0x1D => Key::LeftCtrl,
                0x38 => Key::LeftAlt,
                0x39 => Key::Space,
                0x3A => Key::CapsLock,

                0x3B => Key::F1,
                0x3C => Key::F2,
                0x3D => Key::F3,
                0x3E => Key::F4,
                0x3F => Key::F5,
                0x40 => Key::F6,
                0x41 => Key::F7,
                0x42 => Key::F8,
                0x43 => Key::F9,
                0x44 => Key::F10,
                0x57 => Key::F11,
                0x58 => Key::F12,

                0x0C => Key::Minus,
                0x0D => Key::Equal,
                0x1A => Key::LeftBracket,
                0x1B => Key::RightBracket,
                0x2B => Key::Backslash,
                0x27 => Key::Semicolon,
                0x28 => Key::Apostrophe,
                0x29 => Key::Grave,
                0x33 => Key::Comma,
                0x34 => Key::Period,
                0x35 => Key::Slash,

                0x45 => Key::NumLock,
                0x46 => Key::ScrollLock,
                0x37 => Key::PrintScreen,

                _ => Key::Unknown,
            }
        };

        Some(KeyEvent { key, state })
    }
}
