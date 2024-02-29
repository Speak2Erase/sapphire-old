#[cfg(feature = "embed")]
pub struct Bindings {
    cleanup: magnus::embed::Cleanup,
}

#[cfg(feature = "embed")]
impl Bindings {
    /// # Safety
    ///
    /// Must be called in main(), or at least a function higher up the stack than any code calling Ruby.
    /// Must not drop [`Bindings`] until the very end of the process, after all Ruby execution has finished.
    /// Do not use Ruby values after Cleanup has been dropped.
    pub unsafe fn new() -> Self {
        let cleanup = unsafe { magnus::embed::init() };
        Self { cleanup }
    }
}

pub type Result<T> = std::result::Result<T, magnus::Error>;

#[cfg_attr(not(feature = "embed"), magnus::init)]
fn init_bindings(ruby: &magnus::Ruby) -> Result<()> {
    Ok(())
}
