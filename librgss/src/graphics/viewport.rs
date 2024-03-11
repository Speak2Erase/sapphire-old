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

use super::{DrawableRef, ZList, Z};
use crate::Rect;

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
}
