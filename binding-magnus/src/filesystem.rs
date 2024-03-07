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

use magnus::{function, value::ReprValue, Module, RModule, Value};
use parking_lot::RwLock;
use std::sync::{Arc, OnceLock};

// FIXME find a way around using a static
pub(crate) static FILESYSTEM: OnceLock<RwLock<Arc<librgss::FileSystem>>> = OnceLock::new();

#[inline(always)]
pub fn get_filesystem() -> &'static RwLock<Arc<librgss::FileSystem>> {
    FILESYSTEM
        .get()
        .expect("filesystem static not set! please report how you encountered this crash")
}

fn load_data(path: String) -> Result<Value, magnus::Error> {
    //? SAFETY
    // This function is only exposed to Ruby. It is not possible to call this without it being called on a Ruby thread
    let ruby = unsafe { magnus::Ruby::get_unchecked() };

    // TODO this does *double* copies! which is bad.
    let filesystem = get_filesystem().read();
    // FIXME proper error handling!
    let mut file = filesystem.read_file(path).expect("failed to read file");

    let mut buf = vec![];
    file.read_to_end(&mut buf).expect("failed to read file");

    let ruby_string = ruby.str_from_slice(&buf);

    let marshal: RModule = ruby.module_kernel().const_get("Marshal")?;
    marshal.funcall("load", (ruby_string,))
}

pub fn bind(
    ruby: &magnus::Ruby,
    filesystem: Arc<librgss::FileSystem>,
) -> Result<(), magnus::Error> {
    let module = ruby.module_kernel();

    // panic if filesysten is set! this should not *ever* happen
    if FILESYSTEM.set(RwLock::new(filesystem)).is_err() {
        panic!("graphics static already set! this is not supposed to happen")
    }

    module.define_module_function("load_data", function!(load_data, 1))?;

    Ok(())
}
