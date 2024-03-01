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

    let (event_loop, events) = librgss::EventLoop::new()?;

    let audio = librgss::Audio::new()?;
    let graphics = librgss::Graphics::new(&event_loop).block_on()?;
    let input = librgss::Input::new(events);

    #[cfg(feature = "magnus")]
    let bindings_thread = unsafe { rsgss_binding_magnus::start(audio, graphics, input) };

    event_loop.run()?;

    match bindings_thread.join() {
        Ok(result) => result,
        Err(e) => {
            if let Some(&e) = e.downcast_ref::<&'static str>() {
                Err(color_eyre::Report::msg(e))
            } else if let Ok(e) = e.downcast::<String>() {
                Err(color_eyre::Report::msg(e))
            } else {
                Err(color_eyre::Report::msg("Any { .. }"))
            }
        }
    }
}
