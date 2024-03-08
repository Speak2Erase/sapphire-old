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

use magnus::function;

use crate::graphics::get_graphics;

fn set_title(title: String) {
    let graphics = get_graphics().read();
    graphics.set_window_title(&title);
}

fn set_icon(icon: String) {}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let mod_window = ruby.define_module("ModWindow")?;

    mod_window.define_module_function("SetTitle", function!(set_title, 1))?;
    mod_window.define_module_function("SetIcon", function!(set_icon, 1))?;

    Ok(())
}
