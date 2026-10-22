// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! Generate local grain comparison evidence; images stay outside tracked source.
//! cargo run --release -p lightcraft-engine --example grain_compare -- target/analog-grain [photo]
#![forbid(unsafe_code)]
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]
use lightcraft_develop::{DevelopSettings, Treatment, grain::PRESETS};
use lightcraft_pipeline::{PixelWindow, RenderRequest, SourceInfo, render};
use lightcraft_raster::Rgb32f;
use std::{path::PathBuf, sync::Arc};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let out = PathBuf::from(args.next().ok_or("give output directory")?);
    std::fs::create_dir_all(&out)?;
    let (src, info) = if let Some(path) = args.next() {
        lightcraft_engine::files::load_bytes(&std::fs::read(path)?, usize::MAX)?
    } else {
        (
            Rgb32f::from_fn(6000, 4000, |x, y| {
                if y < 1600 {
                    [x as f32 / 5999.0; 3]
                } else if y < 2800 {
                    let base = match x / 1500 {
                        0 => [0.43, 0.25, 0.16],
                        1 => [0.05, 0.15, 0.32],
                        2 => [0.06, 0.22, 0.08],
                        _ => [0.18; 3],
                    };
                    base.map(|v| v + if (x + y).is_multiple_of(2) { 0.01 } else { -0.01 })
                } else {
                    [if x % 24 < 12 { 0.1 } else { 0.8 }; 3]
                }
            }),
            SourceInfo::default(),
        )
    };
    let src = Arc::new(src);
    let gpu = lightcraft_engine::gpu::available();
    println!("GPU: {:?}; reason: {:?}", lightcraft_engine::gpu::adapter_name(), lightcraft_engine::gpu::unavailable_reason());
    for bw in [false, true] {
        for advanced in [false, true] {
            for preset in PRESETS {
                let mut s = DevelopSettings { treatment: if bw { Treatment::Bw } else { Treatment::Color }, ..Default::default() };
                preset.apply(&mut s.grain);
                s.grain.advanced = advanced;
                let prefix = format!("{}-{}-{}", if bw { "bw" } else { "colour" }, if advanced { "advanced" } else { "legacy" }, preset.iso);
                let side = 720.min(src.width).min(src.height);
                for (name, req) in [
                    ("fit", RenderRequest::fit(1600, 1200)),
                    (
                        "detail",
                        RenderRequest {
                            window: Some(PixelWindow { x: (src.width - side) / 2, y: (src.height - side) / 2, w: side, h: side }),
                            ..RenderRequest::fit(src.width, src.height)
                        },
                    ),
                ] {
                    let result = if gpu {
                        lightcraft_engine::gpu::render(&src, &info, &s, &req, None).ok_or("GPU comparison render fell back")?
                    } else {
                        render(&src, &info, &s, &req)
                    };
                    let bytes = lightcraft_codecs::encode_png(&lightcraft_codecs::EncodeImage::rgba8(&result.image), &Default::default())?;
                    let path = out.join(format!("{prefix}-{name}.png"));
                    std::fs::write(&path, bytes)?;
                    println!("{}", path.display());
                }
            }
        }
    }
    Ok(())
}
