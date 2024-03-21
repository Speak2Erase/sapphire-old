// Copyright (C) 2024 Lily Lyons
//
// This file is part of Sapphire.
//
// Sapphire is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Sapphire is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with Sapphire.  If not, see <http://www.gnu.org/licenses/>.

use color_eyre::eyre::OptionExt;
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::window::Window as NativeWindow;

use crate::{Arenas, EventLoop, FileSystem, Rect};

mod bitmap;
pub use bitmap::Bitmap;
pub(crate) use bitmap::{BitmapInternal, BitmapKey};

mod drawable;
use drawable::{Drawable, DrawableMut, DrawableRef};

mod sprite;
pub use sprite::Sprite;
pub(crate) use sprite::{SpriteInternal, SpriteKey};

mod plane;
pub use plane::Plane;
pub(crate) use plane::{PlaneInternal, PlaneKey};

mod tilemap;
pub use tilemap::Tilemap;
pub(crate) use tilemap::{TileKey, TilemapInternal};

mod viewport;
pub use viewport::Viewport;
pub(crate) use viewport::{ViewportInternal, ViewportKey};

mod window;
pub(crate) use window::WindowKey;
pub use window::{Window, WindowData};

mod z;
use z::{ZList, Z};

mod render;

pub struct Graphics {
    window: Arc<NativeWindow>,
    filesystem: Arc<FileSystem>,
    last_render: Instant,
    pub framerate: u16,
    pub frame_count: u64,

    pub(crate) bind_groups: render::BindGroups,
    pub(crate) pipelines: render::RenderPipelines,

    pub(crate) graphics_state: GraphicsState,
    pub(crate) global_viewport: Viewport,
    pub(crate) bitmap_ops: wgpu::CommandEncoder,
}

pub(crate) struct GraphicsState {
    pub(crate) instance: wgpu::Instance,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) surface_config: wgpu::SurfaceConfiguration,
}

pub(crate) struct RenderState<'a, 'rpass> {
    graphics_state: &'rpass GraphicsState,
    arenas: &'rpass Arenas,
    bind_groups: &'rpass render::BindGroups,
    pipelines: &'rpass render::RenderPipelines,
    render_pass: &'a mut wgpu::RenderPass<'rpass>,
}

const BITMAP_OPS_DESCRIPTOR: wgpu::CommandEncoderDescriptor<'static> =
    wgpu::CommandEncoderDescriptor {
        label: Some("bitmap operations (this frame)"),
    };

impl Graphics {
    pub async fn new(
        arenas: &mut Arenas,
        event_loop: &EventLoop,
        filesystem: Arc<FileSystem>,
    ) -> color_eyre::Result<Self> {
        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(640, 480))
            .with_resizable(false)
            .with_title("Sapphire")
            .build(&event_loop.event_loop)
            .map(Arc::new)?;
        let graphics_state = GraphicsState::new(window.clone()).await?;

        let bind_groups = render::BindGroups::new(&graphics_state);
        let pipelines = render::RenderPipelines::new(&graphics_state, &bind_groups);

        let global_viewport = ViewportInternal::global();
        let global_viewport = Viewport {
            key: arenas.viewport.insert(global_viewport),
        };

        let bitmap_ops = graphics_state
            .device
            .create_command_encoder(&BITMAP_OPS_DESCRIPTOR);

        let mut this = Self {
            window,
            filesystem,
            last_render: Instant::now(),
            framerate: 40,
            frame_count: 0,

            bind_groups,
            pipelines,

            graphics_state,
            global_viewport,
            bitmap_ops,
        };
        this.render(arenas);

        Ok(this)
    }

    fn render(&mut self, arenas: &Arenas) {
        let new_bitmap_ops = self
            .graphics_state
            .device
            .create_command_encoder(&BITMAP_OPS_DESCRIPTOR);
        let bitmap_ops = std::mem::replace(&mut self.bitmap_ops, new_bitmap_ops);
        let bitmap_ops = bitmap_ops.finish();

        // FIXME handle
        let Ok(surface_texture) = self.graphics_state.surface.get_current_texture() else {
            return;
        };
        let surface_view = surface_texture.texture.create_view(&Default::default());

        let mut encoder =
            self.graphics_state
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("sapphire main command encoder"),
                });

        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("sapphire main render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &surface_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        let mut render_state = RenderState {
            graphics_state: &self.graphics_state,
            arenas,
            bind_groups: &self.bind_groups,
            pipelines: &self.pipelines,
            render_pass: &mut render_pass,
        };

        let viewport = arenas
            .viewport
            .get(self.global_viewport.key)
            .expect(Arenas::VIEWPORT_MISSING);
        viewport.draw(&mut render_state);

        drop(render_pass);

        let command_buffer = encoder.finish();
        self.graphics_state
            .queue
            .submit([bitmap_ops, command_buffer]);

        surface_texture.present();
    }

    pub fn update(&mut self, arenas: &Arenas) {
        self.render(arenas);

        let frame_duration = Duration::from_secs_f32(1.0 / self.framerate as f32);
        let now = Instant::now();
        let time_since = now.duration_since(self.last_render);

        let wait_time = frame_duration.saturating_sub(time_since);
        println!("frame duration: {frame_duration:?} time since last frame: {time_since:?} wait time: {wait_time:?}");
        std::thread::sleep(wait_time);

        self.last_render = Instant::now();
    }

    #[cfg(feature = "modshot")]
    pub fn set_window_title(&self, title: &str) {
        self.window.set_title(title)
    }

    #[cfg(feature = "modshot")]
    pub fn set_window_icon(&self, path: &str) {
        let mut file = self.filesystem.read_file(path).unwrap(); // FIXME handle

        let mut buffer = vec![];
        file.read_to_end(&mut buffer).unwrap();

        let icon = image::load_from_memory(&buffer).unwrap().to_rgba8();
        let (width, height) = icon.dimensions();
        let icon = winit::window::Icon::from_rgba(icon.into_vec(), width, height).unwrap();
        self.window.set_window_icon(Some(icon));
    }
}

impl GraphicsState {
    async fn new(window: Arc<NativeWindow>) -> color_eyre::Result<Self> {
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            flags: wgpu::InstanceFlags::from_build_config(),
            ..Default::default()
        });

        let surface = instance.create_surface(window)?;

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptionsBase {
                power_preference: wgpu::util::power_preference_from_env()
                    .unwrap_or(wgpu::PowerPreference::HighPerformance),
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .ok_or_eyre("failed to find suitable adapter")?;

        let surface_config = surface
            .get_default_config(&adapter, 640, 480)
            .ok_or_eyre("surface not supported")?;

        // TODO optimizations based on certain features/limits
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("librgss device"),
                    required_features: wgpu::Features::PUSH_CONSTANTS,
                    required_limits: wgpu::Limits {
                        max_push_constant_size: 128,
                        ..Default::default()
                    },
                },
                None,
            )
            .await?;

        surface.configure(&device, &surface_config);

        Ok(Self {
            instance,
            surface,
            adapter,
            device,
            queue,
            surface_config,
        })
    }
}
