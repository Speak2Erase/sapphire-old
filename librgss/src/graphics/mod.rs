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
use slotmap::SlotMap;
use std::sync::Arc;
use winit::window::Window as NativeWindow;

use crate::{EventLoop, FileSystem, Rect};

mod bitmap;
pub use bitmap::Bitmap;

mod drawable;
use drawable::{Drawable, DrawableMut, DrawableRef};

mod sprite;
pub use sprite::Sprite;
use sprite::{SpriteInternal, SpriteKey};

mod plane;
pub use plane::Plane;
use plane::{PlaneInternal, PlaneKey};

mod tilemap;
pub use tilemap::Tilemap;
use tilemap::{TileKey, TilemapInternal};

mod viewport;
pub use viewport::Viewport;
use viewport::{ViewportInternal, ViewportKey};

mod window;
use window::WindowKey;
pub use window::{Window, WindowData};

mod z;
use z::{ZList, Z};

pub struct Graphics {
    window: Arc<NativeWindow>,
    filesystem: Arc<FileSystem>,
    pub(crate) graphics_state: GraphicsState,
    pub(crate) arenas: Arenas,
    pub(crate) global_viewport: Viewport,
}

#[derive(Default)]
pub(crate) struct Arenas {
    // FIXME use generational arenas instead to avoid aba
    pub sprite: SlotMap<SpriteKey, SpriteInternal>,
    pub plane: SlotMap<PlaneKey, PlaneInternal>,
    pub tilemap: SlotMap<TileKey, TilemapInternal>,
    pub viewport: SlotMap<ViewportKey, ViewportInternal>,
    pub window: SlotMap<WindowKey, WindowData>,
}

pub(crate) struct GraphicsState {
    pub(crate) instance: wgpu::Instance,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl Graphics {
    pub async fn new(
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

        let mut arenas = Arenas::default();

        let global_viewport = ViewportInternal::global();
        let global_viewport = Viewport {
            key: arenas.viewport.insert(global_viewport),
        };

        Ok(Self {
            window,
            filesystem,
            graphics_state,
            arenas,
            global_viewport,
        })
    }

    #[cfg(feature = "modshot")]
    pub fn set_window_title(&self, title: &str) {
        self.window.set_title(title)
    }
}

impl Arenas {
    const WINDOW_MISSING: &'static str =
        "window is missing from graphics arena! please report you you encountered this";
    const VIEWPORT_MISSING: &'static str =
        "viewport is missing from graphics arena! please report you you encountered this";
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
                    required_features: wgpu::Features::empty(),
                    required_limits: wgpu::Limits::default(),
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
        })
    }
}
