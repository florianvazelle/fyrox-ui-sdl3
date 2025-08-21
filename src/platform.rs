//! SDL3 -> Fyrox UI adapter: feed SDL events into Fyrox UI.

use fyrox_ui::UserInterface;
use fyrox_ui::core::algebra::Vector2;
use fyrox_ui::message::{ButtonState, KeyCode, KeyboardModifiers, MouseButton, OsEvent};

use sdl3::{
    event::Event,
    keyboard::{Mod, Scancode},
    mouse::MouseButton as SdlMouseButton,
    video::Window,
};

/// SDL3 backend platform state for Fyrox UI.
pub struct Platform;

impl Default for Platform {
    fn default() -> Self {
        Self::new()
    }
}

impl Platform {
    /// Create a new platform adapter.
    pub fn new() -> Self {
        Self
    }

    /// Handle a single SDL3 event and forward it to Fyrox UI as an `OsEvent`.
    pub fn handle_event(&mut self, ui: &mut UserInterface, event: &Event) -> bool {
        match *event {
            Event::MouseWheel { x, y, .. } => {
                ui.process_os_event(&OsEvent::MouseWheel(x, y));
                true
            }

            Event::MouseButtonDown { mouse_btn, .. } => {
                if let Some(btn) = map_mouse_button(mouse_btn) {
                    ui.process_os_event(&OsEvent::MouseInput {
                        button: btn,
                        state: ButtonState::Pressed,
                    });
                }
                true
            }

            Event::MouseButtonUp { mouse_btn, .. } => {
                if let Some(btn) = map_mouse_button(mouse_btn) {
                    ui.process_os_event(&OsEvent::MouseInput {
                        button: btn,
                        state: ButtonState::Released,
                    });
                }
                true
            }

            Event::TextInput { ref text, .. } => {
                // SDL may deliver multiple UTF-8 chars at once; forward each as a separate key text event.
                for ch in text.chars() {
                    ui.process_os_event(&OsEvent::KeyboardInput {
                        button: KeyCode::Unknown,
                        state: ButtonState::Pressed,
                        text: ch.to_string(),
                    });
                }
                true
            }

            Event::KeyDown {
                scancode: Some(sc),
                keymod,
                ..
            } => {
                // Send modifiers separately (Fyrox models them as an independent event)
                ui.process_os_event(&OsEvent::KeyboardModifiers(map_modifiers(keymod)));
                if let Some(key) = map_scancode(sc) {
                    ui.process_os_event(&OsEvent::KeyboardInput {
                        button: key,
                        state: ButtonState::Pressed,
                        text: String::new(),
                    });
                }
                true
            }

            Event::KeyUp {
                scancode: Some(sc),
                keymod,
                ..
            } => {
                ui.process_os_event(&OsEvent::KeyboardModifiers(map_modifiers(keymod)));
                if let Some(key) = map_scancode(sc) {
                    ui.process_os_event(&OsEvent::KeyboardInput {
                        button: key,
                        state: ButtonState::Released,
                        text: String::new(),
                    });
                }
                true
            }

            Event::MouseMotion { x, y, .. } => {
                ui.process_os_event(&OsEvent::CursorMoved {
                    position: Vector2::new(x, y),
                });
                true
            }

            _ => false,
        }
    }
}

/// Returns `true` if the provided event is associated with the provided window.
pub fn filter_event(window: &Window, event: &Event) -> bool {
    Some(window.id()) == event.get_window_id()
}

fn map_mouse_button(btn: SdlMouseButton) -> Option<MouseButton> {
    Some(match btn {
        SdlMouseButton::Left => MouseButton::Left,
        SdlMouseButton::Right => MouseButton::Right,
        SdlMouseButton::Middle => MouseButton::Middle,
        SdlMouseButton::X1 => MouseButton::Back,
        SdlMouseButton::X2 => MouseButton::Forward,
        SdlMouseButton::Unknown => return None,
    })
}

fn map_modifiers(m: Mod) -> KeyboardModifiers {
    KeyboardModifiers {
        alt: m.intersects(Mod::LALTMOD | Mod::RALTMOD),
        shift: m.intersects(Mod::LSHIFTMOD | Mod::RSHIFTMOD),
        control: m.intersects(Mod::LCTRLMOD | Mod::RCTRLMOD),
        system: m.intersects(Mod::LGUIMOD | Mod::RGUIMOD),
    }
}

fn map_scancode(sc: Scancode) -> Option<KeyCode> {
    use KeyCode as K;
    Some(match sc {
        // Letters
        Scancode::A => K::KeyA,
        Scancode::B => K::KeyB,
        Scancode::C => K::KeyC,
        Scancode::D => K::KeyD,
        Scancode::E => K::KeyE,
        Scancode::F => K::KeyF,
        Scancode::G => K::KeyG,
        Scancode::H => K::KeyH,
        Scancode::I => K::KeyI,
        Scancode::J => K::KeyJ,
        Scancode::K => K::KeyK,
        Scancode::L => K::KeyL,
        Scancode::M => K::KeyM,
        Scancode::N => K::KeyN,
        Scancode::O => K::KeyO,
        Scancode::P => K::KeyP,
        Scancode::Q => K::KeyQ,
        Scancode::R => K::KeyR,
        Scancode::S => K::KeyS,
        Scancode::T => K::KeyT,
        Scancode::U => K::KeyU,
        Scancode::V => K::KeyV,
        Scancode::W => K::KeyW,
        Scancode::X => K::KeyX,
        Scancode::Y => K::KeyY,
        Scancode::Z => K::KeyZ,
        // Digits (top row)
        Scancode::_0 => K::Digit0,
        Scancode::_1 => K::Digit1,
        Scancode::_2 => K::Digit2,
        Scancode::_3 => K::Digit3,
        Scancode::_4 => K::Digit4,
        Scancode::_5 => K::Digit5,
        Scancode::_6 => K::Digit6,
        Scancode::_7 => K::Digit7,
        Scancode::_8 => K::Digit8,
        Scancode::_9 => K::Digit9,
        // Punctuation / symbols
        Scancode::Minus => K::Minus,
        Scancode::Equals => K::Equal,
        Scancode::LeftBracket => K::BracketLeft,
        Scancode::RightBracket => K::BracketRight,
        Scancode::Backslash => K::Backslash,
        Scancode::Grave => K::Backquote,
        Scancode::Semicolon => K::Semicolon,
        Scancode::Apostrophe => K::Quote,
        Scancode::Comma => K::Comma,
        Scancode::Period => K::Period,
        Scancode::Slash => K::Slash,
        // Editing / navigation
        Scancode::Return => K::Enter,
        Scancode::Escape => K::Escape,
        Scancode::Backspace => K::Backspace,
        Scancode::Tab => K::Tab,
        Scancode::Space => K::Space,
        Scancode::Insert => K::Insert,
        Scancode::Delete => K::Delete,
        Scancode::Home => K::Home,
        Scancode::End => K::End,
        Scancode::PageUp => K::PageUp,
        Scancode::PageDown => K::PageDown,
        Scancode::Up => K::ArrowUp,
        Scancode::Down => K::ArrowDown,
        Scancode::Left => K::ArrowLeft,
        Scancode::Right => K::ArrowRight,
        // Function keys
        Scancode::F1 => K::F1,
        Scancode::F2 => K::F2,
        Scancode::F3 => K::F3,
        Scancode::F4 => K::F4,
        Scancode::F5 => K::F5,
        Scancode::F6 => K::F6,
        Scancode::F7 => K::F7,
        Scancode::F8 => K::F8,
        Scancode::F9 => K::F9,
        Scancode::F10 => K::F10,
        Scancode::F11 => K::F11,
        Scancode::F12 => K::F12,
        // Modifiers
        Scancode::LShift => K::ShiftLeft,
        Scancode::RShift => K::ShiftRight,
        Scancode::LCtrl => K::ControlLeft,
        Scancode::RCtrl => K::ControlRight,
        Scancode::LAlt => K::AltLeft,
        Scancode::RAlt => K::AltRight,
        Scancode::LGui => K::SuperLeft,
        Scancode::RGui => K::SuperRight,
        // Keypad (map common ones)
        Scancode::Kp0 => K::Numpad0,
        Scancode::Kp1 => K::Numpad1,
        Scancode::Kp2 => K::Numpad2,
        Scancode::Kp3 => K::Numpad3,
        Scancode::Kp4 => K::Numpad4,
        Scancode::Kp5 => K::Numpad5,
        Scancode::Kp6 => K::Numpad6,
        Scancode::Kp7 => K::Numpad7,
        Scancode::Kp8 => K::Numpad8,
        Scancode::Kp9 => K::Numpad9,
        Scancode::KpPlus => K::NumpadAdd,
        Scancode::KpMinus => K::NumpadSubtract,
        Scancode::KpMultiply => K::NumpadMultiply,
        Scancode::KpDivide => K::NumpadDivide,
        Scancode::KpEnter => K::NumpadEnter,
        Scancode::KpPeriod => K::NumpadDecimal,
        // Fallback
        _ => return None,
    })
}
