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

use std::sync::mpsc::{Receiver, Sender};

use winit::event::{Event, WindowEvent};

pub struct EventLoop {
    pub(crate) event_loop: winit::event_loop::EventLoop<()>,
    pub(crate) event_sender: Sender<Event<()>>,
}

pub struct Events {
    pub(crate) event_reciever: Receiver<Event<()>>,
}

impl EventLoop {
    pub fn new() -> color_eyre::Result<(Self, Events)> {
        let event_loop = winit::event_loop::EventLoop::new()?;
        let (event_sender, event_reciever) = std::sync::mpsc::channel();

        let event_loop = Self {
            event_loop,
            event_sender,
        };
        let events = Events { event_reciever };

        Ok((event_loop, events))
    }

    pub fn run(self) -> color_eyre::Result<()> {
        self.event_loop.run(|event, target| {
            // rendering is not driven by event loop but is instead driven by Graphics::update, so we only need to wait on events
            target.set_control_flow(winit::event_loop::ControlFlow::Wait);

            if matches!(
                event,
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                }
            ) {
                target.exit();
            }

            if self.event_sender.send(event).is_err() {
                eprintln!("Event loop sender error (implies reciever was dropped), exiting");
                target.exit();
            }
        })?;

        Ok(())
    }
}
