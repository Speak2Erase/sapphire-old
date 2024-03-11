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
use magnus::{function, method, typed_data::Obj, Class, Module, Object};

use crate::{bitmap::Bitmap, get_arenas, graphics::get_graphics, viewport::Viewport};

#[magnus::wrap(class = "Window", free_immediately, size, frozen_shareable)]
pub struct Window(AtomicCell<librgss::Window>);

impl Default for Window {
    fn default() -> Self {
        Self(AtomicCell::new(librgss::Window::null()))
    }
}

impl Window {
    fn initialize(rb_self: Obj<Window>, args: &[magnus::Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;
        let (viewport,): (Option<&Viewport>,) = args.optional;

        let graphics = get_graphics().read();
        let mut arenas = get_arenas().write();
        let window =
            librgss::Window::new(&graphics, &mut arenas, viewport.copied().map(Into::into));
        rb_self.0.store(window);

        Ok(())
    }

    fn x(rb_self: Obj<Window>) -> Result<i32, magnus::Error> {
        let arenas = get_arenas().read();
        let data = rb_self.get_data(&arenas)?;
        Ok(data.rect.x)
    }

    fn set_x(rb_self: Obj<Window>, x: i32) -> Result<(), magnus::Error> {
        let mut arenas = get_arenas().write();
        let data = rb_self.get_data_mut(&mut arenas)?;
        data.rect.x = x;
        Ok(())
    }

    fn y(rb_self: Obj<Window>) -> Result<i32, magnus::Error> {
        let arenas = get_arenas().read();
        let data = rb_self.get_data(&arenas)?;
        Ok(data.rect.y)
    }

    fn set_y(rb_self: Obj<Window>, y: i32) -> Result<(), magnus::Error> {
        let mut arenas = get_arenas().write();
        let data = rb_self.get_data_mut(&mut arenas)?;
        data.rect.y = y;
        Ok(())
    }

    fn width(rb_self: Obj<Window>) -> Result<u32, magnus::Error> {
        let arenas = get_arenas().read();
        let data = rb_self.get_data(&arenas)?;
        Ok(data.rect.width)
    }

    fn set_width(rb_self: Obj<Window>, width: u32) -> Result<(), magnus::Error> {
        let mut arenas = get_arenas().write();
        let data = rb_self.get_data_mut(&mut arenas)?;
        data.rect.width = width;
        Ok(())
    }

    fn height(rb_self: Obj<Window>) -> Result<u32, magnus::Error> {
        let arenas = get_arenas().read();
        let data = rb_self.get_data(&arenas)?;
        Ok(data.rect.height)
    }

    fn set_height(rb_self: Obj<Window>, height: u32) -> Result<(), magnus::Error> {
        let mut arenas = get_arenas().write();
        let data = rb_self.get_data_mut(&mut arenas)?;
        data.rect.height = height;
        Ok(())
    }

    fn z(rb_self: Obj<Window>) -> Result<i32, magnus::Error> {
        let arenas = get_arenas().read();
        let window = rb_self.0.load();
        Ok(window.z(&arenas))
    }

    fn set_z(rb_self: Obj<Window>, z: i32) -> Result<(), magnus::Error> {
        let mut arenas = get_arenas().write();
        let window = rb_self.0.load();
        window.set_z(&mut arenas, z);
        Ok(())
    }
}

impl Window {
    fn get_data<'g>(
        &self,
        arenas: &'g librgss::Arenas,
    ) -> Result<&'g librgss::graphics::WindowData, magnus::Error> {
        let window = self.0.load();
        window.get_data(arenas).ok_or_else(|| {
            magnus::Error::new(
                magnus::exception::runtime_error(),
                "invalid window (missing call to super?)",
            )
        })
    }

    fn get_data_mut<'g>(
        &self,
        arenas: &'g mut librgss::Arenas,
    ) -> Result<&'g mut librgss::graphics::WindowData, magnus::Error> {
        let window = self.0.load();
        window.get_data_mut(arenas).ok_or_else(|| {
            magnus::Error::new(
                magnus::exception::runtime_error(),
                "invalid window (missing call to super?)",
            )
        })
    }
}

fn null_getter(rb_self: magnus::Value) -> magnus::value::Qnil {
    magnus::value::qnil()
}

fn null_setter(rb_self: magnus::Value, _: magnus::Value) {}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Window", ruby.class_object())?;

    class.define_alloc_func::<Window>();
    class.define_method("initialize", method!(Window::initialize, -1))?;

    class.define_method("x", method!(Window::x, 0))?;
    class.define_method("x=", method!(Window::set_x, 1))?;

    class.define_method("y", method!(Window::y, 0))?;
    class.define_method("y=", method!(Window::set_y, 1))?;

    class.define_method("width", method!(Window::width, 0))?;
    class.define_method("width=", method!(Window::set_width, 1))?;

    class.define_method("height", method!(Window::height, 0))?;
    class.define_method("height=", method!(Window::set_height, 1))?;

    class.define_method("z", method!(Window::z, 0))?;
    class.define_method("z=", method!(Window::set_z, 1))?;

    class.define_method("windowskin", method!(null_getter, 0))?;
    class.define_method("windowskin=", method!(null_setter, 1))?;

    class.define_method("contents", method!(null_getter, 0))?;
    class.define_method("contents=", method!(null_setter, 1))?;

    class.define_method("visible", method!(null_getter, 0))?;
    class.define_method("visible=", method!(null_setter, 1))?;

    class.define_method("active", method!(null_getter, 0))?;
    class.define_method("active=", method!(null_setter, 1))?;

    class.define_method("back_opacity", method!(null_getter, 0))?;
    class.define_method("back_opacity=", method!(null_setter, 1))?;

    Ok(())
}
