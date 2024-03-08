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

use magnus::{function, Object, Value};

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
        alpha: alpha.unwrap_or(255),
    };

    Ok(Color(color))
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
        grey: grey.unwrap_or(0),
    };

    Ok(Tone(tone))
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let color = ruby.define_class("Color", ruby.class_object())?;

    color.define_singleton_method("new", function!(color_new, -1))?;

    let tone = ruby.define_class("Tone", ruby.class_object())?;

    tone.define_singleton_method("new", function!(tone_new, -1))?;

    Ok(())
}
