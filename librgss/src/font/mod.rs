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

use crate::Color;

pub struct Fonts {
    font_system: glyphon::FontSystem,
    cache: glyphon::SwashCache,

    // FIXME this might be shared across fonts?
    pub default: Font,
}

#[derive(Clone, Debug)]
pub struct Font {
    pub names: Vec<String>,
    pub size: u32,
    pub bold: bool,
    pub italic: bool,
    pub color: Color,

    #[cfg(feature = "rgss2")]
    pub shadow: bool,
    #[cfg(feature = "rgss3")]
    pub outline: Color,
    #[cfg(feature = "rgss3")]
    pub out_color: Color,
}

impl Fonts {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut font_system = glyphon::FontSystem::new();
        font_system.db_mut().load_fonts_dir("Fonts");

        let cache = glyphon::SwashCache::new();

        let default = Font {
            names: vec!["Arial".to_string()],
            size: 22,
            bold: false,
            italic: false,
            color: Color::WHITE,
            #[cfg(feature = "rgss2")] // FIXME not 100% accurate
            shadow: false,
            #[cfg(feature = "rgss3")]
            outline: Color::WHITE,
            #[cfg(feature = "rgss3")]
            out_color: Color::GREY,
        };

        Self {
            font_system,
            cache,
            default,
        }
    }
}

impl Font {
    pub fn new(fonts: &Fonts, names: Vec<String>, size: u32) -> Self {
        Self {
            names,
            size,
            ..fonts.default
        }
    }

    pub fn default(fonts: &Fonts) -> Self {
        fonts.default.clone()
    }
}
