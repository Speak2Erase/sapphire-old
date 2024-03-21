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
// along with sapphire.  If not, see <https://www.gnu.org/licenses/>.

mod binding_helpers;
pub use binding_helpers::{BindGroupBuilder, BindGroupLayoutBuilder};
mod shaders;
pub use shaders::{BindGroups, RenderPipelines};
mod primitives;
pub use primitives::{Quad, Rect, Vertex};
