// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! Supervised native RAW worker. stdout is exclusively the binary protocol.
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use lightcraft_raw_backend::{read_request, write_response};
fn run() -> Result<(), String> {
    let decoder = lightcraft_raw_coreimage::Decoder::new().map_err(|e| e.to_string())?;
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    let mut input = stdin.lock();
    let mut output = stdout.lock();
    while let Some((operation, bytes)) = read_request(&mut input)? {
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| decoder.process(operation, &bytes)))
            .unwrap_or_else(|_| Err(lightcraft_raw_backend::BackendError::decode("Core Image worker request failed unexpectedly")));
        write_response(&mut output, result)?;
    }
    Ok(())
}
fn main() -> std::process::ExitCode {
    match run() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("RAW worker: {e}");
            std::process::ExitCode::FAILURE
        }
    }
}
