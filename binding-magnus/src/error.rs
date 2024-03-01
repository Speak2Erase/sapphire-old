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

use color_eyre::Section;
use magnus::{error::ErrorType, value::ReprValue, Class};

pub fn magnus_to_eyre(value: magnus::Error) -> color_eyre::Report {
    let handle = unsafe { magnus::Ruby::get_unchecked() };

    let exception = match value.error_type() {
        ErrorType::Jump(_) => unimplemented!(),
        ErrorType::Error(class, msg) => class
            .new_instance((handle.str_new(msg.as_ref()),))
            .expect("fatal error converting magnus error"),
        ErrorType::Exception(exception) => *exception,
    };

    // we're pretty quckily converting this to an owned value so this is ok.
    let class_name = unsafe { exception.classname() };
    let mut report = color_eyre::Report::msg(format!("{class_name}: {exception}"));
    // get rid of class_name so we don't have to worry about it.
    drop(class_name);

    let backtrace: Option<magnus::RArray> = exception.funcall("backtrace", ()).ok().flatten();
    if let Some(backtrace) = backtrace {
        report = report.note("ruby backtrace:");
        for line in backtrace.each() {
            let Ok(line) = line else {
                break;
            };
            report = report.note(line.to_string());
        }
    }

    report
}
