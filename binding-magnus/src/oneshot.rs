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

use magnus::{function, Module};

fn set_yes_no(yes: String, no: String) {}
fn msgbox(kind: u8, text: String) -> bool {
    false
}

fn exiting(value: bool) {}

fn allow_exit(value: bool) {}

fn wallpaper_reset() {}

fn journal_set(name: String) {}

fn journal_active() -> bool {
    false
}

pub fn bind(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    let oneshot = ruby.define_module("Oneshot")?;

    oneshot.define_module_function("set_yes_no", function!(set_yes_no, 2))?;
    oneshot.define_module_function("msgbox", function!(msgbox, 2))?;

    oneshot.define_module_function("exiting", function!(exiting, 1))?;
    oneshot.define_module_function("allow_exit", function!(allow_exit, 1))?;

    let msg = oneshot.define_module("Msg")?;
    msg.const_set("INFO", 1)?;
    msg.const_set("YESNO", 3)?;

    let data_dir = dirs::data_local_dir()
        .expect("no data dir found")
        .join("OneShot");
    if !data_dir.exists() {
        std::fs::create_dir(&data_dir).expect("failed to create data dir");
    }

    let docs_dir = dirs::document_dir().expect("no document dir found");
    let game_dir = docs_dir.join("MyGames");

    let username = whoami::username();

    oneshot.const_set("SAVE_PATH", data_dir)?;
    oneshot.const_set("DOCS_PATH", docs_dir)?;
    oneshot.const_set("GAME_PATH", game_dir)?;
    oneshot.const_set("USER_NAME", username)?;

    oneshot.const_set("LANG", "en")?;

    #[cfg(target_os = "linux")]
    oneshot.const_set("OS", "linux")?;
    #[cfg(target_os = "windows")]
    module.const_set("OS", "windows")?;

    let wallpaper = ruby.define_module("Wallpaper")?;

    wallpaper.define_module_function("reset", function!(wallpaper_reset, 0))?;

    let journal = ruby.define_module("Journal")?;

    journal.define_module_function("set", function!(journal_set, 1))?;
    journal.define_module_function("active?", function!(journal_active, 0))?;

    Ok(())
}
