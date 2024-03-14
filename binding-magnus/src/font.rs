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

use crate::{
    bitmap::BitmapFontProvider,
    data::Color,
    helpers::{Provider, ProviderVal},
};
use magnus::{
    class, function, method, typed_data::Obj, value::ReprValue, Class, Module, Object, TryConvert,
};
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

// FIXME this is pretty complicated. do we put this in an arena instead?
// putting this in an arena would convert font to a "disposable" and in rgss font is definitely not disposable.
// what do we do about that?
#[magnus::wrap(class = "Font", free_immediately, size)]
pub struct Font(pub RwLock<ProviderVal<librgss::Font, BitmapFontProvider>>);

impl Default for Font {
    fn default() -> Self {
        let fonts = get_fonts().read();
        let font = librgss::Font::default(&fonts);
        Self(RwLock::new(ProviderVal::val(font)))
    }
}

#[derive(Clone, Copy)]
pub struct DefaultFontColorProvider;

// TODO macro?
impl Provider<librgss::Color> for DefaultFontColorProvider {
    fn provide<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&librgss::Color) -> R,
    {
        let fonts = get_fonts().read();
        f(&fonts.default.color)
    }

    fn provide_mut<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut librgss::Color) -> R,
    {
        let mut fonts = get_fonts().write();
        f(&mut fonts.default.color)
    }
}

impl Font {
    // A lot of this behavior is copy-pasted from mkxp.
    // See https://github.com/Ancurio/mkxp/blob/ac8f4b15946e37f0563653e6bb288d9a5a380e5b/binding-mri/font-binding.cpp#L72-L110
    fn initialize(rb_self: Obj<Self>, args: &[magnus::Value]) -> Result<(), magnus::Error> {
        let fonts = get_fonts().read();
        let args = magnus::scan_args::scan_args::<(), _, (), (), (), ()>(args)?;

        let (names, size) = args.optional;

        // We store names in an ivar to ensure it is identical (object wise).
        let names_val = names
            .map(Ok)
            .unwrap_or_else(|| rb_self.class().ivar_get("default_name"))?;
        let names = Self::collect_names(names_val)?;
        rb_self.ivar_set("name", names_val)?;

        let size = size.unwrap_or(fonts.default.size);

        let font = librgss::Font::new(&fonts, names, size);
        rb_self.0.write().provide_mut(|f| *f = font);

        Ok(())
    }

    fn color(rb_self: Obj<Self>) -> Result<magnus::Value, magnus::Error> {
        rb_self.ivar_get("color")
    }

    pub fn from_provider(p: impl Into<BitmapFontProvider>) -> Self {
        let provider = ProviderVal::provider(p);
        Self(RwLock::new(provider))
    }
}

impl Font {
    fn initialize_class_vars(class: magnus::RClass) -> Result<(), magnus::Error> {
        let fonts = get_fonts().read();

        match fonts.default.names.as_slice() {
            [name] => class.ivar_set("default_name", name.as_str())?,
            names => {
                let ary = magnus::RArray::from_iter(names.iter().map(String::as_str));
                class.ivar_set("default_name", ary)?
            }
        }

        let color = Color::from_provider(DefaultFontColorProvider);
        class.ivar_set("default_color", color)?;

        Ok(())
    }

    fn collect_names(value: magnus::Value) -> Result<Vec<String>, magnus::Error> {
        let ruby = magnus::Ruby::get_with(value);

        if value.is_kind_of(ruby.class_array()) {
            let names = Vec::<String>::try_convert(value)?;
            Ok(names)
        } else if value.is_kind_of(ruby.class_string()) {
            let name = String::try_convert(value)?;
            Ok(vec![name])
        } else {
            // TODO proper error message
            let error = magnus::Error::new(ruby.exception_type_error(), "dsajfjlsdfb");
            Err(error)
        }
    }

    fn default_name(class: magnus::RClass) -> Result<magnus::Value, magnus::Error> {
        class.ivar_get("default_name")
    }

    fn set_default_name(class: magnus::RClass, arg: magnus::Value) -> Result<(), magnus::Error> {
        let names = Self::collect_names(arg)?;
        let mut fonts = get_fonts().write();
        fonts.default.names = names;

        class.ivar_set("name", arg)?;

        Ok(())
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

    fn default_color(class: magnus::RClass) -> Result<magnus::Value, magnus::Error> {
        class.ivar_get("default_color")
    }

    fn set_default_color(color: &Color) {
        let mut fonts = get_fonts().write();
        let color = color.as_color();
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
    fn default_outline(class: magnus::RClass) -> Result<magnus::Value, magnus::Error> {
        class.ivar_get("default_outline")
    }

    #[cfg(feature = "rgss3")]
    fn set_default_outline(color: &Color) {
        let mut fonts = get_fonts().write();
        let color = color.as_color();
        fonts.default.outline = color;
    }

    #[cfg(feature = "rgss3")]
    fn default_out_color(class: magnus::RClass) -> Result<magnus::Value, magnus::Error> {
        class.ivar_get("default_out_color")
    }

    #[cfg(feature = "rgss3")]
    fn set_default_out_color(color: &Color) {
        let mut fonts = get_fonts().write();
        let color = color.as_color();
        fonts.default.out_color = color;
    }
}

pub fn bind(ruby: &magnus::Ruby, fonts: librgss::Fonts) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Font", ruby.class_object())?;

    // panic if graphic is set! this should not *ever* happen
    if FONTS.set(RwLock::new(fonts)).is_err() {
        panic!("fonts static already set! this is not supposed to happen")
    }

    Font::initialize_class_vars(class)?;

    class.define_alloc_func::<Font>();
    class.define_method("initialize", method!(Font::initialize, -1))?;

    class.define_singleton_method("default_name", method!(Font::default_name, 0))?;
    class.define_singleton_method("default_name=", method!(Font::set_default_name, 1))?;

    class.define_singleton_method("default_size", function!(Font::default_size, 0))?;
    class.define_singleton_method("default_size=", function!(Font::set_default_size, 1))?;

    class.define_singleton_method("default_bold", function!(Font::default_bold, 0))?;
    class.define_singleton_method("default_bold=", function!(Font::set_default_bold, 1))?;

    class.define_singleton_method("default_italic", function!(Font::default_italic, 0))?;
    class.define_singleton_method("default_italic=", function!(Font::set_default_italic, 1))?;

    class.define_singleton_method("default_color", method!(Font::default_color, 0))?;
    class.define_singleton_method("default_color=", function!(Font::set_default_color, 1))?;

    #[cfg(feature = "rgss2")]
    {
        class.define_singleton_method("default_shadow", function!(Font::default_shadow, 0))?;
        class.define_singleton_method("default_shadow=", function!(Font::set_default_shadow, 1))?;
    }

    #[cfg(feature = "rgss3")]
    {
        class.define_singleton_method("default_outline", method!(Font::default_outline, 0))?;
        class
            .define_singleton_method("default_outline=", function!(Font::set_default_outline, 1))?;

        class.define_singleton_method("default_out_color", method!(Font::default_out_color, 0))?;
        class.define_singleton_method(
            "default_out_color=",
            function!(Font::set_default_out_color, 1),
        )?;
    }

    Ok(())
}
