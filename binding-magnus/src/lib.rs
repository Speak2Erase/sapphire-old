#![forbid(unsafe_op_in_unsafe_fn)]

mod scripts;
use scripts::Script;

mod error;

mod audio;

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
) -> std::thread::JoinHandle<color_eyre::Result<()>> {
    std::thread::Builder::new()
        .name("librgss ruby thread".to_string())
        .spawn(move || {
            //? Safety
            //? These bindings don't provide a way to access ruby values *at all* so it's not possible to access ruby values outside of this function call.
            let result = unsafe { run_ruby_thread(audio, graphics, input) };
            // exit the event loop after we're finished running ruby code (for any reason)
            input::get_input().read().exit();
            result
        })
        .expect("failed to start ruby thread")
}

unsafe fn run_ruby_thread(
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    input: librgss::Input,
) -> color_eyre::Result<()> {
    let cleanup = unsafe { magnus::embed::init() };

    std::env::set_current_dir("OSFM/")?;

    init_bindings(&cleanup, audio, graphics, input).map_err(error::magnus_to_eyre)?;

    rpg::eval(&cleanup).map_err(error::magnus_to_eyre)?;

    let script_data = std::fs::read("Data/xScripts.rxdata")?;
    let scripts: Vec<Script> = alox_48::from_bytes(&script_data)?;

    // run all scripts. due to the design of rgss, this will block until script completion
    // if the event loop has exited, the next call to Input::update will raise SystemExit, so this loop will exit
    for script in scripts {
        cleanup.script(script.name);
        cleanup
            .eval::<magnus::Value>(&script.script_text)
            .map_err(error::magnus_to_eyre)?;
    }

    Ok(())
}

#[cfg_attr(not(feature = "embed"), magnus::init)]
fn init_bindings(
    ruby: &magnus::Ruby,
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    input: librgss::Input,
) -> Result<(), magnus::Error> {
    #[cfg(feature = "modshot")]
    ruby.eval::<magnus::Value>(
        "$LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby'))\n
         $LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby', RUBY_PLATFORM))\n",
    )?;

    audio::bind(ruby, audio)?;

    graphics::bind(ruby, graphics)?;
    bitmap::bind(ruby)?;
    sprite::bind(ruby)?;
    font::bind(ruby)?;
    plane::bind(ruby)?;
    tilemap::bind(ruby)?;

    input::bind(ruby, input)?;

    #[cfg(feature = "modshot")]
    oneshot::bind(ruby)?;

    Ok(())
}
