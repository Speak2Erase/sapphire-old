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

use crate::{get_arenas, graphics::get_graphics};

#[magnus::wrap(class = "Bitmap", free_immediately, size)]
pub struct Bitmap(pub AtomicCell<librgss::Bitmap>);

impl Default for Bitmap {
    fn default() -> Self {
        Self(AtomicCell::new(librgss::Bitmap::null()))
    }
}

impl Bitmap {
    fn initialize(rb_self: Obj<Bitmap>, args: &[Value]) -> Result<(), magnus::Error> {
        magnus::scan_args::check_arity(args.len(), 1..=2)?;

        let graphics = get_graphics().read();
        let mut arenas = get_arenas().write();
        let bitmap = match args {
            [path] => {
                let path = String::try_convert(*path)?;

                librgss::Bitmap::new_path(&graphics, &mut arenas, path)
            }
            [width, height] => {
                let width = u32::try_convert(*width)?;
                let height = u32::try_convert(*height)?;

                librgss::Bitmap::new(&graphics, &mut arenas, width, height)
            }
            _ => unreachable!(),
        };

        rb_self.0.store(bitmap);

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

    class.define_method("clear", method!(Bitmap::clear, 0))?;
    class.define_method("disposed?", method!(Bitmap::disposed, 0))?;

    Ok(())
}
