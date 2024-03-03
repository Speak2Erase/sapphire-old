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

use parking_lot::Mutex;
use std::sync::OnceLock;

// FIXME find a way around using a static
pub(crate) static AUDIO: OnceLock<Mutex<librgss::Audio>> = OnceLock::new();

#[inline(always)]
pub fn get_audio() -> &'static Mutex<librgss::Audio> {
    AUDIO
        .get()
        .expect("audio static not set! please report how you encountered this crash")
}

pub fn bind(ruby: &magnus::Ruby, audio: librgss::Audio) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Audio")?;

    // panic if audio is set! this should not *ever* happen
    if AUDIO.set(Mutex::new(audio)).is_err() {
        panic!("audio static already set! this is not supposed to happen")
    }

    Ok(())
}
