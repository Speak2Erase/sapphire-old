mod audio;
pub use audio::Audio;

mod data;
pub use data::{Color, Rect, Table, Tone};

mod event_loop;
pub use event_loop::{EventLoop, Events};

mod filesystem;
pub use filesystem::{Error, File, FileSystem};

mod font;
pub use font::{Font, Fonts};

pub mod graphics;
pub use graphics::{Bitmap, Graphics, Plane, Sprite, Tilemap, Viewport};

mod input;
pub use input::Input;

pub fn join_handle_result_to_eyre<T>(result: std::thread::Result<T>) -> color_eyre::Result<T> {
    result.map_err(|e| {
        if let Some(&e) = e.downcast_ref::<&'static str>() {
            color_eyre::Report::msg(e)
        } else if let Ok(e) = e.downcast::<String>() {
            color_eyre::Report::msg(e)
        } else {
            color_eyre::Report::msg("Any { .. }")
        }
    })
}
