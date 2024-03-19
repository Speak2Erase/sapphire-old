// Copyright (C) 2024 Lily Lyons
//
// This file is part of sapphire.
//
// sapphire is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// sapphire is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with sapphire.  If not, see <http://www.gnu.org/licenses/>.

use std::collections::{HashMap, HashSet};
use winit::keyboard::{KeyCode, PhysicalKey};

#[derive(Debug)]
pub struct Buttons {
    pub bindings: Bindings,

    pub(crate) current_states: States,
    pub(crate) last_states: States,
    pub(crate) repeats: States,
}

pub(crate) type States = HashSet<Button>;

#[derive(Debug)]
pub struct Bindings {
    pub map: enum_map::EnumMap<KeyBind, KeyCode>,
}

// Rebindable keys
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug, enum_map::Enum)]
pub enum KeyBind {
    Down = 2,
    Left = 4,
    Right = 6,
    Up = 8,

    Action = 11,
    Cancel = 12,
    Menu = 13,
    Items = 14,
    Run = 15,
    Deactivate = 16,

    L = 17,
    R = 18,

    Settings = 41,
    Pause = 42,
}

#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum NamedButton {
    F5 = 25,
    F6 = 26,
    F7 = 27,
    F8 = 28,
    F9 = 29,

    MouseLeft = 38,
    MouseMiddle = 39,
    MouseRight = 40,
}

// Set of all recognized "Buttons"
#[derive(Clone, Copy, Hash, PartialEq, Eq, Debug)]
pub enum Button {
    KeyBind(KeyBind),
    KeyCode(KeyCode),
    Named(NamedButton),
}

const DIR_FLAGS: [u16; 4] = [
    1 << KeyBind::Down as u16,
    1 << KeyBind::Left as u16,
    1 << KeyBind::Right as u16,
    1 << KeyBind::Up as u16,
];
const DEAD_DIR_FLAGS: [u16; 2] = [DIR_FLAGS[0] | DIR_FLAGS[3], DIR_FLAGS[1] | DIR_FLAGS[2]];
const OTHER_DIRS: [[KeyBind; 3]; 4] = [
    [KeyBind::Left, KeyBind::Right, KeyBind::Up],
    [KeyBind::Down, KeyBind::Up, KeyBind::Right],
    [KeyBind::Down, KeyBind::Up, KeyBind::Left],
    [KeyBind::Left, KeyBind::Right, KeyBind::Up],
];

impl Default for Bindings {
    // TODO load from persistent config
    fn default() -> Self {
        let map = enum_map::enum_map! {
            KeyBind::Down => KeyCode::ArrowDown,
            KeyBind::Left => KeyCode::ArrowUp,
            KeyBind::Right => KeyCode::ArrowRight,
            KeyBind::Up => KeyCode::ArrowUp,

            KeyBind::Action =>KeyCode::KeyZ,
            KeyBind::Cancel => KeyCode::KeyX,
            KeyBind::Menu =>KeyCode::KeyA,
            KeyBind::Items => KeyCode::KeyS,
            KeyBind::Run => KeyCode::KeyR,
            KeyBind::Deactivate => KeyCode::KeyC,

            KeyBind::L => KeyCode::KeyQ,
            KeyBind::R => KeyCode::KeyW,

            KeyBind::Settings => KeyCode::Tab,
            KeyBind::Pause => KeyCode::KeyP
        };
        Self { map }
    }
}

impl Default for Buttons {
    fn default() -> Self {
        let bindings = Bindings::default();

        let current_states = States::default();
        let last_states = States::default();
        let repeats = States::default();

        Self {
            bindings,
            current_states,
            last_states,
            repeats,
        }
    }
}

impl Buttons {
    pub fn start_frame(&mut self) {
        std::mem::swap(&mut self.current_states, &mut self.last_states);
        self.current_states.clone_from(&self.last_states);
        self.repeats.clear();
    }

    pub fn process_key(&mut self, event: winit::event::KeyEvent) {
        let PhysicalKey::Code(key) = event.physical_key else {
            eprintln!("unrecognized keycode!");
            return;
        };

        let button = self
            .bindings
            .map
            .iter()
            .find(|(_, &k)| k == key)
            .map(|(k, _)| Button::KeyBind(k))
            .unwrap_or_else(|| match key {
                KeyCode::F5 => Button::Named(NamedButton::F5),
                KeyCode::F6 => Button::Named(NamedButton::F6),
                KeyCode::F7 => Button::Named(NamedButton::F7),
                KeyCode::F8 => Button::Named(NamedButton::F8),
                KeyCode::F9 => Button::Named(NamedButton::F9),
                _ => Button::KeyCode(key),
            });

        if event.repeat {
            self.repeats.insert(button);
        }

        if event.state.is_pressed() {
            self.current_states.insert(button);
        } else if !event.repeat {
            self.current_states.remove(&button);
        }
    }

    pub fn process_mouse(&mut self, event: winit::event::MouseButton) {}

    pub fn triggered(&self, button: Button) -> bool {
        self.current_states.contains(&button) && !self.last_states.contains(&button)
    }

    pub fn pressed(&self, button: Button) -> bool {
        self.current_states.contains(&button)
    }

    pub fn repeat(&self, button: Button) -> bool {
        self.repeats.contains(&button)
    }
}
