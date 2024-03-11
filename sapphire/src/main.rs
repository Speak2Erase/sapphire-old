use std::sync::Arc;

use color_eyre::Section;
use pollster::FutureExt;

fn main() -> std::process::ExitCode {
    let result = run();
    match result {
        Ok(_) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("fatal error: {e:?}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn run() -> color_eyre::Result<()> {
    color_eyre::install()?;
    env_logger::init();

    let (event_loop, events) = librgss::EventLoop::new()?;
    let input = librgss::Input::new(events);

    // temporary hack
    std::env::set_current_dir("OSFM/")?;
    let filesystem = librgss::FileSystem::new(".", None).map(Arc::new)?;

    let (audio, audio_thread) = librgss::Audio::new(filesystem.clone())?;
    let mut arenas = librgss::Arenas::default();
    // we block on graphics because creating graphics is an async operation.
    // if we were to be running this on say, the browser, we would need to actually await this (rather than using block_on)
    let graphics =
        librgss::Graphics::new(&mut arenas, &event_loop, filesystem.clone()).block_on()?;

    let fonts = librgss::Fonts::new();

    #[cfg(feature = "magnus")]
    let bindings_thread =
        sapphire_binding_magnus::start(audio, arenas, graphics, fonts, input, filesystem);

    // run the event loop to completion. for compatibility reasons, this blocks the main thread
    event_loop.run()?;

    let binding_thread_result = bindings_thread.join();
    librgss::join_handle_result_to_eyre(binding_thread_result)
        .note("panic in binding thread")?
        .note("fatal error in binding thread")?;

    // not sure about this
    let audio_thread_result = audio_thread.join();
    librgss::join_handle_result_to_eyre(audio_thread_result)
        .note("panic in audio thread")?
        .note("error in audio thread")?;

    Ok(())
}
