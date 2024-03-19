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

use magnus::{function, Module};

use parking_lot::RwLock;
use std::sync::OnceLock;

use librgss::{Button, KeyBind};

// FIXME find a way around using a static
pub(crate) static INPUT: OnceLock<RwLock<librgss::Input>> = OnceLock::new();

#[magnus::wrap(class = "Input::Button", size, free_immediately, frozen_shareable)]
pub(crate) struct RButton(Button);

#[inline(always)]
pub fn get_input() -> &'static RwLock<librgss::Input> {
    INPUT
        .get()
        .expect("input static not set! please report how you encountered this crash")
}

fn update() -> Result<(), magnus::Error> {
    let mut input = get_input().write();
    input.update();

    if input.exited() {
        Err(magnus::Error::new(magnus::exception::system_exit(), " "))
    } else {
        Ok(())
    }
}

fn trigger(button: &RButton) -> bool {
    let input = get_input().read();
    input.triggered(button.0)
}

fn press(button: &RButton) -> bool {
    let input = get_input().read();
    input.pressed(button.0)
}

fn repeat(button: &RButton) -> bool {
    let input = get_input().read();
    input.repeat(button.0)
}

pub fn bind(ruby: &magnus::Ruby, input: librgss::Input) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Input")?;
    module.define_class("Button", ruby.class_basic_object())?;

    // panic if input is set! this should not *ever* happen
    if INPUT.set(RwLock::new(input)).is_err() {
        panic!("input static already set! this is not supposed to happen")
    }

    module.define_module_function("update", function!(update, 0))?;

    module.define_module_function("trigger?", function!(trigger, 1))?;
    module.define_module_function("press?", function!(press, 1))?;
    module.define_module_function("repeat?", function!(repeat, 1))?;

    module.const_set("KEY_M", 0)?;
    module.const_set("KEY_E", 0)?;
    module.const_set("KEY_O", 0)?;
    module.const_set("KEY_W", 0)?;

    module.const_set("ACTION", RButton(Button::KeyBind(KeyBind::Action)))?;
    module.const_set("CANCEL", RButton(Button::KeyBind(KeyBind::Cancel)))?;
    module.const_set("R", RButton(Button::KeyBind(KeyBind::R)))?;

    Ok(())
}
