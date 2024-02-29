use color_eyre::eyre::Ok;
use pollster::FutureExt;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let (event_loop, events) = librgss::EventLoop::new()?;

    let audio = librgss::Audio::new()?;
    let graphics = librgss::Graphics::new(&event_loop).block_on()?;
    let input = librgss::Input::new(events);

    #[cfg(feature = "magnus")]
    let bindings_thread = unsafe { rsgss_binding_magnus::start(audio, graphics, input) };

    event_loop.run()?;

    if let Err(e) = bindings_thread.join() {
        eprintln!("error: binding thread panicked with {e:?}");
    }

    Ok(())
}
