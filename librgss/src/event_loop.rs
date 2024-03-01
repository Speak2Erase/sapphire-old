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
    pub(crate) event_loop: winit::event_loop::EventLoop<UserEvent>,
    pub(crate) event_sender: Sender<Event<UserEvent>>,
}

pub struct Events {
    pub(crate) event_reciever: Receiver<Event<UserEvent>>,
    pub(crate) event_proxy: winit::event_loop::EventLoopProxy<UserEvent>,
}

pub(crate) enum UserEvent {
    ExitEventLoop,
    // TODO is this the right way to implement this?
    ExitBindingThread,
}

impl Drop for Events {
    fn drop(&mut self) {
        let _ = self.event_proxy.send_event(UserEvent::ExitEventLoop);
    }
}

impl EventLoop {
    pub fn new() -> color_eyre::Result<(Self, Events)> {
        let event_loop = winit::event_loop::EventLoopBuilder::with_user_event().build()?;
        let event_proxy = event_loop.create_proxy();

        let (event_sender, event_reciever) = std::sync::mpsc::channel();

        let event_loop = Self {
            event_loop,
            event_sender,
        };

        let events = Events {
            event_reciever,
            event_proxy,
        };

        Ok((event_loop, events))
    }

    pub fn run(self) -> color_eyre::Result<()> {
        self.event_loop.run(|event, target| {
            // rendering is not driven by event loop but is instead driven by Graphics::update, so we only need to wait on events
            target.set_control_flow(winit::event_loop::ControlFlow::Wait);

            match &event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => target.exit(),
                    WindowEvent::Destroyed => {
                        eprintln!("warning: window has been destroyed. exiting");
                        target.exit()
                    }
                    _ => {}
                },
                Event::UserEvent(event) => match event {
                    UserEvent::ExitEventLoop => target.exit(),
                    UserEvent::ExitBindingThread => {}
                },
                _ => {}
            }

            if self.event_sender.send(event).is_err() && !target.exiting() {
                eprintln!("event loop sender error (implies reciever was dropped), exiting");
                target.exit();
            }
        })?;

        Ok(())
    }
}
