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

pub struct Audio {
    stream: rodio::OutputStream,
    stream_handle: rodio::OutputStreamHandle,
}

// FIXME this is probably not correct
unsafe impl Send for Audio {}

impl Audio {
    pub fn new() -> color_eyre::Result<Self> {
        let (stream, stream_handle) = rodio::OutputStream::try_default()?;

        Ok(Self {
            stream,
            stream_handle,
        })
    }
}
