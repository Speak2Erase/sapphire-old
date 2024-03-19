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

use super::{DrawableRef, Graphics, Viewport, Z};
use crate::{Arenas, Bitmap, Rect};

#[derive(Clone, Copy)]
pub struct Window {
    key: WindowKey,
}

pub struct WindowData {
    pub rect: Rect,
    pub cursor_rect: Rect,
    pub active: bool,
    pub windowskin: Option<Bitmap>,
    pub contents: Option<Bitmap>,
    pub contents_opacity: u8,
    viewport: Viewport,
    z: Z,
}

slotmap::new_key_type! {
  pub(crate) struct WindowKey;
}

impl Window {
    pub fn new(graphics: &Graphics, arenas: &mut Arenas, viewport: Option<Viewport>) -> Self {
        let viewport = viewport.unwrap_or(graphics.global_viewport);
        let z = Z::new(0);

        let internal = WindowData {
            rect: Rect::default(),
            cursor_rect: Rect::default(),
            active: false,
            windowskin: None,
            contents: None,
            contents_opacity: 255,
            viewport,
            z,
        };

        let viewport = arenas
            .viewport
            .get_mut(viewport.key)
            .expect(Arenas::VIEWPORT_MISSING);

        let key = arenas.window.insert(internal);
        let drawable = DrawableRef::Window(key);
        viewport.z_list.insert(z, drawable);

        Self { key }
    }

    pub fn null() -> Self {
        Self {
            key: WindowKey::null(),
        }
    }

    pub fn viewport(&self, graphics: &Graphics, arenas: &Arenas) -> Option<Viewport> {
        let internal = arenas.window.get(self.key).expect(Arenas::WINDOW_MISSING);

        if internal.viewport == graphics.global_viewport {
            None
        } else {
            Some(internal.viewport)
        }
    }

    pub fn set_viewport(
        &mut self,
        graphics: &Graphics,
        arenas: &mut Arenas,
        viewport: Option<Viewport>,
    ) {
        let internal = arenas
            .window
            .get_mut(self.key)
            .expect(Arenas::WINDOW_MISSING);
        let new_viewport = viewport.unwrap_or(graphics.global_viewport);

        // viewports are identical, no need to do any work
        if internal.viewport == new_viewport {
            return;
        }

        let [current_viewport, new_viewport] = arenas
            .viewport
            .get_disjoint_mut([internal.viewport.key, new_viewport.key])
            .expect(Arenas::VIEWPORT_MISSING);
        new_viewport.swap(current_viewport, internal.z);
    }

    pub fn z(&self, arenas: &Arenas) -> i32 {
        let internal = arenas.window.get(self.key).expect(Arenas::WINDOW_MISSING);
        internal.z.value()
    }

    pub fn set_z(&self, arenas: &mut Arenas, value: i32) {
        let internal = arenas
            .window
            .get_mut(self.key)
            .expect(Arenas::WINDOW_MISSING);

        if internal.z.value() == value {
            return;
        }

        let viewport = arenas
            .viewport
            .get_mut(internal.viewport.key)
            .expect(Arenas::VIEWPORT_MISSING);

        let new_z = internal.z.update_value(value);
        viewport.z_list.re_insert(internal.z, new_z);
        internal.z = new_z;
    }

    pub fn get_data<'g>(&self, arenas: &'g Arenas) -> Option<&'g WindowData> {
        arenas.window.get(self.key)
    }

    pub fn get_data_mut<'g>(&self, arenas: &'g mut Arenas) -> Option<&'g mut WindowData> {
        arenas.window.get_mut(self.key)
    }
}

impl WindowData {
    pub(crate) fn draw<'rpass>(
        &'rpass self,
        arenas: &'rpass Arenas,
        render_pass: &mut wgpu::RenderPass<'rpass>,
    ) {
    }
}
