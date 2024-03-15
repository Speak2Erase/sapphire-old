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

use crossbeam::atomic::AtomicCell;
use magnus::{function, method, Class, Module, Object, RString, TryConvert, Value};
use parking_lot::RwLock;

use crate::helpers::{ColorProvider, Provider, ProviderVal, RectProvider};

#[derive(Default)]
#[magnus::wrap(class = "Color", size, free_immediately)]
pub struct Color(pub RwLock<ProviderVal<librgss::Color, ColorProvider>>);

impl Color {
    fn initialize(&self, args: &[Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

        let (red, green, blue) = args.required;
        let (alpha,) = args.optional;

        let color = librgss::Color {
            red,
            blue,
            green,
            alpha: alpha.unwrap_or(255.0),
        };

        let mut provider = self.0.write();
        provider.provide_mut(|c| *c = color);

        Ok(())
    }

    fn deserialize(bytes: RString) -> Color {
        //? Safety
        // We don't store bytes anywhere or hold onto it long enough for ruby to garbage colect it.
        let bytes = unsafe { bytes.as_slice() };

        let color: librgss::Color = *bytemuck::from_bytes(bytes);
        Color(ProviderVal::val(color).into())
    }

    fn serialize(color: &Color) -> RString {
        let provider = color.0.read();
        let color = provider.provide_copy();
        let bytes = bytemuck::bytes_of(&color);
        RString::from_slice(bytes)
    }

    pub fn from_provider(p: impl Into<ColorProvider>) -> Self {
        let provider = ProviderVal::provider(p);
        Self(RwLock::new(provider))
    }

    pub fn as_color(&self) -> librgss::Color {
        self.0.read().provide_copy()
    }
}

#[derive(Default)]
#[magnus::wrap(class = "Tone", size, free_immediately)]
pub struct Tone(pub librgss::SharedTone);

impl Tone {
    fn initialize(&self, args: &[Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

        let (red, green, blue) = args.required;
        let (grey,) = args.optional;

        let tone = librgss::Tone {
            red,
            blue,
            green,
            grey: grey.unwrap_or(0.0),
        };
        self.0.store(tone);

        Ok(())
    }

    fn deserialize(bytes: RString) -> Tone {
        //? Safety
        // We don't store bytes anywhere or hold onto it long enough for ruby to garbage colect it.
        let bytes = unsafe { bytes.as_slice() };

        let tone: librgss::Tone = *bytemuck::from_bytes(bytes);
        Tone(tone.into())
    }

    fn serialize(tone: &Tone) -> RString {
        let tone = tone.0.load();
        let bytes = bytemuck::bytes_of(&tone);
        RString::from_slice(bytes)
    }
}

#[derive(Default)]
#[magnus::wrap(class = "Rect", size, free_immediately, frozen_shareable)]
pub struct Rect(pub(crate) RwLock<ProviderVal<librgss::Rect, RectProvider>>);

impl Rect {
    fn initialize(&self, x: i32, y: i32, width: u32, height: u32) {
        self.set(x, y, width, height)
    }

    fn set(&self, x: i32, y: i32, width: u32, height: u32) {
        let mut provider = self.0.write();
        provider.provide_mut(|rect| *rect = librgss::Rect::new(x, y, width, height));
    }

    fn empty(&self) {
        self.set(0, 0, 0, 0)
    }

    fn width(&self) -> u32 {
        self.as_rect().width
    }

    fn height(&self) -> u32 {
        self.as_rect().height
    }

    pub fn from_provider(p: impl Into<RectProvider>) -> Self {
        let provider = ProviderVal::provider(p);
        Self(RwLock::new(provider))
    }

    pub fn from_val(p: impl Into<librgss::Rect>) -> Self {
        let provider = ProviderVal::val(p);
        Self(RwLock::new(provider))
    }

    pub fn as_rect(&self) -> librgss::Rect {
        self.0.read().provide_copy()
    }
}

#[derive(Default)]
#[magnus::wrap(class = "Table", size, free_immediately, frozen_shareable)]
pub struct Table(pub librgss::SharedTable);

impl Table {
    fn initialize(&self, args: &[Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

        let (xsize,) = args.required;
        let (ysize, zsize) = args.optional;

        let table = librgss::Table::new(xsize, ysize.unwrap_or(0), zsize.unwrap_or(0));
        *self.0.write() = table;

        Ok(())
    }

    fn xsize(&self) -> usize {
        self.0.read().xsize()
    }

    fn ysize(&self) -> usize {
        self.0.read().ysize()
    }

    fn zsize(&self) -> usize {
        self.0.read().zsize()
    }

    fn resize(&self, args: &[Value]) -> Result<(), magnus::Error> {
        let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

        let (xsize,) = args.required;
        let (ysize, zsize) = args.optional;

        let mut table = self.0.write();
        table.resize(xsize, ysize.unwrap_or(0), zsize.unwrap_or(0));

        Ok(())
    }

    fn get(&self, args: &[Value]) -> Result<i16, magnus::Error> {
        let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

        let (x,) = args.required;
        let (y, z) = args.optional;

        let table = self.0.read();
        let value = table[(x, y.unwrap_or(0), z.unwrap_or(0))];

        Ok(value)
    }

    fn set(&self, args: &[Value]) -> Result<(), magnus::Error> {
        let (x, y, z, val) = match *args {
            [x, val] => {
                let x = usize::try_convert(x)?;
                let val = i16::try_convert(val)?;

                (x, 0, 0, val)
            }
            [x, y, val] => {
                let x = usize::try_convert(x)?;
                let y = usize::try_convert(y)?;
                let val = i16::try_convert(val)?;

                (x, y, 0, val)
            }
            [x, y, z, val] => {
                let x = usize::try_convert(x)?;
                let y = usize::try_convert(y)?;
                let z = usize::try_convert(z)?;
                let val = i16::try_convert(val)?;

                (x, y, z, val)
            }
            _ => {
                let err = magnus::Error::new(
                    magnus::exception::arg_error(),
                    "wrong number  of arguments",
                );
                return Err(err);
            }
        };

        let mut table = self.0.write();
        table[(x, y, z)] = val;

        Ok(())
    }

    fn deserialize(bytes: RString) -> Table {
        //? Safety
        // We don't store bytes anywhere or hold onto it long enough for ruby to garbage colect it.
        let bytes = unsafe { bytes.as_slice() };

        let u32_slice: &[u32] = bytemuck::cast_slice(bytes);

        let [_, xsize, ysize, zsize, len, data @ ..] = u32_slice else {
            todo!()
        };
        let data = bytemuck::cast_slice(data).to_vec();
        assert_eq!(*len as usize, data.len());

        let table =
            librgss::Table::new_data(*xsize as usize, *ysize as usize, *zsize as usize, data);
        Table(table.into())
    }

    fn serialize(table: &Table) -> RString {
        let table = &table.0.read();
        // FIXME calculate capacity
        let string = RString::buf_new(0);

        let size = 1 + (table.ysize() > 0) as u32 + (table.zsize() > 0) as u32;
        let header = [
            size,
            table.xsize() as u32,
            table.ysize() as u32,
            table.zsize() as u32,
            table.len() as u32,
        ];

        string.cat(bytemuck::bytes_of(&header));
        string.cat(bytemuck::cast_slice(table.data()));

        string
    }
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let color = ruby.define_class("Color", ruby.class_object())?;

    color.define_alloc_func::<Color>();
    color.define_method("initialize", method!(Color::initialize, -1))?;
    color.define_singleton_method("_load", function!(Color::deserialize, 1))?;
    color.define_method("_dump_data", method!(Color::serialize, 0))?;

    let tone = ruby.define_class("Tone", ruby.class_object())?;

    tone.define_alloc_func::<Tone>();
    tone.define_method("initialize", method!(Tone::initialize, -1))?;
    tone.define_singleton_method("_load", function!(Tone::deserialize, 1))?;
    tone.define_method("_dump_data", method!(Tone::serialize, 0))?;

    let rect = ruby.define_class("Rect", ruby.class_object())?;

    rect.define_alloc_func::<Rect>();
    rect.define_method("initialize", method!(Rect::initialize, 4))?;

    rect.define_method("width", method!(Rect::width, 0))?;
    rect.define_method("height", method!(Rect::height, 0))?;

    rect.define_method("set", method!(Rect::set, 4))?;
    rect.define_method("empty", method!(Rect::empty, 0))?;

    let table = ruby.define_class("Table", ruby.class_object())?;

    table.define_alloc_func::<Table>();
    table.define_method("initialize", method!(Table::initialize, -1))?;
    table.define_singleton_method("_load", function!(Table::deserialize, 1))?;
    table.define_method("_dump_data", method!(Table::serialize, 0))?;

    table.define_method("xsize", method!(Table::xsize, 0))?;
    table.define_method("ysize", method!(Table::ysize, 0))?;
    table.define_method("zsize", method!(Table::zsize, 0))?;
    table.define_method("resize", method!(Table::resize, -1))?;

    table.define_method("[]", method!(Table::get, -1))?;
    table.define_method("[]=", method!(Table::set, -1))?;

    Ok(())
}
