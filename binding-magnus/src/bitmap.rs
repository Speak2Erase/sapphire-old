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
use magnus::{function, method, typed_data::Obj, Class, Module, Object, TryConvert, Value};

use crate::{
    font::{get_fonts, Font},
    get_arenas,
    graphics::get_graphics,
    helpers::Provider,
};

#[magnus::wrap(class = "Bitmap", free_immediately, size)]
pub struct Bitmap(pub AtomicCell<librgss::Bitmap>);

impl Default for Bitmap {
    fn default() -> Self {
        Self(AtomicCell::new(librgss::Bitmap::null()))
    }
}

#[derive(Clone, Copy)]
pub struct BitmapFontProvider(librgss::Bitmap);

impl Provider<librgss::Font> for BitmapFontProvider {
    fn provide<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&librgss::Font) -> R,
    {
        let arenas = get_arenas().read();
        let font = self.0.font(&arenas);
        f(font)
    }

    fn provide_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut librgss::Font) -> R,
    {
        let mut arenas = get_arenas().write();
        let font = self.0.font_mut(&mut arenas);
        f(font)
    }
}

impl Bitmap {
    fn initialize(rb_self: Obj<Bitmap>, args: &[Value]) -> Result<(), magnus::Error> {
        magnus::scan_args::check_arity(args.len(), 1..=2)?;

        let graphics = get_graphics().read();
        let fonts = get_fonts().read();
        let mut arenas = get_arenas().write();

        let bitmap = match args {
            [path] => {
                let path = String::try_convert(*path)?;

                librgss::Bitmap::new_path(&graphics, &fonts, &mut arenas, path)
            }
            [width, height] => {
                let width = u32::try_convert(*width)?;
                let height = u32::try_convert(*height)?;

                librgss::Bitmap::new(&graphics, &fonts, &mut arenas, width, height)
            }
            _ => unreachable!(),
        };

        rb_self.0.store(bitmap);

        let font = Font::from_provider(BitmapFontProvider(bitmap));
        rb_self.ivar_set("font", font)?;

        Ok(())
    }

    fn font(rb_self: Obj<Bitmap>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("font")
    }

    fn set_font(&self, font: &Font) -> Result<(), magnus::Error> {
        // RGSS props do not take references, the actually take values! (kind of)
        // See https://github.com/Ancurio/mkxp/commit/f8c26fc515cb4fb6b24b766889d4b0b0a3c12a26#diff-dbf082db65931f45df274de8694f3df0ecbb77952084bfb3565e0bb184489160
        let mut arenas = get_arenas().write();
        font.0.read().provide(|f| {
            *self.0.load().font_mut(&mut arenas) = f.clone();
        });
        Ok(())
    }

    fn clear(&self) {
        // TODO
    }

    fn disposed(&self) -> bool {
        false
    }
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Bitmap", ruby.class_object())?;

    class.define_alloc_func::<Bitmap>();
    class.define_method("initialize", method!(Bitmap::initialize, -1))?;

    class.define_method("font", method!(Bitmap::font, 0))?;
    class.define_method("font=", method!(Bitmap::set_font, 1))?;

    class.define_method("clear", method!(Bitmap::clear, 0))?;
    class.define_method("disposed?", method!(Bitmap::disposed, 0))?;

    Ok(())
}
