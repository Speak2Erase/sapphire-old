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
// along with sapphire.  If not, see <https://www.gnu.org/licenses/>.

use glam::{vec2, Vec2};

#[derive(Debug, Clone, Copy, PartialEq, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec2,
    pub tex_coords: Vec2,
}

impl Vertex {
    const ATTRS: &'static [wgpu::VertexAttribute] =
        &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2];
    pub const fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as u64,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: Self::ATTRS,
        }
    }

    pub const fn new(position: Vec2, tex_coords: Vec2) -> Self {
        Self {
            position,
            tex_coords,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Quad {
    pub rect: Rect,
    pub tex_coords: Rect,
}

#[derive(Debug, Clone, Copy, PartialEq, Default, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Rect {
    pub position: Vec2,
    pub size: Vec2,
}

impl From<crate::Rect> for Rect {
    fn from(value: crate::Rect) -> Self {
        Rect {
            position: vec2(value.x as f32, value.y as f32),
            size: vec2(value.width as f32, value.height as f32),
        }
    }
}

impl Rect {
    pub fn from_min_max(min: Vec2, max: Vec2) -> Self {
        let size = max - min;
        Self {
            position: min,
            size,
        }
    }

    pub fn from_pos_size(position: Vec2, size: Vec2) -> Self {
        Self { position, size }
    }

    pub const fn min(&self) -> Vec2 {
        self.position
    }

    pub fn max(&self) -> Vec2 {
        self.position + self.size
    }

    pub const fn left(&self) -> f32 {
        self.position.x
    }

    pub fn right(&self) -> f32 {
        self.max().x
    }

    pub const fn top(&self) -> f32 {
        self.position.y
    }

    pub fn bottom(&self) -> f32 {
        self.max().y
    }

    pub const fn left_top(&self) -> Vec2 {
        vec2(self.left(), self.top())
    }

    pub fn right_top(&self) -> Vec2 {
        vec2(self.right(), self.top())
    }

    pub fn left_bottom(&self) -> Vec2 {
        vec2(self.left(), self.bottom())
    }

    pub fn right_bottom(&self) -> Vec2 {
        vec2(self.right(), self.bottom())
    }

    pub fn scale_by(&self, scale: Vec2) -> Self {
        Self {
            position: self.position * scale,
            size: self.size * scale,
        }
    }

    pub fn shrink_by(&self, scale: Vec2) -> Self {
        Self {
            position: self.position / scale,
            size: self.size / scale,
        }
    }
}

impl Quad {
    // Quads are made like this:
    // TL------TR
    // |  \ /  |
    // |  / \  |
    // BL-----BR
    pub const QUAD_INDICES: [u32; 6] = [
        // Triangle A
        // TL
        // |  \
        // |    \
        // BL-----BR
        0, 2, 3,
        // Triangle B
        // TL------TR
        //    \    |
        //      \  |
        //        BR
        0, 1, 2,
    ];

    pub fn norm_tex_coords(self, extents: wgpu::Extent3d) -> Self {
        let scale = vec2(extents.width as f32, extents.height as f32);
        let tex_coords = self.tex_coords.shrink_by(scale);

        Self { tex_coords, ..self }
    }

    pub fn into_individual_verts(self) -> [Vertex; 4] {
        let Quad { rect, tex_coords } = self;

        [
            Vertex::new(rect.left_top(), tex_coords.left_top()),
            Vertex::new(rect.right_top(), tex_coords.right_top()),
            Vertex::new(rect.right_bottom(), tex_coords.right_bottom()),
            Vertex::new(rect.left_bottom(), tex_coords.left_bottom()),
        ]
    }

    pub fn into_verts_indices(self) -> ([Vertex; 4], [u32; 6]) {
        (self.into_individual_verts(), Self::QUAD_INDICES)
    }

    pub fn into_verts(self) -> [Vertex; 6] {
        let [tl, tr, br, bl] = self.into_individual_verts();
        [tl, br, bl, tl, tr, br]
    }
}
