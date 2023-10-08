use glium::glutin::event::VirtualKeyCode;
use serde::Deserialize;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Deserialize)]
pub struct KeyBinding {
    pub action: Actions,
    pub key: VirtualKeyCode,
}

#[derive(Debug, EnumIter, Deserialize, Clone, Copy)]
pub enum Actions {
    SaveShip,
    LoadShip,
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    RotateClockwise,
    RotateCounterClockwise,
    ZoomIn,
    ZoomOut,
}

pub fn get_default_keybindings() -> Vec<KeyBinding> {
    let mut keybindings = Vec::new();
    for action in Actions::iter() {
        use Actions::*;
        use VirtualKeyCode::*;
        keybindings.push(match action {
            SaveShip => KeyBinding { action, key: F5 },
            LoadShip => KeyBinding { action, key: F9 },
            MoveUp => KeyBinding { action, key: W },
            MoveDown => KeyBinding { action, key: S },
            MoveLeft => KeyBinding { action, key: A },
            MoveRight => KeyBinding { action, key: D },
            RotateClockwise => KeyBinding { action, key: E },
            RotateCounterClockwise => KeyBinding { action, key: Q },
            ZoomIn => KeyBinding {
                action,
                key: LShift,
            },
            ZoomOut => KeyBinding {
                action,
                key: LControl,
            },
        })
    }
    keybindings
}
