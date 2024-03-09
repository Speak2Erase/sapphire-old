#![forbid(unsafe_op_in_unsafe_fn)]

mod scripts;
use std::sync::Arc;

use magnus::{function, value::ReprValue};
use scripts::Script;

mod error;

mod data;

mod audio;

mod filesystem;

mod bitmap;
mod font;
mod graphics;
mod plane;
mod sprite;
mod tilemap;
mod viewport;
mod window;

mod input;

#[cfg(feature = "modshot")]
mod modshot;
#[cfg(feature = "modshot")]
mod oneshot;
#[cfg(feature = "modshot")]
mod steam;

mod helpers;

mod rpg;

pub fn start(
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    fonts: librgss::Fonts,
    input: librgss::Input,
    filesystem: Arc<librgss::FileSystem>,
) -> std::thread::JoinHandle<color_eyre::Result<()>> {
    std::thread::Builder::new()
        .name("librgss ruby thread".to_string())
        .spawn(move || {
            //? Safety
            //? These bindings don't provide a way to access ruby values *at all* so it's not possible to access ruby values outside of this function call.
            let result = unsafe { run_ruby_thread(audio, graphics, fonts, input, filesystem) };
            // exit the event loop after we're finished running ruby code (for any reason)
            input::get_input().read().exit();
            // stop audio processing
            // FIXME should we do this here, or in main.rs?
            audio::get_audio().read().stop_processing();

            result
        })
        .expect("failed to start ruby thread")
}

unsafe fn run_ruby_thread(
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    fonts: librgss::Fonts,
    input: librgss::Input,
    filesystem: Arc<librgss::FileSystem>,
) -> color_eyre::Result<()> {
    let ruby = unsafe { magnus::embed::init() };

    // It is *really* important that we call this function before doing anyhting else!
    // If any initialization fails, input::get_input() might fail and we will panic.
    init_bindings(&ruby, audio, graphics, fonts, input, filesystem)
        .map_err(error::magnus_to_eyre)?;

    rpg::eval(&ruby).map_err(error::magnus_to_eyre)?;

    // FIXME should we just use marshal directly from ruby?
    let script_data = std::fs::read("Data/xScripts.rxdata")?;
    let scripts: Vec<Script> = alox_48::from_bytes(&script_data)?;

    // run all scripts. due to the design of rgss, this will block until script completion
    // if the event loop has exited, the next call to Input::update will raise SystemExit, so this loop will exit
    for script in scripts {
        ruby.script(script.name);
        let result = ruby.eval::<magnus::Value>(&script.script_text);

        if let Err(error) = result {
            if !error.is_kind_of(ruby.exception_system_exit()) {
                return Err(error::magnus_to_eyre(error));
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "embed"))]
#[magnus::init]
fn init(ruby: &magnus::Ruby) {
    // TODO figure out how to run the event loop on demand or on another thread (in a cross platform way)
    todo!()
}

fn print(value: magnus::Value) {
    let string_data = value.to_string();
    rfd::MessageDialog::new()
        .set_buttons(rfd::MessageButtons::Ok)
        .set_description(string_data)
        .set_title("Sapphire")
        .show();
}

fn p(value: magnus::Value) {
    let string_data = value.inspect();
    rfd::MessageDialog::new()
        .set_buttons(rfd::MessageButtons::Ok)
        .set_description(string_data)
        .show();
}

fn init_bindings(
    ruby: &magnus::Ruby,
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    fonts: librgss::Fonts,
    input: librgss::Input,
    filesystem: Arc<librgss::FileSystem>,
) -> Result<(), magnus::Error> {
    audio::bind(ruby, audio)?;

    data::bind(ruby)?;
    error::bind(ruby)?;

    filesystem::bind(ruby, filesystem)?;

    graphics::bind(ruby, graphics)?;
    bitmap::bind(ruby)?;
    sprite::bind(ruby)?;
    font::bind(ruby, fonts)?;
    plane::bind(ruby)?;
    tilemap::bind(ruby)?;
    viewport::bind(ruby)?;
    window::bind(ruby)?;

    input::bind(ruby, input)?;

    #[cfg(feature = "modshot")]
    {
        oneshot::bind(ruby)?;
        modshot::bind(ruby)?;
        steam::bind(ruby)?;
    }

    #[cfg(feature = "modshot")]
    ruby.eval::<magnus::Value>(
        "$LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby'))\n
         $LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby', RUBY_PLATFORM))\n",
    )?;

    let kernel = ruby.module_kernel();
    kernel.define_module_function("p", function!(p, 1))?;
    kernel.define_module_function("print", function!(print, 1))?;

    Ok(())
}
