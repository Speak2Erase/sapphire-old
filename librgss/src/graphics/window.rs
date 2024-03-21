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

use glam::vec2;
use slotmap::Key;
use wgpu::util::DeviceExt;

use super::{
    render::{BindGroupBuilder, Quad, Rect as RRect},
    DrawableRef, Graphics, GraphicsState, RenderState, Viewport, ViewportInternal, Z,
};
use crate::{Arenas, Bitmap, Rect};

#[derive(Clone, Copy)]
pub struct Window {
    key: WindowKey,
}

pub struct WindowData {
    pub rect: Rect,
    pub cursor_rect: Rect,
    pub active: bool,
    pub contents_opacity: u8,
    viewport: Viewport,
    z: Z,

    vertex_buffer: Option<wgpu::Buffer>,
    windowskin: Option<Contents>,
    contents: Option<Contents>,
}

struct Contents {
    bitmap: Bitmap,
    bind_group: wgpu::BindGroup,
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
            vertex_buffer: None,
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

    pub fn set_windowskin(&self, graphics: &Graphics, arenas: &mut Arenas, bitmap: Option<Bitmap>) {
        let internal = arenas
            .window
            .get_mut(self.key)
            .expect(Arenas::WINDOW_MISSING);

        let (windowskin, vertex_buffer) = bitmap
            .map(|bitmap| {
                let bitmap_internal = arenas.bitmap.get(bitmap.key).unwrap();

                let sampler =
                    graphics
                        .graphics_state
                        .device
                        .create_sampler(&wgpu::SamplerDescriptor {
                            label: Some("windowskin sampler"),
                            ..Default::default()
                        });

                let bind_group = BindGroupBuilder::new()
                    .append(wgpu::BindingResource::TextureView(&bitmap_internal.view))
                    .append(wgpu::BindingResource::Sampler(&sampler))
                    .build(
                        &graphics.graphics_state.device,
                        Some("windowskin bindgroup"),
                        &graphics.bind_groups.simple,
                    );

                let quad = Quad {
                    rect: RRect::from_pos_size(vec2(0.0, 0.0), vec2(192.0, 128.0)),
                    tex_coords: RRect::from_pos_size(vec2(0.0, 0.0), vec2(192.0, 128.0)),
                };
                let quad = quad.norm_tex_coords(bitmap_internal.texture.size());

                let vertices = quad.into_verts();
                let vertex_buffer = graphics.graphics_state.device.create_buffer_init(
                    &wgpu::util::BufferInitDescriptor {
                        label: Some("window vertex buffer"),
                        contents: bytemuck::cast_slice(&vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    },
                );

                (Contents { bitmap, bind_group }, vertex_buffer)
            })
            .unzip();

        internal.windowskin = windowskin;
        internal.vertex_buffer = vertex_buffer;
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
        viewport: &ViewportInternal,
        render_state: &mut RenderState<'_, 'rpass>,
    ) {
        let RenderState {
            pipelines,
            render_pass,
            ..
        } = render_state;

        let Some(vertex_buffer) = &self.vertex_buffer else {
            return;
        };
        let Some(windowskin) = &self.windowskin else {
            return;
        };

        render_pass.set_pipeline(&pipelines.simple);
        render_pass.set_bind_group(0, &windowskin.bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));

        let matrix = glam::Mat4::orthographic_lh(
            viewport.rect.x as f32,
            viewport.rect.x as f32 + viewport.rect.width as f32,
            viewport.rect.y as f32 + viewport.rect.height as f32,
            viewport.rect.y as f32,
            0.0,
            1.0,
        );
        render_pass.set_push_constants(wgpu::ShaderStages::VERTEX, 0, bytemuck::bytes_of(&matrix));

        render_pass.draw(0..6, 0..1);
    }
}
