// Copyright (C) 2024 Lily Lyons
//
// This file is part of rsgss.
//
// rsgss is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// rsgss is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with rsgss.  If not, see <http://www.gnu.org/licenses/>.

use std::sync::Arc;

use winit::window::Window;

use color_eyre::eyre::OptionExt;

use crate::event_loop::EventLoop;

pub struct Graphics {
    window: Arc<Window>,
    pub(crate) graphics_state: GraphicsState,
}

pub(crate) struct GraphicsState {
    pub(crate) instance: wgpu::Instance,
    pub(crate) surface: wgpu::Surface<'static>,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
}

impl Graphics {
    pub async fn new(event_loop: &EventLoop) -> color_eyre::Result<Self> {
        let window = winit::window::WindowBuilder::new()
            .with_inner_size(winit::dpi::PhysicalSize::new(640, 480))
            .with_resizable(false)
            .build(&event_loop.event_loop)
            .map(Arc::new)?;
        let graphics_state = GraphicsState::new(window.clone()).await?;

        Ok(Self {
            window,
            graphics_state,
        })
    }
}

impl GraphicsState {
    async fn new(window: Arc<Window>) -> color_eyre::Result<Self> {
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
