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

use crate::graphics::{
    render::{binding_helpers::BindGroupLayoutBuilder, primitives::Vertex},
    GraphicsState,
};

use super::BindGroups;

pub fn create_shader(
    graphics_state: &GraphicsState,
    bind_groups: &BindGroups,
) -> wgpu::RenderPipeline {
    let layout = graphics_state
        .device
        .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("simple render pipeline layout"),
            bind_group_layouts: &[&bind_groups.simple],
            push_constant_ranges: &[wgpu::PushConstantRange {
                stages: wgpu::ShaderStages::VERTEX,
                range: 0..64,
            }],
        });

    let shader = graphics_state
        .device
        .create_shader_module(wgpu::include_wgsl!("simple.wgsl"));

    graphics_state
        .device
        .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("object render pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: graphics_state.surface_config.format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: u64::MAX,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        })
}

pub fn create_bind_group(graphics_state: &GraphicsState) -> wgpu::BindGroupLayout {
    BindGroupLayoutBuilder::new()
        .append(
            wgpu::ShaderStages::FRAGMENT,
            wgpu::BindingType::Texture {
                sample_type: wgpu::TextureSampleType::Float { filterable: true },
                view_dimension: wgpu::TextureViewDimension::D2,
                multisampled: false,
            },
            None,
        )
        .append(
            wgpu::ShaderStages::FRAGMENT,
            wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
            None,
        )
        .build(&graphics_state.device, Some("simple alpha bgl"))
}
