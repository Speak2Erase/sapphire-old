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

use wgpu::util::DeviceExt;

use crate::Graphics;

pub struct Bitmap {
    // TODO investigate texture atlases
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
}

impl Bitmap {
    pub fn new(graphics: &Graphics, width: u32, height: u32) -> Self {
        // TODO handle bitmaps that are too large
        let texture = graphics
            .graphics_state
            .device
            .create_texture(&bitmap_texture_descriptor(width, height));
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, view }
    }

    pub fn new_path(graphics: &Graphics, path: impl AsRef<camino::Utf8Path>) -> Self {
        // TODO handle errors
        let mut image_file = graphics.filesystem.read_file(path).unwrap();

        let mut image_data = vec![];
        image_file.read_to_end(&mut image_data).unwrap();

        let image = image::load_from_memory(&image_data).unwrap().to_rgba8();

        let texture = graphics.graphics_state.device.create_texture_with_data(
            &graphics.graphics_state.queue,
            &bitmap_texture_descriptor(image.width() as u32, image.height() as u32),
            wgpu::util::TextureDataOrder::LayerMajor,
            &image,
        );
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        Self { texture, view }
    }
}

pub(crate) fn bitmap_texture_descriptor(
    width: u32,
    height: u32,
) -> wgpu::TextureDescriptor<'static> {
    wgpu::TextureDescriptor {
        label: Some("sapphire bitmap"),
        size: wgpu::Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        mip_level_count: 1,
        sample_count: 1,
        dimension: wgpu::TextureDimension::D2,
        format: wgpu::TextureFormat::Rgba8UnormSrgb,
        usage: wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::TEXTURE_BINDING,
        view_formats: &[],
    }
}
