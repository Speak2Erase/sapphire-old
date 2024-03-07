use std::sync::Arc;

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
    // we block on graphics because creating graphics is an async operation.
    // if we were to be running this on say, the browser, we would need to actually await this (rather than using block_on)
    let graphics = librgss::Graphics::new(&event_loop, filesystem.clone()).block_on()?;

    #[cfg(feature = "magnus")]
    let bindings_thread = sapphire_binding_magnus::start(audio, graphics, input, filesystem);

    // run the event loop to completion. for compatibility reasons, this blocks the main thread
    event_loop.run()?;

    let binding_thread_result = bindings_thread.join();
    librgss::join_handle_result_to_eyre(binding_thread_result)??;

    // not sure about this
    let audio_thread_result = audio_thread.join();
    librgss::join_handle_result_to_eyre(audio_thread_result)??;

    Ok(())
}
