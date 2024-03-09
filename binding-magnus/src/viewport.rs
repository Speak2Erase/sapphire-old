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

use magnus::{method, Module};

#[magnus::wrap(class = "Viewport", free_immediately, size)]
#[derive(Clone, Copy)]
pub struct Viewport(librgss::Viewport);

impl From<Viewport> for librgss::Viewport {
    fn from(val: Viewport) -> Self {
        val.0
    }
}

impl From<librgss::Viewport> for Viewport {
    fn from(val: librgss::Viewport) -> Self {
        Viewport(val)
    }
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let class = ruby.define_class("Viewport", ruby.class_object())?;

    Ok(())
}
