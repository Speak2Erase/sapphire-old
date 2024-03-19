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

use winit::event::{Event, WindowEvent};

use crate::{event_loop::UserEvent, Events};

mod buttons;
pub use buttons::{Button, KeyBind, NamedButton};

pub struct Input {
    events: Events,
    buttons: buttons::Buttons,
    exited: bool,
}

// TODO add an optional pump_events feature that uses winit::EventLoopExtPumpEvents that allows running bindings on the main thread
impl Input {
    pub fn new(events: Events) -> Self {
        Self {
            events,
            buttons: buttons::Buttons::default(),
            exited: false,
        }
    }

    /// Process all incoming events from the event loop, updating all input state.
    pub fn update(&mut self) {
        self.buttons.start_frame();
        for event in self.events.event_reciever.try_iter() {
            match event {
                // TODO handle window events
                Event::WindowEvent { event, .. } => {
                    //
                    match event {
                        WindowEvent::KeyboardInput { event, .. } => self.buttons.process_key(event),
                        WindowEvent::MouseInput { button, state, .. } if state.is_pressed() => {
                            self.buttons.process_mouse(button)
                        }
                        WindowEvent::Destroyed => self.exit(), // TODO handle properly
                        WindowEvent::CloseRequested => self.exit(), // TODO handle oneshot close stuff
                        _ => {}
                    }
                }
                Event::LoopExiting => self.exited = true,
                _ => {}
            }
        }
        println!("{:#?}", self.buttons)
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

    pub fn triggered(&self, button: Button) -> bool {
        self.buttons.triggered(button)
    }

    pub fn pressed(&self, button: Button) -> bool {
        self.buttons.pressed(button)
    }

    pub fn repeat(&self, button: Button) -> bool {
        self.buttons.repeat(button)
    }
}
