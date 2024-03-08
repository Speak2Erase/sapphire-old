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

use parking_lot::RwLock;

use super::{drawable::DrawableWeak, ZList, Z};
use crate::{Graphics, Rect};

pub struct Viewport {
    // TODO see if we can replace inner with something else
    pub(crate) inner: RwLock<Inner>,
}

pub(crate) struct GlobalViewport {
    list: RwLock<DrawableWeak>,
}

pub(crate) struct Inner {
    pub(crate) rect: Rect,
    pub(crate) z: Z,
    pub(crate) z_list: ZList<DrawableWeak>,
}

impl Viewport {
    pub(crate) fn insert(&self, z: Z, item: DrawableWeak) {
        let mut inner = self.inner.write();

        inner.z_list.insert(z, item)
    }

    pub(crate) fn remove(&self, z: Z) -> Option<DrawableWeak> {
        let mut inner = self.inner.write();

        inner.z_list.remove(z)
    }

    pub(crate) fn update_z(&self, old_z: Z, new_z: Z) {
        if let Some(item) = self.remove(old_z) {
            self.insert(new_z, item)
        }
    }

    pub(crate) fn swap(&self, other: &Self, z: Z) {
        if let Some(item) = self.remove(z) {
            other.insert(z, item)
        }
    }

    pub(crate) fn draw(&self, graphics: &Graphics) {
        let mut inner = self.inner.write();
        inner.z_list.retain(|_, weak| {
            let Some(drawable) = weak.upgrade() else {
                return false;
            };

            true
        });
    }
}
