fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let (event_loop, events) = librgss::EventLoop::new()?;

    let audio = librgss::Audio::new()?;
    let graphics = librgss::Graphics::new(&event_loop)?;

    #[cfg(feature = "magnus")]
    let bindings = unsafe { rsgss_binding_magnus::Bindings::new() };

    event_loop.run()
}
