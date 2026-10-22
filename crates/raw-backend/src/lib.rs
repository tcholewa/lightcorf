// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! Safe, bounded protocol and supervised client for the crash-isolated RAW worker.
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use lightcraft_raster::Rgb32f;
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};

#[cfg(not(target_arch = "wasm32"))]
mod client;
#[cfg(not(target_arch = "wasm32"))]
pub use client::Client;

pub const VERSION: u32 = 1;
pub const RECIPE: &str = "coreimage-linear-rec2020-v1";
pub const MAX_INPUT: usize = 1024 * 1024 * 1024;
pub const MAX_PIXELS: usize = 128 * 1024 * 1024;
const MAX_HEADER: usize = 64 * 1024;
const MAGIC: &[u8; 8] = b"LCRAW001";

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Operation {
    Probe,
    Decode,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Info {
    pub width: usize,
    pub height: usize,
    pub neutral_xy: [f64; 2],
    pub decoder: String,
    pub os_build: String,
    pub recipe: String,
}
impl Info {
    pub fn stamp(&self) -> String {
        format!("{}:{}:{}", self.recipe, self.decoder, self.os_build)
    }
    pub fn validate(&self) -> Result<usize, String> {
        let count = pixel_count(self.width, self.height)?;
        let [x, y] = self.neutral_xy;
        if !x.is_finite() || !y.is_finite() || x <= 0.0 || y <= 0.0 || x + y >= 1.0 {
            return Err("RAW worker returned invalid white chromaticity".into());
        }
        if self.recipe != RECIPE || self.decoder.is_empty() || self.os_build.is_empty() {
            return Err("RAW worker returned incompatible provenance".into());
        }
        Ok(count)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorKind {
    Unsupported,
    Decode,
    Protocol,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BackendError {
    pub kind: ErrorKind,
    pub message: String,
}
impl BackendError {
    pub fn decode(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Decode, message: message.into() }
    }
    pub fn unsupported(message: impl Into<String>) -> Self {
        Self { kind: ErrorKind::Unsupported, message: message.into() }
    }
}
impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for BackendError {}

#[derive(Debug)]
pub struct Decoded {
    pub info: Info,
    pub image: Option<Rgb32f>,
}
#[derive(Serialize, Deserialize)]
struct Request {
    version: u32,
    operation: Operation,
    bytes: usize,
}
#[derive(Serialize, Deserialize)]
struct Response {
    version: u32,
    info: Option<Info>,
    error: Option<BackendError>,
    pixels: usize,
}

pub fn pixel_count(width: usize, height: usize) -> Result<usize, String> {
    width
        .checked_mul(height)
        .filter(|&n| width > 0 && height > 0 && width <= 65536 && height <= 65536 && n <= MAX_PIXELS)
        .ok_or_else(|| "RAW dimensions exceed the 128-megapixel allocation limit".into())
}

fn header_write(w: &mut impl Write, value: &impl Serialize) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?;
    if bytes.len() > MAX_HEADER {
        return Err("RAW protocol header too large".into());
    }
    w.write_all(MAGIC).and_then(|()| w.write_all(&(bytes.len() as u32).to_le_bytes())).and_then(|()| w.write_all(&bytes)).map_err(|e| e.to_string())
}
fn header_read<T: serde::de::DeserializeOwned>(r: &mut impl Read) -> Result<Option<T>, String> {
    let mut magic = [0u8; 8];
    let n = r.read(magic.get_mut(..1).ok_or("header buffer")?).map_err(|e| e.to_string())?;
    if n == 0 {
        return Ok(None);
    }
    r.read_exact(magic.get_mut(1..).ok_or("header buffer")?).map_err(|e| e.to_string())?;
    if &magic != MAGIC {
        return Err("Invalid RAW worker protocol signature".into());
    }
    let mut size = [0u8; 4];
    r.read_exact(&mut size).map_err(|e| e.to_string())?;
    let size = u32::from_le_bytes(size) as usize;
    if size > MAX_HEADER {
        return Err("RAW protocol header too large".into());
    }
    let mut bytes = vec![0; size];
    r.read_exact(&mut bytes).map_err(|e| e.to_string())?;
    serde_json::from_slice(&bytes).map(Some).map_err(|e| format!("Invalid RAW worker header: {e}"))
}
pub fn write_request(w: &mut impl Write, operation: Operation, bytes: &[u8]) -> Result<(), String> {
    if bytes.is_empty() || bytes.len() > MAX_INPUT {
        return Err("RAW input is empty or exceeds 1 GiB".into());
    }
    header_write(w, &Request { version: VERSION, operation, bytes: bytes.len() })?;
    w.write_all(bytes).and_then(|()| w.flush()).map_err(|e| e.to_string())
}
pub fn read_request(r: &mut impl Read) -> Result<Option<(Operation, Vec<u8>)>, String> {
    let Some(request): Option<Request> = header_read(r)? else { return Ok(None) };
    if request.version != VERSION || request.bytes == 0 || request.bytes > MAX_INPUT {
        return Err("Invalid RAW request version or input length".into());
    }
    let mut data = Vec::new();
    data.try_reserve_exact(request.bytes).map_err(|e| e.to_string())?;
    data.resize(request.bytes, 0);
    r.read_exact(&mut data).map_err(|e| e.to_string())?;
    Ok(Some((request.operation, data)))
}
pub fn write_response(w: &mut impl Write, result: Result<Decoded, BackendError>) -> Result<(), String> {
    match result {
        Err(error) => header_write(w, &Response { version: VERSION, info: None, error: Some(error), pixels: 0 })?,
        Ok(decoded) => {
            let n = decoded.info.validate()?;
            if let Some(image) = &decoded.image {
                if image.width != decoded.info.width || image.height != decoded.info.height || image.data.len() != n {
                    return Err("Invalid RAW worker image dimensions".into());
                }
                if image.data.iter().flatten().any(|f| !f.is_finite()) {
                    return Err("Nonfinite RAW pixels".into());
                }
            }
            header_write(
                w,
                &Response { version: VERSION, info: Some(decoded.info), error: None, pixels: if decoded.image.is_some() { n } else { 0 } },
            )?;
            if let Some(image) = decoded.image {
                // Protocol is little-endian regardless of the host.
                if cfg!(target_endian = "little") {
                    w.write_all(bytemuck::cast_slice(&image.data)).map_err(|e| e.to_string())?;
                } else {
                    for f in image.data.iter().flatten() {
                        w.write_all(&f.to_le_bytes()).map_err(|e| e.to_string())?;
                    }
                }
            }
        }
    }
    w.flush().map_err(|e| e.to_string())
}
pub fn read_response(r: &mut impl Read, operation: Operation) -> Result<Result<Decoded, BackendError>, String> {
    let response: Response = header_read(r)?.ok_or("RAW worker exited without a reply")?;
    if response.version != VERSION {
        return Err("RAW worker protocol version mismatch".into());
    }
    if let Some(error) = response.error {
        if response.info.is_some() || response.pixels != 0 {
            return Err("Invalid RAW error response".into());
        }
        return Ok(Err(error));
    }
    let info = response.info.ok_or("RAW response has no metadata")?;
    let count = info.validate()?;
    let expected = if operation == Operation::Decode { count } else { 0 };
    if response.pixels != expected {
        return Err("RAW response pixel count mismatch".into());
    }
    let image = if expected > 0 {
        let mut data = Vec::<[f32; 3]>::new();
        data.try_reserve_exact(count).map_err(|e| e.to_string())?;
        data.resize(count, [0.0; 3]);
        r.read_exact(bytemuck::cast_slice_mut(&mut data)).map_err(|e| e.to_string())?;
        if cfg!(target_endian = "big") {
            for f in data.iter_mut().flatten() {
                *f = f32::from_bits(f.to_bits().swap_bytes());
            }
        }
        if data.iter().flatten().any(|f| !f.is_finite()) {
            return Err("RAW worker returned nonfinite pixels".into());
        }
        Some(Rgb32f { width: info.width, height: info.height, data })
    } else {
        None
    };
    Ok(Ok(Decoded { info, image }))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn oversized_or_truncated_requests_are_rejected() {
        let mut bytes = Vec::new();
        header_write(&mut bytes, &Request { version: VERSION, operation: Operation::Decode, bytes: MAX_INPUT + 1 }).unwrap();
        assert!(read_request(&mut bytes.as_slice()).is_err());
        bytes.clear();
        write_request(&mut bytes, Operation::Probe, b"raw").unwrap();
        bytes.pop();
        assert!(read_request(&mut bytes.as_slice()).is_err());
        assert!(pixel_count(usize::MAX, 2).is_err());
        assert!(pixel_count(0, 20).is_err());
    }
    #[test]
    fn float_roundtrip_preserves_hdr_and_negative_values() {
        let info = Info { width: 2, height: 1, neutral_xy: [0.3127, 0.329], decoder: "test".into(), os_build: "test".into(), recipe: RECIPE.into() };
        let image = Rgb32f { width: 2, height: 1, data: vec![[4.0, -0.1, 0.25], [0.2, 1.2, 0.1]] };
        let mut bytes = Vec::new();
        write_response(&mut bytes, Ok(Decoded { info: info.clone(), image: Some(image.clone()) })).unwrap();
        let decoded = read_response(&mut bytes.as_slice(), Operation::Decode).unwrap().unwrap();
        assert_eq!(decoded.info, info);
        assert_eq!(decoded.image, Some(image));
        assert!(read_response(&mut bytes.as_slice(), Operation::Probe).is_err());
        bytes.truncate(bytes.len() - 1);
        assert!(read_response(&mut bytes.as_slice(), Operation::Decode).is_err());
    }
    #[test]
    fn malicious_response_cannot_allocate_and_nonfinite_is_rejected() {
        let info =
            Info { width: usize::MAX, height: 2, neutral_xy: [0.3, 0.3], decoder: "test".into(), os_build: "test".into(), recipe: RECIPE.into() };
        let mut bytes = Vec::new();
        header_write(&mut bytes, &Response { version: VERSION, info: Some(info.clone()), error: None, pixels: usize::MAX }).unwrap();
        assert!(read_response(&mut bytes.as_slice(), Operation::Decode).is_err());
        let info = Info { width: 1, height: 1, ..info };
        assert!(write_response(&mut Vec::new(), Ok(Decoded { info, image: Some(Rgb32f::filled(1, 1, [f32::NAN; 3])) })).is_err());
    }
}
