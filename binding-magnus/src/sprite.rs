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

use magnus::{function, method, Module};

fn initialize(args: &[magnus::Value]) {}

#[deprecated = "FIXME: stub"]
fn null_getter(rb_self: magnus::Value) -> magnus::value::Qnil {
    magnus::value::qnil()
}

#[deprecated = "FIXME: stub"]
fn null_setter(rb_self: magnus::Value, _: magnus::Value) {}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Sprite", ruby.class_object())?;

    class.define_method("initialize", function!(initialize, -1))?;

    class.define_method("bitmap", method!(null_getter, 0))?;
    class.define_method("bitmap=", method!(null_setter, 1))?;

    Ok(())
}
