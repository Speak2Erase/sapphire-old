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

use winit::event::Event;

use crate::{event_loop::UserEvent, Events};

pub struct Input {
    events: Events,
    exited: bool,
}

impl Input {
    pub fn new(events: Events) -> Self {
        Self {
            events,
            exited: false,
        }
    }

    /// Process all incoming events from the event loop, updating all input state.
    pub fn update(&mut self) {
        for event in self.events.event_reciever.try_iter() {
            match event {
                // TODO handle window events
                Event::WindowEvent { window_id, event } => {}
                Event::LoopExiting => self.exited = true,
                _ => {}
            }
        }
    }

    /// Notifies the event loop that we'd like to exit.
    pub fn exit(&self) {
        let _ = self.events.event_proxy.send_event(UserEvent::ExitEventLoop);
    }

    /// Returns true if the event loop has exited.
    ///
    /// You should always exit after this.
    pub fn exited(&self) -> bool {
        self.exited
    }
}
