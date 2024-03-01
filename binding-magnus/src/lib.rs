#![forbid(unsafe_op_in_unsafe_fn)]

mod scripts;
use scripts::Script;

pub type Result<T> = std::result::Result<T, magnus::Error>;

/// # Safety
///
/// Must be called in main(), or at least a function higher up the stack than any code calling Ruby.
/// Must not drop [`Bindings`] until the very end of the process, after all Ruby execution has finished.
/// Do not use Ruby values after Cleanup has been dropped.
pub unsafe fn start(
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    input: librgss::Input,
) -> std::thread::JoinHandle<()> {
    std::thread::Builder::new()
        .name("librgss ruby thread".to_string())
        .spawn(move || unsafe { run_ruby_thread(audio, graphics, input) })
        .expect("failed to start ruby thread")
}

unsafe fn run_ruby_thread(
    audio: librgss::Audio,
    graphics: librgss::Graphics,
    input: librgss::Input,
) {
    let cleanup = unsafe { magnus::embed::init() };

    std::env::set_current_dir("OSFM/").unwrap();

    #[cfg(feature = "modshot")]
    cleanup
        .eval::<magnus::Value>(
            "$LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby'))\n
            $LOAD_PATH.unshift(File.join(Dir.pwd, 'lib', 'ruby', RUBY_PLATFORM))\n",
        )
        .expect("failed to setup load path");

    init_bindings(&cleanup).expect("failed to init ruby bindings");

    let script_data = std::fs::read("Data/xScripts.rxdata").expect("failed to read script data");
    let scripts: Vec<Script> =
        alox_48::from_bytes(&script_data).expect("failed to decode script data");

    // run all scripts.
    // due to the design of rgss, this will block until script completion
    for script in scripts {
        cleanup.script(script.name);
        cleanup
            .eval::<magnus::Value>(&script.script_text)
            .expect("failed to evaluate script");
    }

    // when input is dropped the event loop will exit by proxy
    // this is because the Events struct inside Input has a reciever used for passing events
    // when the reciever is dropped, the event loop will exit
}

#[cfg_attr(not(feature = "embed"), magnus::init)]
fn init_bindings(ruby: &magnus::Ruby) -> Result<()> {
    Ok(())
}
