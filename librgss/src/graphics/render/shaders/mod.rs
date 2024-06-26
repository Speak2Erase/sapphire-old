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

use crate::graphics::GraphicsState;

mod simple;

pub struct RenderPipelines {
    pub simple: wgpu::RenderPipeline,
}

pub struct BindGroups {
    pub simple: wgpu::BindGroupLayout,
}

impl BindGroups {
    pub fn new(graphics_state: &GraphicsState) -> Self {
        let simple = simple::create_bind_group(graphics_state);
        Self { simple }
    }
}

impl RenderPipelines {
    pub fn new(graphics_state: &GraphicsState, bind_groups: &BindGroups) -> Self {
        let simple = simple::create_shader(graphics_state, bind_groups);
        Self { simple }
    }
}
