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
pub(crate) static AUDIO: OnceLock<RwLock<librgss::Audio>> = OnceLock::new();

#[inline(always)]
pub fn get_audio() -> &'static RwLock<librgss::Audio> {
    AUDIO
        .get()
        .expect("audio static not set! please report how you encountered this crash")
}

fn bgm_play(args: &[Value]) -> Result<(), magnus::Error> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

    let (path,): (String,) = args.required;
    let (volume, pitch) = args.optional;

    get_audio()
        .read()
        .bgm_play(path, volume.unwrap_or(100), pitch.unwrap_or(100));

    Ok(())
}

fn bgm_stop() {
    get_audio().read().bgm_stop()
}

fn bgm_fade(time: u32) {
    get_audio().read().bgm_fade(time)
}

fn bgs_play(args: &[Value]) -> Result<(), magnus::Error> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

    let (path,): (String,) = args.required;
    let (volume, pitch) = args.optional;

    get_audio()
        .read()
        .bgs_play(path, volume.unwrap_or(100), pitch.unwrap_or(100));

    Ok(())
}

fn bgs_stop() {
    get_audio().read().bgs_stop()
}

fn bgs_fade(time: u32) {
    get_audio().read().bgs_fade(time)
}

fn me_play(args: &[Value]) -> Result<(), magnus::Error> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

    let (path,): (String,) = args.required;
    let (volume, pitch) = args.optional;

    get_audio()
        .read()
        .me_play(path, volume.unwrap_or(100), pitch.unwrap_or(100));

    Ok(())
}

fn me_stop() {
    get_audio().read().me_stop()
}

fn me_fade(time: u32) {
    get_audio().read().me_fade(time)
}

fn se_play(args: &[Value]) -> Result<(), magnus::Error> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

    let (path,): (String,) = args.required;
    let (volume, pitch) = args.optional;

    get_audio()
        .read()
        .se_play(path, volume.unwrap_or(100), pitch.unwrap_or(100));

    Ok(())
}

fn se_stop() {
    get_audio().read().se_stop()
}

#[cfg(feature = "modshot")]
fn bgm_crossfade(args: &[Value]) {
    // TODO
}

#[cfg(feature = "modshot")]
fn bgs_crossfade(args: &[Value]) {
    // TODO
}

#[cfg(feature = "modshot")]
fn me_crossfade(args: &[Value]) {
    // TODO
}

#[cfg(feature = "modshot")]
fn ch_play(args: &[Value]) {
    // TODO
}

#[cfg(feature = "modshot")]
fn ch_crossfade(args: &[Value]) {
    // TODO
}

#[cfg(feature = "modshot")]
fn lch_play(args: &[Value]) {
    // TODO
}

#[cfg(feature = "modshot")]
fn lch_crossfade(args: &[Value]) {
    // TODO
}

pub fn bind(ruby: &magnus::Ruby, audio: librgss::Audio) -> Result<(), magnus::Error> {
    let module = ruby.define_module("Audio")?;

    // panic if audio is set! this should not *ever* happen
    if AUDIO.set(RwLock::new(audio)).is_err() {
        panic!("audio static already set! this is not supposed to happen")
    }

    module.define_module_function("bgm_play", function!(bgm_play, -1))?;
    module.define_module_function("bgm_stop", function!(bgm_stop, 0))?;
    module.define_module_function("bgm_fade", function!(bgm_fade, 1))?;

    module.define_module_function("bgs_play", function!(bgs_play, -1))?;
    module.define_module_function("bgs_stop", function!(bgs_stop, 0))?;
    module.define_module_function("bgs_fade", function!(bgs_fade, 1))?;

    module.define_module_function("me_play", function!(me_play, -1))?;
    module.define_module_function("me_stop", function!(me_stop, 0))?;
    module.define_module_function("me_fade", function!(me_fade, 1))?;

    module.define_module_function("se_play", function!(se_play, -1))?;
    module.define_module_function("se_stop", function!(se_stop, 0))?;

    #[cfg(feature = "modshot")]
    {
        module.define_module_function("bgm_crossfade", function!(bgm_crossfade, -1))?;
        module.define_module_function("bgs_crossfade", function!(bgs_crossfade, -1))?;
        module.define_module_function("me_crossfade", function!(me_crossfade, -1))?;

        module.define_module_function("ch_play", function!(ch_play, -1))?;
        module.define_module_function("ch_crossfade", function!(ch_crossfade, -1))?;
        module.define_module_function("lch_play", function!(lch_play, -1))?;
        module.define_module_function("lch_crossfade", function!(lch_crossfade, -1))?;
    }

    Ok(())
}
