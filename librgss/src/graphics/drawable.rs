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

use super::{
    Graphics, PlaneInternal, PlaneKey, SpriteInternal, SpriteKey, TileKey, TilemapInternal,
    ViewportInternal, ViewportKey, WindowInternal, WindowKey,
};

#[derive(Clone, Copy)]
pub enum DrawableRef {
    Plane(PlaneKey),
    Sprite(SpriteKey),
    Tilemap(TileKey),
    Viewport(ViewportKey),
    Window(WindowKey),
}

pub enum Drawable<'res> {
    Plane(&'res PlaneInternal),
    Sprite(&'res SpriteInternal),
    Tilemap(&'res TilemapInternal),
    Viewport(&'res ViewportInternal),
    Window(&'res WindowInternal),
}

pub enum DrawableMut<'res> {
    Plane(&'res mut PlaneInternal),
    Sprite(&'res mut SpriteInternal),
    Tilemap(&'res mut TilemapInternal),
    Viewport(&'res mut ViewportInternal),
    Window(&'res mut WindowInternal),
}

impl DrawableRef {
    pub fn fetch(self, graphics: &Graphics) -> Option<Drawable<'_>> {
        match self {
            DrawableRef::Plane(p) => graphics.arenas.plane.get(p).map(Drawable::Plane),
            DrawableRef::Sprite(s) => graphics.arenas.sprite.get(s).map(Drawable::Sprite),
            DrawableRef::Tilemap(t) => graphics.arenas.tilemap.get(t).map(Drawable::Tilemap),
            DrawableRef::Viewport(v) => graphics.arenas.viewport.get(v).map(Drawable::Viewport),
            DrawableRef::Window(w) => graphics.arenas.window.get(w).map(Drawable::Window),
        }
    }

    pub fn fetch_mut(self, graphics: &mut Graphics) -> Option<DrawableMut<'_>> {
        match self {
            DrawableRef::Plane(p) => graphics.arenas.plane.get_mut(p).map(DrawableMut::Plane),
            DrawableRef::Sprite(s) => graphics.arenas.sprite.get_mut(s).map(DrawableMut::Sprite),
            DrawableRef::Tilemap(t) => graphics.arenas.tilemap.get_mut(t).map(DrawableMut::Tilemap),
            DrawableRef::Viewport(v) => graphics
                .arenas
                .viewport
                .get_mut(v)
                .map(DrawableMut::Viewport),
            DrawableRef::Window(w) => graphics.arenas.window.get_mut(w).map(DrawableMut::Window),
        }
    }
}
