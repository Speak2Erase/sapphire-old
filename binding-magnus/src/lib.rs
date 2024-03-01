#![forbid(unsafe_op_in_unsafe_fn)]

mod scripts;
use scripts::Script;

mod error;

/// # Safety
///
/// Must be called in main(), or at least a function higher up the stack than any code calling Ruby.
/// Must not drop [`Bindings`] until the very end of the process, after all Ruby execution has finished.
/// Do not use Ruby values after Cleanup has been dropped.
pub unsafe fn start(
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    input: librgss::Input,
) -> std::thread::JoinHandle<color_eyre::Result<()>> {
    std::thread::Builder::new()
        .name("librgss ruby thread".to_string())
        .spawn(move || unsafe { run_ruby_thread(audio, graphics, input) })
        .expect("failed to start ruby thread")
}

unsafe fn run_ruby_thread(
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    input: librgss::Input,
) -> color_eyre::Result<()> {
    let cleanup = unsafe { magnus::embed::init() };

    std::env::set_current_dir("OSFM/")?;

    init_bindings(&cleanup).map_err(error::magnus_to_eyre)?;

    let script_data = std::fs::read("Data/xScripts.rxdata")?;
    let scripts: Vec<Script> = alox_48::from_bytes(&script_data)?;

    // run all scripts.
    // due to the design of rgss, this will block until script completion
    for script in scripts {
        cleanup.script(script.name);
        cleanup
            .eval::<magnus::Value>(&script.script_text)
            .map_err(error::magnus_to_eyre)?;
    }

    // when input is dropped the event loop will exit by proxy
    Ok(())
}

#[cfg_attr(not(feature = "embed"), magnus::init)]
fn init_bindings(ruby: &magnus::Ruby) -> Result<(), magnus::Error> {
    #[cfg(feature = "modshot")]
    ruby.eval::<magnus::Value>(
        "$LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby'))\n
            $LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby', RUBY_PLATFORM))\n",
    )?;

    Ok(())
}
