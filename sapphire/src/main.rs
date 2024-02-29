fn main() {
    #[cfg(feature = "magnus")]
    let bindings = unsafe { rsgss_binding_magnus::Bindings::new() };
}
