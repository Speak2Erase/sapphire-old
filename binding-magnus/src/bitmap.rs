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

use magnus::{function, method, Module, Object, TryConvert, Value};
use std::sync::Arc;

use crate::graphics::get_graphics;

#[magnus::wrap(class = "Bitmap", free_immediately, size)]
pub struct Bitmap(Arc<librgss::Bitmap>);

fn new(args: &[Value]) -> Result<Bitmap, magnus::Error> {
    magnus::scan_args::check_arity(args.len(), 1..=2)?;

    let graphics = get_graphics().read();
    Ok(match args {
        [path] => {
            let path = String::try_convert(*path)?;

            let bitmap = librgss::Bitmap::new_path(&graphics, path);
            Bitmap(bitmap)
        }
        [width, height] => {
            let width = u32::try_convert(*width)?;
            let height = u32::try_convert(*height)?;

            let bitmap = librgss::Bitmap::new(&graphics, width, height);
            Bitmap(bitmap)
        }
        _ => unreachable!(),
    })
}

fn disposed(bitmap: &Bitmap) -> bool {
    false
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Bitmap", ruby.class_object())?;

    class.define_singleton_method("new", function!(new, -1))?;

    class.define_method("disposed?", method!(disposed, 0))?;

    Ok(())
}
