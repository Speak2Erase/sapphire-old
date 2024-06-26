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

use slotmap::Key;

use super::{drawable::DrawableMut, DrawableRef, Graphics, GraphicsState, RenderState, ZList, Z};
use crate::{Arenas, Rect};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Viewport {
    pub(crate) key: ViewportKey,
}

#[derive(Debug)]
pub(crate) struct ViewportInternal {
    pub rect: Rect,
    pub z: Z,
    pub z_list: ZList<DrawableRef>,
}

slotmap::new_key_type! {
  pub(crate) struct ViewportKey;
}

impl Viewport {
    pub fn new(
        graphics: &Graphics,
        arenas: &mut Arenas,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> Self {
        let z = Z::new(0);
        let internal = ViewportInternal {
            rect: Rect::new(x, y, width, height),
            z,
            z_list: ZList::new(),
        };

        let key = arenas.viewport.insert(internal);

        let global_viewport = arenas
            .viewport
            .get_mut(graphics.global_viewport.key)
            .expect(Arenas::VIEWPORT_MISSING);
        global_viewport.z_list.insert(z, DrawableRef::Viewport(key));

        Self { key }
    }

    pub fn null() -> Self {
        Self {
            key: ViewportKey::null(),
        }
    }
}

impl ViewportInternal {
    pub(crate) fn global() -> Self {
        ViewportInternal {
            rect: Rect::new(0, 0, 640, 480),
            z: Z::new(0),
            z_list: ZList::new(),
        }
    }

    pub(crate) fn insert(&mut self, z: Z, item: DrawableRef) {
        self.z_list.insert(z, item)
    }

    pub(crate) fn remove(&mut self, z: Z) -> Option<DrawableRef> {
        self.z_list.remove(z)
    }

    pub(crate) fn update_z(&mut self, old_z: Z, new_z: Z) {
        if let Some(item) = self.remove(old_z) {
            self.insert(new_z, item)
        }
    }

    pub(crate) fn swap(&mut self, other: &mut Self, z: Z) {
        if let Some(item) = self.remove(z) {
            other.insert(z, item)
        }
    }

    pub(crate) fn draw<'rpass>(&'rpass self, render_state: &mut RenderState<'_, 'rpass>) {
        render_state.render_pass.set_viewport(
            self.rect.x as f32,
            self.rect.y as f32,
            self.rect.width as f32,
            self.rect.height as f32,
            0.0,
            1.0,
        );

        // FIXME do this, but mutably (or add some kind of prepare method)
        for (_, drawable) in self.z_list.iter() {
            let Some(drawable) = drawable.fetch(render_state.arenas) else {
                continue;
            };

            drawable.draw(self, render_state);
        }
    }
}
