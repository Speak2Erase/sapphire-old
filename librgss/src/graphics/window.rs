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

use super::{Arenas, DrawableRef, Graphics, Viewport, Z};
use crate::Rect;

#[derive(Clone, Copy)]
pub struct Window {
    key: WindowKey,
    viewport: Option<Viewport>,
}

pub(crate) struct WindowInternal {
    rect: Rect,
}

slotmap::new_key_type! {
  pub(crate) struct WindowKey;
}

impl Window {
    fn get_internal<'g>(&self, graphics: &'g Graphics) -> &'g WindowInternal {
        graphics
            .arenas
            .window
            .get(self.key)
            .expect(Arenas::WINDOW_MISSING)
    }

    fn get_internal_mut<'g>(&self, graphics: &'g mut Graphics) -> &'g mut WindowInternal {
        graphics
            .arenas
            .window
            .get_mut(self.key)
            .expect(Arenas::WINDOW_MISSING)
    }
}

impl Window {
    pub fn new(graphics: &mut Graphics, viewport: Option<Viewport>) -> Self {
        let internal = WindowInternal {
            rect: Rect::default(),
        };

        let key = graphics.arenas.window.insert(internal);
        let z = Z::new(0);

        let drawable = DrawableRef::Window(key);

        if let Some(viewport) = viewport {
            let viewport = graphics
                .arenas
                .viewport
                .get_mut(viewport.key)
                .expect(Arenas::VIEWPORT_MISSING);
            viewport.z_list.insert(z, drawable);
        } else {
            graphics.global_viewport.z_list.insert(z, drawable)
        }

        Self { key, viewport }
    }

    pub fn rect<'g>(&self, graphics: &'g Graphics) -> &'g Rect {
        &self.get_internal(graphics).rect
    }

    pub fn rect_mut<'g>(&self, graphics: &'g mut Graphics) -> &'g mut Rect {
        &mut self.get_internal_mut(graphics).rect
    }
}
