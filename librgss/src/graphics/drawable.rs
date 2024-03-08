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

use std::sync::{Arc, Weak};

use crate::{Plane, Sprite, Tilemap, Viewport};

use super::Window;

// FIXME arc is pain. we should try and use an arena, if possible
#[derive(Clone)]
pub enum Drawable {
    Plane(Arc<Plane>),
    Sprite(Arc<Sprite>),
    Tilemap(Arc<Tilemap>),
    Viewport(Arc<Viewport>),
    Window(Arc<Window>),
}

#[derive(Clone)]
pub enum DrawableWeak {
    Plane(Weak<Plane>),
    Sprite(Weak<Sprite>),
    Tilemap(Weak<Tilemap>),
    Viewport(Weak<Viewport>),
    Window(Weak<Window>),
}

impl From<&Drawable> for DrawableWeak {
    fn from(value: &Drawable) -> Self {
        match value {
            Drawable::Plane(p) => DrawableWeak::Plane(Arc::downgrade(p)),
            Drawable::Sprite(s) => DrawableWeak::Sprite(Arc::downgrade(s)),
            Drawable::Tilemap(t) => DrawableWeak::Tilemap(Arc::downgrade(t)),
            Drawable::Viewport(v) => DrawableWeak::Viewport(Arc::downgrade(v)),
            Drawable::Window(w) => DrawableWeak::Window(Arc::downgrade(w)),
        }
    }
}

impl DrawableWeak {
    pub fn upgrade(&self) -> Option<Drawable> {
        match self {
            DrawableWeak::Plane(p) => p.upgrade().map(Drawable::Plane),
            DrawableWeak::Sprite(s) => s.upgrade().map(Drawable::Sprite),
            DrawableWeak::Tilemap(t) => t.upgrade().map(Drawable::Tilemap),
            DrawableWeak::Viewport(v) => v.upgrade().map(Drawable::Viewport),
            DrawableWeak::Window(w) => w.upgrade().map(Drawable::Window),
        }
    }
}
