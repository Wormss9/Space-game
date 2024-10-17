use serde::Deserialize;
use strum::{EnumIter, IntoEnumIterator};
use winit::keyboard::{Key, NamedKey};

#[derive(Debug, Deserialize)]
pub struct KeyBinding {
    pub action: Actions,
    pub key: Key,
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
        use Key::*;
        use NamedKey::*;
        keybindings.push(match action {
            SaveShip => KeyBinding {
                action,
                key: Named(F5),
            },
            LoadShip => KeyBinding {
                action,
                key: Named(F9),
            },
            MoveUp => KeyBinding {
                action,
                key: Character("w".into()),
            },
            MoveDown => KeyBinding {
                action,
                key: Character("s".into()),
            },
            MoveLeft => KeyBinding {
                action,
                key: Character("a".into()),
            },
            MoveRight => KeyBinding {
                action,
                key: Character("d".into()),
            },
            RotateClockwise => KeyBinding {
                action,
                key: Character("e".into()),
            },
            RotateCounterClockwise => KeyBinding {
                action,
                key: Character("q".into()),
            },
            Actions::ZoomIn => KeyBinding {
                action,
                key: Named(Shift),
            },
            Actions::ZoomOut => KeyBinding {
                action,
                key: Named(Control),
            },
        })
    }
    keybindings
}
