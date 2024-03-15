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

use slotmap::Key;
use wgpu::util::DeviceExt;

use crate::{Arenas, Font, Fonts, Graphics, Rect};

#[derive(Clone, Copy)]
pub struct Bitmap {
    // TODO investigate texture atlases
    pub(crate) key: BitmapKey,
}

pub(crate) struct BitmapInternal {
    pub(crate) texture: wgpu::Texture,
    pub(crate) view: wgpu::TextureView,
    pub(crate) font: Font,
}

slotmap::new_key_type! {
    pub(crate) struct BitmapKey;
}

impl Bitmap {
    pub fn new(
        graphics: &Graphics,
        fonts: &Fonts,
        arenas: &mut Arenas,
        width: u32,
        height: u32,
    ) -> Self {
        // TODO handle bitmaps that are too large
        let texture = graphics
            .graphics_state
            .device
            .create_texture(&bitmap_texture_descriptor(width, height));
        let view = texture.create_view(&wgpu::TextureViewDescriptor::default());

        let internal = BitmapInternal {
            texture,
            view,
            font: Font::default(fonts),
        };
        let key = arenas.bitmap.insert(internal);

        Self { key }
    }

    pub fn new_path(
        graphics: &Graphics,
        fonts: &Fonts,
        arenas: &mut Arenas,
        path: impl AsRef<camino::Utf8Path>,
    ) -> Self {
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

        let internal = BitmapInternal {
            texture,
            view,
            font: Font::default(fonts),
        };
        let key = arenas.bitmap.insert(internal);

        Self { key }
    }

    pub fn width(&self, arenas: &Arenas) -> u32 {
        let internal = arenas.bitmap.get(self.key).unwrap();
        internal.texture.width()
    }

    pub fn height(&self, arenas: &Arenas) -> u32 {
        let internal = arenas.bitmap.get(self.key).unwrap();
        internal.texture.height()
    }

    pub fn text_size(&self, arenas: &Arenas, fonts: &mut Fonts, text: &str) -> Rect {
        let BitmapInternal { font, .. } = arenas.bitmap.get(self.key).unwrap();
        let Fonts { font_system, .. } = fonts;
        println!("{text}");

        // FIXME line height is probably wrong
        let metrics = glyphon::Metrics::new(font.size as f32, font.size as f32);
        let mut buffer = glyphon::Buffer::new(font_system, metrics);
        buffer.set_size(font_system, f32::INFINITY, f32::INFINITY);
        // FIXME font name and attrs
        let attrs = glyphon::Attrs::new().family(glyphon::Family::SansSerif);
        buffer.set_text(font_system, text, attrs, glyphon::Shaping::Advanced);
        buffer.shape_until_scroll(font_system);

        let mut width = 0_f32;
        let mut height = 0_f32;
        for run in buffer.layout_runs() {
            width = width.max(run.line_w);
            height = height.max(run.line_y);
            println!("hsdfvmhfsdmghfdhgcsfhgncsdfh {} {}", run.line_w, run.line_y)
        }

        Rect::new(0, 0, width as u32, height as u32)
    }

    pub fn null() -> Self {
        Self {
            key: BitmapKey::null(),
        }
    }

    pub fn font<'a>(&self, arenas: &'a Arenas) -> &'a Font {
        // FIXME
        let internal = arenas.bitmap.get(self.key).unwrap();
        &internal.font
    }

    pub fn font_mut<'a>(&self, arenas: &'a mut Arenas) -> &'a mut Font {
        // FIXME
        let internal = arenas.bitmap.get_mut(self.key).unwrap();
        &mut internal.font
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
