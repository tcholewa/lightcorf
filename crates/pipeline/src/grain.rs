// Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md.
//! Original, bounded sparse-convolution grain. No scanned textures or external implementation.
//!
//! Two populations of soft clumps live in oriented image coordinates. Pixel footprints widen
//! their kernels and reduce unresolved contrast. The same hash and equations live in WGSL.
use lightcraft_develop::{DevelopSettings, Treatment};

#[derive(Clone, Copy, Debug)]
pub struct AnalogGrain {
    pub amount: f32,
    pub scale: f32,
    pub roughness: f32,
    pub seed: u32,
    pub colour: f32,
    pub footprint: f32,
    pub softness: f32,
}

fn unit(x: f64) -> f32 {
    if x.is_finite() { (x.clamp(0.0, 100.0) / 100.0) as f32 } else { 0.0 }
}

impl AnalogGrain {
    pub fn new(s: &DevelopSettings, pixels_per_long: f64) -> Option<Self> {
        let amount = unit(s.grain.amount);
        if !s.grain.advanced || !s.section_enabled("effects") || amount == 0.0 {
            return None;
        }
        let size = unit(s.grain.size);
        let scale = 1.0 / (0.0004 + 0.0012 * size);
        let ppl = if pixels_per_long.is_finite() { pixels_per_long.max(1.0) as f32 } else { 1.0 };
        Some(Self {
            amount,
            scale,
            roughness: unit(s.grain.roughness),
            seed: s.grain.seed,
            colour: if s.treatment == Treatment::Bw { 0.0 } else { 0.16 },
            footprint: (scale / ppl).min(256.0),
            softness: amount * (0.25 + 0.35 * size),
        })
    }

    /// The finite blur support is also used to pad windowed renders.
    pub fn sigma(self, pixels_per_long: f64) -> f32 {
        ((pixels_per_long.max(1.0) as f32 / self.scale) * 0.35).clamp(0.01, 64.0)
    }

    pub fn apply(self, rgb: [f32; 3], x: f32, y: f32) -> [f32; 3] {
        let a = clumps(x * self.scale, y * self.scale, self.seed, self.roughness, self.footprint, self.colour);
        let b = clumps(x * self.scale * 0.53, y * self.scale * 0.53, self.seed ^ 0x735a, self.roughness, self.footprint * 0.53, self.colour);
        let lum = (0.2627 * rgb[0] + 0.6780 * rgb[1] + 0.0593 * rgb[2]).clamp(0.0, 1.0);
        let weight = self.amount * 0.10 * (1.25 - 0.5 * lum);
        std::array::from_fn(|k| {
            let v = rgb[k].clamp(0.0, 1.0);
            let noise = (a[k] + b[k] * self.roughness * 0.65).clamp(-1.0, 1.0);
            // Zero at the endpoints and above SDR white; HDR headroom passes through untouched.
            rgb[k] + noise * weight * 4.0 * v * (1.0 - v)
        })
    }
}

#[inline]
fn hash(x: i32, y: i32, seed: u32) -> u32 {
    let mut v = (x as u32).wrapping_mul(0x8da6_b343) ^ (y as u32).wrapping_mul(0xd816_3841) ^ seed.wrapping_mul(0xcb1a_b31f);
    v ^= v >> 13;
    v = v.wrapping_mul(0x5bd1_e995);
    v ^ (v >> 15)
}

#[inline]
fn signed(v: u32) -> f32 {
    (v & 0xffff) as f32 / 32768.0 - 1.0
}

fn clumps(x: f32, y: f32, seed: u32, rough: f32, footprint: f32, colour: f32) -> [f32; 3] {
    if !(x.is_finite() && y.is_finite()) {
        return [0.0; 3];
    }
    // Bounds make the float-to-cell conversion and its fixed neighbours safe even under bad geometry.
    let (x, y) = (x.clamp(-1e6, 1e6), y.clamp(-1e6, 1e6));
    let (ix, iy) = (x.floor() as i32, y.floor() as i32);
    let (fx, fy) = (x - x.floor(), y - y.floor());
    let mut out = [0.0; 3];
    for dy in -1i32..=1 {
        for dx in -1i32..=1 {
            let (cx, cy) = (ix.wrapping_add(dx), iy.wrapping_add(dy));
            let h = hash(cx, cy, seed);
            let px = dx as f32 + (h & 1023) as f32 / 1024.0;
            let py = dy as f32 + ((h >> 10) & 1023) as f32 / 1024.0;
            let r = 0.62 + 0.22 * rough * signed(hash(cx, cy, seed ^ 0x17ab));
            let r2 = r * r;
            let filtered = (r2 + footprint * footprint / 6.0).min(0.9801);
            let dist = (fx - px) * (fx - px) + (fy - py) * (fy - py);
            let t = (1.0 - dist / filtered).max(0.0);
            let k = t * t * t * (r2 / filtered) * 2.8 / (1.0 + footprint * footprint).sqrt();
            let base = signed(hash(cx, cy, seed ^ 0x93d1));
            let chroma = [signed(h.rotate_left(7)), signed(h.rotate_left(17)), signed(h.rotate_left(27))];
            for (o, ch) in out.iter_mut().zip(chroma) {
                *o += k * (base + colour * ch);
            }
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PixelWindow, RenderRequest, SourceInfo, StageCache, render, render_cached};
    use lightcraft_develop::grain::PRESETS;
    use lightcraft_raster::Rgb32f;
    use std::sync::Arc;

    fn settings() -> DevelopSettings {
        let mut s = DevelopSettings::default();
        PRESETS[2].apply(&mut s.grain);
        s.grain.advanced = true;
        s
    }

    #[test]
    fn grain_statistics_endpoints_and_hostile_values() {
        let mut previous = 0.0;
        for p in PRESETS {
            let mut s = settings();
            p.apply(&mut s.grain);
            let g = AnalogGrain::new(&s, 6000.0).unwrap();
            let mut sum = 0.0;
            let mut energy = 0.0;
            let mut colour = 0.0;
            for y in 0..128 {
                for x in 0..128 {
                    let q = g.apply([0.18; 3], x as f32 / 6000.0, y as f32 / 6000.0);
                    let d = q[0] - 0.18;
                    sum += d;
                    energy += d * d;
                    colour += (q[0] - q[1]).abs();
                }
            }
            let rms = (energy / 16384.0).sqrt();
            assert!(rms > previous, "ISO {} rms {rms}, preceding {previous}", p.iso);
            assert!((sum / 16384.0).abs() < 0.0006, "mean shifted");
            assert!(colour > 0.1);
            previous = rms;
            for rgb in [[0.0; 3], [1.0; 3], [4.0; 3]] {
                assert_eq!(g.apply(rgb, 0.123, 0.456), rgb);
            }
            let mono = AnalogGrain { colour: 0.0, ..g };
            let q = mono.apply([0.18; 3], 0.0011, 0.0021);
            assert_eq!(q[0], q[1]);
            assert_eq!(q[1], q[2]);
            assert!(g.apply([0.18; 3], f32::MAX, f32::MIN).iter().all(|v| v.is_finite()));
        }
        for x in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY, -1.0, 1e300] {
            let mut s = settings();
            s.grain.amount = x;
            s.grain.size = x;
            s.grain.roughness = x;
            if let Some(g) = AnalogGrain::new(&s, 100.0) {
                assert!(g.apply([0.18; 3], 0.01, 0.02).iter().all(|v| v.is_finite()));
            }
        }
    }

    #[test]
    fn bypass_cache_reopen_and_window_match() {
        let src = Arc::new(Rgb32f::from_fn(800, 600, |x, y| [0.12 + ((x * 17 + y * 13) % 23) as f32 * 0.002; 3]));
        let info = SourceInfo::default();
        let req = RenderRequest::fit(800, 600);
        let cache = StageCache::default();
        let mut s = settings();
        let full = render(&src, &info, &s, &req).image;
        assert_eq!(full.data, render_cached(&src, &info, &s, &req, &cache).image.data);
        let reopened = DevelopSettings::from_json(&s.to_json()).unwrap();
        assert_eq!(full.data, render(&src, &info, &reopened, &req).image.data);
        let win = PixelWindow { x: 121, y: 93, w: 177, h: 109 };
        let window = render(&src, &info, &s, &RenderRequest { window: Some(win), ..req }).image;
        let cut = full.crop(win.x, win.y, win.w, win.h);
        let max = cut.data.iter().zip(&window.data).flat_map(|(a, b)| (0..3).map(move |k| a[k].abs_diff(b[k]))).max().unwrap();
        assert!(max <= 1, "window seam: {max}");
        for change in [0, 1, 2, 3] {
            match change {
                0 => s.grain.size = 70.0,
                1 => s.grain.amount = 10.0,
                2 => s.grain.advanced = false,
                _ => s.grain.advanced = true,
            }
            assert_eq!(render(&src, &info, &s, &req).image.data, render_cached(&src, &info, &s, &req, &cache).image.data);
        }
        s.grain.amount = 0.0;
        let zero = render(&src, &info, &s, &req).image;
        s.grain.advanced = false;
        assert_eq!(zero.data, render(&src, &info, &s, &req).image.data);
        s.grain.advanced = true;
        s.grain.amount = 50.0;
        s.set_section_enabled("effects", false);
        assert_eq!(zero.data, render(&src, &info, &s, &req).image.data);
    }

    #[test]
    fn deep_windows_geometry_and_hdr_headroom() {
        use crate::{DeepSamples, OutputDepth, OutputSpace};
        let src = Rgb32f::from_fn(1920, 1280, |x, y| [0.15 + ((x * 17 + y * 13) % 23) as f32 * 0.002; 3]);
        let info = SourceInfo::default();
        let mut s = settings();
        s.crop.geometry.rect = lightcraft_geom::Rect { x0: 0.1, y0: 0.15, x1: 0.85, y1: 0.9 };
        s.crop.geometry.angle = 4.0;
        s.crop.flip_h = true;
        let win = PixelWindow { x: 231, y: 193, w: 177, h: 109 };
        for depth in [OutputDepth::U16, OutputDepth::F32Linear, OutputDepth::F32Hdr] {
            let req = RenderRequest { depth, space: OutputSpace::Rec2020, ..RenderRequest::fit(1920, 1280) };
            let full = render(&src, &info, &s, &req).deep.unwrap().crop(win);
            let part = render(&src, &info, &s, &RenderRequest { window: Some(win), ..req }).deep.unwrap();
            assert_eq!((part.width, part.height), (win.w, win.h));
            match (&full.samples, &part.samples) {
                (DeepSamples::U16(a), DeepSamples::U16(b)) => {
                    assert_eq!(a.len(), b.len());
                    assert!(a.iter().zip(b).all(|(x, y)| x.abs_diff(*y) <= 3));
                }
                (DeepSamples::F32(a), DeepSamples::F32(b)) => {
                    assert_eq!(a.len(), b.len());
                    assert!(a.iter().zip(b).all(|(x, y)| x.is_finite() && y.is_finite() && (x - y).abs() < 0.0001));
                }
                _ => panic!("sample format changed"),
            }
        }
        let src = Rgb32f::from_fn(32, 24, |_, _| [4.0; 3]);
        s.hdr.enabled = true;
        let req = RenderRequest { depth: OutputDepth::F32Hdr, ..RenderRequest::fit(32, 24) };
        let DeepSamples::F32(v) = render(&src, &info, &s, &req).deep.unwrap().samples else { panic!("float HDR") };
        assert!(v.iter().all(|x| x.is_finite() && *x > 1.0));
    }

    #[test]
    fn softness_reduces_fine_detail_and_grain_follows_saturation() {
        let src = Rgb32f::from_fn(1920, 1280, |x, y| [if (x + y) % 2 == 0 { 0.17 } else { 0.19 }; 3]);
        let info = SourceInfo::default();
        let req = RenderRequest::fit(1920, 1280);
        let s = settings();
        let mut no_soft = s.clone();
        no_soft.grain.amount = 0.0;
        let a = render(&src, &info, &s, &req).image;
        let b = render(&src, &info, &no_soft, &req).image;
        // Project onto the known checkerboard: random grain must not be mistaken for
        // retained image detail by an absolute-gradient measurement.
        let contrast = |im: &lightcraft_raster::Rgba8| -> f64 {
            im.data
                .iter()
                .enumerate()
                .map(|(i, p)| f64::from(p[0]) * if (i % im.width + i / im.width).is_multiple_of(2) { -1.0 } else { 1.0 })
                .sum::<f64>()
                / im.len() as f64
        };
        assert!(contrast(&a) < contrast(&b), "detail {} >= {}", contrast(&a), contrast(&b));
        let mut s = s;
        s.color.saturation = -100.0;
        let gray = render(&src, &info, &s, &req).image;
        assert!(gray.data.iter().all(|p| p[0].abs_diff(p[1]) <= 1 && p[1].abs_diff(p[2]) <= 1));
        for (w, h) in [(1, 1), (1, 9), (9, 1)] {
            let src = Rgb32f::from_fn(w, h, |_, _| [0.18; 3]);
            let r = render(&src, &info, &s, &RenderRequest::fit(w, h));
            assert_eq!(r.image.data.len(), w * h);
        }
    }
}
