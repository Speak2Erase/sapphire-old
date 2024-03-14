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
use slotmap::SlotMap;

use crate::graphics::{
    BitmapInternal, BitmapKey, PlaneInternal, PlaneKey, SpriteInternal, SpriteKey, TileKey,
    TilemapInternal, ViewportInternal, ViewportKey, WindowData, WindowKey,
};

#[derive(Default)]
pub struct Arenas {
    pub(crate) sprite: SlotMap<SpriteKey, SpriteInternal>,
    pub(crate) plane: SlotMap<PlaneKey, PlaneInternal>,
    pub(crate) tilemap: SlotMap<TileKey, TilemapInternal>,
    pub(crate) bitmap: SlotMap<BitmapKey, BitmapInternal>,
    pub(crate) viewport: SlotMap<ViewportKey, ViewportInternal>,
    pub(crate) window: SlotMap<WindowKey, WindowData>,
}

impl Arenas {
    pub(crate) const WINDOW_MISSING: &'static str =
        "window is missing from graphics arena! please report you you encountered this";
    pub(crate) const VIEWPORT_MISSING: &'static str =
        "viewport is missing from graphics arena! please report you you encountered this";
}
