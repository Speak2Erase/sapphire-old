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

use magnus::{function, Value};
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

fn update() {
    let graphics = get_graphics().write();
}

fn fullscreen() -> bool {
    false
}

fn set_fullscreen(fullscreen: bool) {}

fn frame_rate() -> u32 {
    0
}

fn set_frame_rate(framerate: u32) {}

fn frame_count() -> u64 {
    0
}

fn transition(args: &[Value]) -> Result<(), magnus::Error> {
    let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;

    let (duration, filename, vague): (Option<u32>, Option<String>, Option<bool>) = args.optional;

    Ok(())
}

pub fn bind(ruby: &magnus::Ruby, graphics: librgss::Graphics) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Graphics")?;

    // panic if graphic is set! this should not *ever* happen
    if GRAPHICS.set(RwLock::new(graphics)).is_err() {
        panic!("graphics static already set! this is not supposed to happen")
    }

    module.define_module_function("update", function!(update, 0))?;

    module.define_module_function("transition", function!(transition, -1))?;

    module.define_module_function("fullscreen", function!(fullscreen, 0))?;
    module.define_module_function("fullscreen=", function!(set_fullscreen, 1))?;

    module.define_module_function("frame_rate", function!(frame_rate, 0))?;
    module.define_module_function("frame_rate=", function!(set_frame_rate, 1))?;

    module.define_module_function("frame_count", function!(frame_count, 0))?;

    Ok(())
}
