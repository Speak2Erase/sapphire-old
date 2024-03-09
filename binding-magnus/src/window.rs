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

use magnus::{function, method, typed_data::Obj, Class, Module, Object};

use crate::{bitmap::Bitmap, graphics::get_graphics, viewport::Viewport};

#[magnus::wrap(class = "Window", free_immediately, size)]
#[derive(Clone, Copy)]
pub struct Window(librgss::Window);

impl From<Window> for librgss::Window {
    fn from(val: Window) -> Self {
        val.0
    }
}

impl From<librgss::Window> for Window {
    fn from(val: librgss::Window) -> Self {
        Window(val)
    }
}

fn window_new(class: magnus::RClass, args: &[magnus::Value]) -> Result<Obj<Window>, magnus::Error> {
    let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;
    let (viewport,): (Option<&Viewport>,) = args.optional;

    let mut graphics = get_graphics().write();
    let window = librgss::Window::new(&mut graphics, viewport.copied().map(Into::into));

    let wrapped = Obj::wrap_as(Window(window), class);
    Ok(wrapped)
}

fn get_x(window: &Window) -> i32 {
    let graphics = get_graphics().read();
    let rect = window.0.rect(&graphics);
    rect.x
}

fn set_x(window: &Window, x: i32) {
    let mut graphics = get_graphics().write();
    window.0.rect_mut(&mut graphics).x = x;
}

fn windowskin(rb_self: magnus::Value) -> Bitmap {
    todo!()
}

fn set_windowskin(rb_self: magnus::Value, bitmap: &Bitmap) {}

fn update(window: &Window) {}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Window", ruby.class_object())?;

    class.define_singleton_method("new", method!(window_new, -1))?;
    class.define_singleton_method(
        "inherited",
        function!(magnus::RClass::undef_default_alloc_func, 1),
    )?;

    class.define_method("x", method!(get_x, 0))?;
    class.define_method("x=", method!(set_x, 1))?;

    class.define_method("windowskin", method!(windowskin, 0))?;
    class.define_method("windowskin=", method!(set_windowskin, 1))?;

    class.define_method("update", method!(update, 0))?;

    Ok(())
}
