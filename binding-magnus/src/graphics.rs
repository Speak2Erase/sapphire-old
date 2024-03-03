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

use parking_lot::RwLock;
use std::sync::OnceLock;

// FIXME find a way around using a static
pub(crate) static GRAPHICS: OnceLock<RwLock<librgss::Graphics>> = OnceLock::new();

#[inline(always)]
pub fn get_graphics() -> &'static RwLock<librgss::Graphics> {
    GRAPHICS
        .get()
        .expect("graphics static not set! please report how you encountered this crash")
}

pub fn bind(ruby: &magnus::Ruby, graphics: librgss::Graphics) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Graphics")?;

    // panic if graphic is set! this should not *ever* happen
    if GRAPHICS.set(RwLock::new(graphics)).is_err() {
        panic!("graphics static already set! this is not supposed to happen")
    }

    Ok(())
}
