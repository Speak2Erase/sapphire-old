#![forbid(unsafe_op_in_unsafe_fn)]

mod scripts;
use std::sync::Arc;

use scripts::Script;

mod error;

mod audio;

mod filesystem;

mod bitmap;
mod font;
mod graphics;
mod plane;
mod sprite;
mod tilemap;

mod input;

#[cfg(feature = "modshot")]
mod oneshot;

mod rpg;

pub fn start(
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    input: librgss::Input,
    filesystem: Arc<librgss::FileSystem>,
) -> std::thread::JoinHandle<color_eyre::Result<()>> {
    std::thread::Builder::new()
        .name("librgss ruby thread".to_string())
        .spawn(move || {
            //? Safety
            //? These bindings don't provide a way to access ruby values *at all* so it's not possible to access ruby values outside of this function call.
            let result = unsafe { run_ruby_thread(audio, graphics, input, filesystem) };
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
    input: librgss::Input,
    filesystem: Arc<librgss::FileSystem>,
) -> color_eyre::Result<()> {
    let ruby = unsafe { magnus::embed::init() };

    // It is *really* important that we call this function before doing anyhting else!
    // If any initialization fails, input::get_input() might fail and we will panic.
    init_bindings(&ruby, audio, graphics, input, filesystem).map_err(error::magnus_to_eyre)?;

    rpg::eval(&ruby).map_err(error::magnus_to_eyre)?;

    // FIXME should we just use marshal directly from ruby?
    let script_data = std::fs::read("Data/xScripts.rxdata")?;
    let scripts: Vec<Script> = alox_48::from_bytes(&script_data)?;

    // run all scripts. due to the design of rgss, this will block until script completion
    // if the event loop has exited, the next call to Input::update will raise SystemExit, so this loop will exit
    for script in scripts {
        ruby.script(script.name);
        ruby.eval::<magnus::Value>(&script.script_text)
            .map_err(error::magnus_to_eyre)?;
    }

    Ok(())
}

#[cfg(not(feature = "embed"))]
#[magnus::init]
fn init(ruby: &magnus::Ruby) {
    // TODO figure out how to run the event loop on demand or on another thread (in a cross platform way)
    todo!()
}

fn init_bindings(
    ruby: &magnus::Ruby,
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    input: librgss::Input,
    filesystem: Arc<librgss::FileSystem>,
) -> Result<(), magnus::Error> {
    audio::bind(ruby, audio)?;

    filesystem::bind(ruby, filesystem)?;

    graphics::bind(ruby, graphics)?;
    bitmap::bind(ruby)?;
    sprite::bind(ruby)?;
    font::bind(ruby)?;
    plane::bind(ruby)?;
    tilemap::bind(ruby)?;

    input::bind(ruby, input)?;

    #[cfg(feature = "modshot")]
    oneshot::bind(ruby)?;

    #[cfg(feature = "modshot")]
    ruby.eval::<magnus::Value>(
        "$LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby'))\n
         $LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby', RUBY_PLATFORM))\n",
    )?;

    Ok(())
}
