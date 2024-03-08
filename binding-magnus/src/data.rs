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

use magnus::{function, Object, RString, Value};

#[magnus::wrap(class = "Color", size, free_immediately)]
pub struct Color(pub librgss::Color);

fn color_new(args: &[Value]) -> Result<Color, magnus::Error> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

    let (red, green, blue) = args.required;
    let (alpha,) = args.optional;

    let color = librgss::Color {
        red,
        blue,
        green,
        alpha: alpha.unwrap_or(255.0),
    };

    Ok(Color(color))
}

fn deserialize_color(bytes: RString) -> Color {
    //? Safety
    // We don't store bytes anywhere or hold onto it long enough for ruby to garbage colect it.
    let bytes = unsafe { bytes.as_slice() };

    let color = bytemuck::cast_slice(bytes)[0];
    Color(color)
}

#[magnus::wrap(class = "Tone", size, free_immediately)]
pub struct Tone(pub librgss::Tone);

fn tone_new(args: &[Value]) -> Result<Tone, magnus::Error> {
    let args = magnus::scan_args::scan_args::<_, _, (), (), (), ()>(args)?;

    let (red, green, blue) = args.required;
    let (grey,) = args.optional;

    let tone = librgss::Tone {
        red,
        blue,
        green,
        grey: grey.unwrap_or(0.0),
    };

    Ok(Tone(tone))
}

fn deserialize_tone(bytes: RString) -> Tone {
    //? Safety
    // We don't store bytes anywhere or hold onto it long enough for ruby to garbage colect it.
    let bytes = unsafe { bytes.as_slice() };

    let tone = bytemuck::cast_slice(bytes)[0];
    Tone(tone)
}

#[magnus::wrap(class = "Table", size, free_immediately)]
pub struct Table(pub librgss::Table);

fn deserialize_table(bytes: RString) -> Table {
    //? Safety
    // We don't store bytes anywhere or hold onto it long enough for ruby to garbage colect it.
    let bytes = unsafe { bytes.as_slice() };

    let u32_slice: &[u32] = bytemuck::cast_slice(bytes);

    let [_, xsize, ysize, zsize, len, data @ ..] = u32_slice else {
        todo!()
    };
    let data = bytemuck::cast_slice(data).to_vec();
    assert_eq!(*len as usize, data.len());

    let table = librgss::Table::new_data(*xsize as usize, *ysize as usize, *zsize as usize, data);
    Table(table)
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let color = ruby.define_class("Color", ruby.class_object())?;

    color.define_singleton_method("new", function!(color_new, -1))?;
    color.define_singleton_method("_load", function!(deserialize_color, 1))?;

    let tone = ruby.define_class("Tone", ruby.class_object())?;

    tone.define_singleton_method("new", function!(tone_new, -1))?;
    tone.define_singleton_method("_load", function!(deserialize_tone, 1))?;

    let table = ruby.define_class("Table", ruby.class_object())?;

    table.define_singleton_method("_load", function!(deserialize_table, 1))?;

    Ok(())
}
