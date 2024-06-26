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

use std::{
    ops::{Index, IndexMut},
    sync::Arc,
};

use crossbeam::atomic::AtomicCell;
use parking_lot::RwLock;

#[derive(Clone, Copy, PartialEq, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Color {
    pub red: f64,
    pub blue: f64,
    pub green: f64,
    pub alpha: f64,
}

pub type SharedColor = Arc<AtomicCell<Color>>;

impl Color {
    pub const WHITE: Self = Self {
        red: 255.0,
        blue: 255.0,
        green: 255.0,
        alpha: 255.0,
    };

    pub const BLACK: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
        alpha: 255.0,
    };

    pub const GREY: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
        alpha: 128.0,
    };

    pub const TRANSPARENT: Self = Self {
        red: 0.0,
        blue: 0.0,
        green: 0.0,
        alpha: 0.0,
    };
}

impl From<SharedColor> for Color {
    fn from(value: SharedColor) -> Self {
        value.load()
    }
}

impl From<Color> for SharedColor {
    fn from(value: Color) -> Self {
        Arc::new(AtomicCell::new(value))
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Tone {
    pub red: f64,
    pub blue: f64,
    pub green: f64,
    pub grey: f64,
}

pub type SharedTone = Arc<AtomicCell<Tone>>;

impl From<SharedTone> for Tone {
    fn from(value: SharedTone) -> Self {
        value.load()
    }
}

impl From<Tone> for SharedTone {
    fn from(value: Tone) -> Self {
        Arc::new(AtomicCell::new(value))
    }
}

#[derive(Clone, Copy, PartialEq, Debug, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

pub type SharedRect = Arc<AtomicCell<Rect>>;

impl Rect {
    pub fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

impl From<SharedRect> for Rect {
    fn from(value: SharedRect) -> Self {
        value.load()
    }
}

impl From<Rect> for SharedRect {
    fn from(value: Rect) -> Self {
        Arc::new(AtomicCell::new(value))
    }
}

#[derive(Default)]
pub struct Table {
    xsize: usize,
    ysize: usize,
    zsize: usize,
    data: Vec<i16>,
}

pub type SharedTable = Arc<RwLock<Table>>;

impl From<Table> for SharedTable {
    fn from(value: Table) -> Self {
        Arc::new(RwLock::new(value))
    }
}

impl Table {
    pub fn new(xsize: usize, ysize: usize, zsize: usize) -> Self {
        let data = vec![0; xsize * ysize * zsize];
        Self {
            xsize,
            ysize,
            zsize,
            data,
        }
    }

    pub fn new_data(xsize: usize, ysize: usize, zsize: usize, data: Vec<i16>) -> Self {
        assert_eq!(xsize * ysize * zsize, data.len());

        Self {
            xsize,
            ysize,
            zsize,
            data,
        }
    }

    pub fn xsize(&self) -> usize {
        self.xsize
    }

    pub fn ysize(&self) -> usize {
        self.ysize
    }

    pub fn zsize(&self) -> usize {
        self.zsize
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn data(&self) -> &[i16] {
        &self.data
    }

    pub fn data_mut(&mut self) -> &mut [i16] {
        &mut self.data
    }

    pub fn resize(&mut self, xsize: usize, ysize: usize, zsize: usize) {
        let mut new_data = vec![0; xsize * ysize];

        // A naive for loop like this is optimized to a handful of memcpys.
        for z in 0..self.zsize.min(zsize) {
            for y in 0..self.ysize.min(ysize) {
                for x in 0..self.xsize.min(xsize) {
                    new_data[(xsize * ysize * z) + (xsize * y) + x] = self[(x, y, z)]
                }
            }
        }

        self.xsize = xsize;
        self.ysize = ysize;
        self.zsize = zsize;

        self.data = new_data;
    }
}

impl Index<usize> for Table {
    type Output = i16;

    fn index(&self, index: usize) -> &Self::Output {
        debug_assert!(index < self.xsize);

        &self.data[index]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        debug_assert!(index < self.xsize);

        &mut self.data[index]
    }
}

impl Index<(usize, usize)> for Table {
    type Output = i16;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        debug_assert!(x < self.xsize);
        debug_assert!(y < self.ysize);

        &self[x + (y * self.xsize)]
    }
}

impl IndexMut<(usize, usize)> for Table {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        debug_assert!(x < self.xsize);
        debug_assert!(y < self.ysize);

        let xsize = self.xsize;
        &mut self.data[x + (y * xsize)]
    }
}

impl Index<(usize, usize, usize)> for Table {
    type Output = i16;

    fn index(&self, (x, y, z): (usize, usize, usize)) -> &Self::Output {
        debug_assert!(x < self.xsize);
        debug_assert!(y < self.ysize);
        debug_assert!(z < self.zsize);

        &self.data[x + (y * self.xsize + (z * self.xsize * self.ysize))]
    }
}

impl IndexMut<(usize, usize, usize)> for Table {
    fn index_mut(&mut self, (x, y, z): (usize, usize, usize)) -> &mut Self::Output {
        debug_assert!(x < self.xsize);
        debug_assert!(y < self.ysize);
        debug_assert!(z < self.zsize);

        &mut self.data[x + (y * self.xsize + (z * self.xsize * self.ysize))]
    }
}
