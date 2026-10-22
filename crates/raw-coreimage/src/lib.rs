//! Apple RAW decoding. The only caller is the supervised helper process, never the UI process.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
#[cfg(target_os = "macos")]
#[allow(unsafe_code)]
mod native;
#[cfg(target_os = "macos")]
pub use native::Decoder;
#[cfg(not(target_os = "macos"))]
pub struct Decoder;
#[cfg(not(target_os = "macos"))]
impl Decoder {
    pub fn new() -> Result<Self, lightcraft_raw_backend::BackendError> {
        Err(lightcraft_raw_backend::BackendError::unsupported("Core Image requires macOS 12 or newer"))
    }
    pub fn process(
        &self,
        _: lightcraft_raw_backend::Operation,
        _: &[u8],
    ) -> Result<lightcraft_raw_backend::Decoded, lightcraft_raw_backend::BackendError> {
        Err(lightcraft_raw_backend::BackendError::unsupported("Core Image requires macOS 12 or newer"))
    }
}
