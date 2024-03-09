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

use std::{
    sync::mpsc::{Receiver, Sender},
    sync::Arc,
    thread::JoinHandle,
};

use camino::Utf8PathBuf;

use crate::FileSystem;

pub struct Audio {
    sender: Sender<Event>,
}

#[derive(Debug)]
enum Event {
    PlayBGM(PlayArgs),
    StopBGM,
    FadeBGM(u32),
    PlayBGS(PlayArgs),
    StopBGS,
    FadeBGS(u32),
    PlayME(PlayArgs),
    StopME,
    FadeME(u32),
    PlaySE(PlayArgs),
    StopSE,
    Exit,
}

#[derive(Debug)]
struct PlayArgs {
    path: Utf8PathBuf,
    pitch: u32,
    volume: u32,
}

// TODO better error handling in this function
fn audio_thread_fun(
    receiver: Receiver<Event>,
    filesystem: Arc<FileSystem>,
) -> color_eyre::Result<()> {
    let (output_stream, output_stream_handle) = rodio::OutputStream::try_default()?;

    let mut bgm_sink = None;
    let mut se_sink = None;

    // TODO extract while loop body into a function to process events
    // TODO gradual event timeout (have infinite timeout when audio processing effects are not required)
    while let Ok(event) = receiver.recv() {
        println!("{:?}", event);
        match event {
            Event::PlayBGM(args) => {
                let sink = bgm_sink
                    .get_or_insert_with(|| rodio::Sink::try_new(&output_stream_handle).unwrap());

                let file = filesystem.read_file(args.path).unwrap();
                let decoder = rodio::Decoder::new_looped(file).unwrap();
                sink.append(decoder);
                sink.set_volume(args.volume as f32 / 100.);
                sink.set_speed(args.pitch as f32 / 100.);

                sink.play();
            }
            Event::StopBGM => {}
            Event::FadeBGM(_) => {}
            Event::PlayBGS(_) => {}
            Event::StopBGS => {}
            Event::FadeBGS(_) => {}
            Event::PlayME(_) => {}
            Event::StopME => {}
            Event::FadeME(_) => {}
            Event::PlaySE(args) => {
                let sink = se_sink
                    .get_or_insert_with(|| rodio::Sink::try_new(&output_stream_handle).unwrap());

                let file = filesystem.read_file(args.path).unwrap();
                let decoder = rodio::Decoder::new(file).unwrap();
                sink.append(decoder);
                sink.set_volume(args.volume as f32 / 100.);
                sink.set_speed(args.pitch as f32 / 100.);

                sink.play();
            }
            Event::StopSE => {}
            Event::Exit => return Ok(()),
        }
    }

    Ok(())
}

impl Audio {
    // Do we return a join handle as well?
    pub fn new(
        filesystem: Arc<FileSystem>,
    ) -> color_eyre::Result<(Self, JoinHandle<color_eyre::Result<()>>)> {
        let (sender, receiver) = std::sync::mpsc::channel();
        let join_handle = std::thread::spawn(|| audio_thread_fun(receiver, filesystem));

        Ok((Self { sender }, join_handle))
    }
}

// TODO handle send error. these errors mean that the audio thread has exited and we should probably panic (or return an error)
impl Audio {
    pub fn bgm_play(&self, path: impl Into<Utf8PathBuf>, volume: u32, pitch: u32) {
        let play_args = PlayArgs {
            path: path.into(),
            volume,
            pitch,
        };
        let _ = self.sender.send(Event::PlayBGM(play_args));
    }

    pub fn bgm_stop(&self) {
        let _ = self.sender.send(Event::StopBGM);
    }

    pub fn bgm_fade(&self, time: u32) {
        let _ = self.sender.send(Event::FadeBGM(time));
    }

    pub fn bgs_play(&self, path: impl Into<Utf8PathBuf>, volume: u32, pitch: u32) {
        let play_args = PlayArgs {
            path: path.into(),
            volume,
            pitch,
        };
        let _ = self.sender.send(Event::PlayBGS(play_args));
    }

    pub fn bgs_stop(&self) {
        let _ = self.sender.send(Event::StopBGS);
    }

    pub fn bgs_fade(&self, time: u32) {
        let _ = self.sender.send(Event::FadeBGS(time));
    }

    pub fn me_play(&self, path: impl Into<Utf8PathBuf>, volume: u32, pitch: u32) {
        let play_args = PlayArgs {
            path: path.into(),
            volume,
            pitch,
        };
        let _ = self.sender.send(Event::PlayME(play_args));
    }

    pub fn me_stop(&self) {
        let _ = self.sender.send(Event::StopME);
    }

    pub fn me_fade(&self, time: u32) {
        let _ = self.sender.send(Event::FadeME(time));
    }

    pub fn se_play(&self, path: impl Into<Utf8PathBuf>, volume: u32, pitch: u32) {
        let play_args = PlayArgs {
            path: path.into(),
            volume,
            pitch,
        };
        let _ = self.sender.send(Event::PlaySE(play_args));
    }

    pub fn se_stop(&self) {
        let _ = self.sender.send(Event::StopSE);
    }
}

impl Audio {
    pub fn stop_processing(&self) {
        let _ = self.sender.send(Event::Exit);
    }
}
