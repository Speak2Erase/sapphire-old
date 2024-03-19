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

use crossbeam::atomic::AtomicCell;
use magnus::{method, typed_data::Obj, Class, Module};

use crate::{get_arenas, graphics::get_graphics};

#[magnus::wrap(class = "Viewport", free_immediately, size)]
pub struct Viewport(AtomicCell<librgss::Viewport>);

impl Default for Viewport {
    fn default() -> Self {
        Self(AtomicCell::new(librgss::Viewport::null()))
    }
}

impl From<&Viewport> for librgss::Viewport {
    fn from(val: &Viewport) -> Self {
        val.0.load()
    }
}

impl From<librgss::Viewport> for Viewport {
    fn from(val: librgss::Viewport) -> Self {
        Viewport(AtomicCell::new(val))
    }
}

impl Viewport {
    fn initialize(
        rb_self: Obj<Viewport>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Result<(), magnus::Error> {
        let graphics = get_graphics().read();
        let mut arenas = get_arenas().write();
        let viewport = librgss::Viewport::new(&graphics, &mut arenas, x, y, width, height);
        rb_self.0.store(viewport);

        Ok(())
    }
}

#[deprecated = "FIXME: stub"]
fn null_getter(rb_self: magnus::Value) -> magnus::value::Qnil {
    magnus::value::qnil()
}

#[deprecated = "FIXME: stub"]
fn null_setter(rb_self: magnus::Value, _: magnus::Value) {}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Viewport", ruby.class_object())?;

    class.define_alloc_func::<Viewport>();
    class.define_method("initialize", method!(Viewport::initialize, 4))?;

    class.define_method("z", method!(null_getter, 0))?;
    class.define_method("z=", method!(null_setter, 1))?;

    Ok(())
}
