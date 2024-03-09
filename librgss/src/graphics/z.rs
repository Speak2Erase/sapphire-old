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

use std::collections::BTreeMap;
use std::time::Instant;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Z {
    pub z: i32,
    creation_time: Instant,
}

pub struct ZList<T> {
    // TODO benchmark replacing with Vec<T> and using a dirty flag
    tree_map: BTreeMap<Z, T>,
}

impl<T> Default for ZList<T> {
    fn default() -> Self {
        Self {
            tree_map: BTreeMap::new(),
        }
    }
}

impl PartialOrd for Z {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Z {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.z
            .cmp(&other.z)
            .then(self.creation_time.cmp(&other.creation_time))
    }
}

impl Z {
    pub fn new(z: i32) -> Self {
        Self {
            z,
            creation_time: Instant::now(),
        }
    }
}

impl<T> ZList<T> {
    pub fn new() -> Self {
        Self {
            tree_map: BTreeMap::new(),
        }
    }

    pub fn insert(&mut self, z: Z, value: T) {
        let old = self.tree_map.insert(z, value);
        debug_assert!(old.is_none())
    }

    pub fn get(&self, z: Z) -> Option<&T> {
        self.tree_map.get(&z)
    }

    pub fn get_mut(&mut self, z: Z) -> Option<&mut T> {
        self.tree_map.get_mut(&z)
    }

    pub fn remove(&mut self, z: Z) -> Option<T> {
        self.tree_map.remove(&z)
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Z, &T)> {
        self.tree_map.iter()
    }

    pub fn retain(&mut self, f: impl FnMut(&Z, &mut T) -> bool) {
        self.tree_map.retain(f)
    }
}
