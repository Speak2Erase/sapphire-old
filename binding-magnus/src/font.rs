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

use crate::data::Color;
use magnus::{function, value::ReprValue, Object, TryConvert};
use parking_lot::RwLock;
use std::sync::OnceLock;

// FIXME find a way around using a static
pub(crate) static FONTS: OnceLock<RwLock<librgss::Fonts>> = OnceLock::new();

#[inline(always)]
pub fn get_fonts() -> &'static RwLock<librgss::Fonts> {
    FONTS
        .get()
        .expect("fonts static not set! please report how you encountered this crash")
}

fn default_name() -> magnus::Value {
    // This function is only exposed to Ruby. It is not possible to call this without it being called on a Ruby thread
    let ruby = unsafe { magnus::Ruby::get_unchecked() };

    let fonts = get_fonts().read();
    match fonts.default.fonts.as_slice() {
        [name] => ruby.str_new(name).as_value(),
        names => ruby
            .ary_from_iter(names.iter().map(String::as_str))
            .as_value(),
    }
}

fn set_default_name(arg: magnus::Value) -> Result<(), magnus::Error> {
    let ruby = magnus::Ruby::get_with(arg);

    let mut fonts = get_fonts().write();
    if arg.is_kind_of(ruby.class_array()) {
        let names = Vec::<String>::try_convert(arg)?;
        fonts.default.fonts = names;

        Ok(())
    } else if arg.is_kind_of(ruby.class_string()) {
        let name = String::try_convert(arg)?;
        fonts.default.fonts = vec![name];

        Ok(())
    } else {
        // TODO proper error message
        let error = magnus::Error::new(ruby.exception_type_error(), "dsajfjlsdfb");
        Err(error)
    }
}

fn default_size() -> u32 {
    get_fonts().read().default.size
}

fn set_default_size(size: u32) {
    get_fonts().write().default.size = size
}

fn default_bold() -> bool {
    get_fonts().read().default.bold
}

fn set_default_bold(bold: bool) {
    get_fonts().write().default.bold = bold
}

fn default_italic() -> bool {
    get_fonts().read().default.italic
}

fn set_default_italic(italic: bool) {
    get_fonts().write().default.italic = italic
}

fn default_color() -> Color {
    let fonts = get_fonts().read();
    let color = fonts.default.color.clone();
    Color(color)
}

fn set_default_color(color: &Color) {
    let mut fonts = get_fonts().write();
    let color = color.0.clone();
    fonts.default.color = color;
}

#[cfg(feature = "rgss2")]
fn default_shadow() -> bool {
    get_fonts().read().default.shadow
}

#[cfg(feature = "rgss2")]
fn set_default_shadow(shadow: bool) {
    get_fonts().write().default.shadow = shadow
}

#[cfg(feature = "rgss3")]
fn default_outline() -> Color {
    let fonts = get_fonts().read();
    let color = fonts.default.outline.clone();
    Color(color)
}

#[cfg(feature = "rgss3")]
fn set_default_outline(color: &Color) {
    let mut fonts = get_fonts().write();
    let color = color.0.clone();
    fonts.default.outline = color;
}

#[cfg(feature = "rgss3")]
fn default_out_color() -> Color {
    let fonts = get_fonts().read();
    let color = fonts.default.out_color.clone();
    Color(color)
}

#[cfg(feature = "rgss3")]
fn set_default_out_color(color: &Color) {
    let mut fonts = get_fonts().write();
    let color = color.0.clone();
    fonts.default.out_color = color;
}

pub fn bind(ruby: &magnus::Ruby, fonts: librgss::Fonts) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Font", ruby.class_object())?;

    // panic if graphic is set! this should not *ever* happen
    if FONTS.set(RwLock::new(fonts)).is_err() {
        panic!("fonts static already set! this is not supposed to happen")
    }

    class.define_singleton_method("default_name", function!(default_name, 0))?;
    class.define_singleton_method("default_name=", function!(set_default_name, 1))?;

    class.define_singleton_method("default_size", function!(default_size, 0))?;
    class.define_singleton_method("default_size=", function!(set_default_size, 1))?;

    class.define_singleton_method("default_bold", function!(default_bold, 0))?;
    class.define_singleton_method("default_bold=", function!(set_default_bold, 1))?;

    class.define_singleton_method("default_italic", function!(default_italic, 0))?;
    class.define_singleton_method("default_italic=", function!(set_default_italic, 1))?;

    class.define_singleton_method("default_color", function!(default_color, 0))?;
    class.define_singleton_method("default_color=", function!(set_default_color, 1))?;

    #[cfg(feature = "rgss2")]
    {
        class.define_singleton_method("default_shadow", function!(default_shadow, 0))?;
        class.define_singleton_method("default_shadow=", function!(set_default_shadow, 1))?;
    }

    #[cfg(feature = "rgss3")]
    {
        class.define_singleton_method("default_outline", function!(default_outline, 0))?;
        class.define_singleton_method("default_outline=", function!(set_default_outline, 1))?;

        class.define_singleton_method("default_out_color", function!(default_out_color, 0))?;
        class.define_singleton_method("default_out_color=", function!(set_default_out_color, 1))?;
    }

    Ok(())
}
