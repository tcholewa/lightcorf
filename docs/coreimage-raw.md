# Lightcraft BW Core Image (experimental macOS branch)

Last updated: 2026-10-21. Task: CI.1. Branch: `feature/macos-coreimage-raw`.

This branch replaces the macOS RAW development path with Apple's CIRAWFilter (macOS 12+).
Windows, Linux and WASM retain the independent Rust decoder. This is a deliberate system-framework
exception to the project's pure-Rust implementation policy; the bindings and worker are Rust source.
There is no runtime Rust sensor fallback on macOS. Camera support and rendering depend on the installed
Apple RAW decoder and OS build; old and new backends are not expected to produce identical images.

## Isolation and interfaces

`raw-backend` (L1) implements a safe protocol and supervised client; `raw-coreimage` (L1) contains the
native FFI, and only `apps/raw-worker` links it. Engine, desktop and CLI communicate with the worker
through pipes. A persistent worker reuses one float CIContext, processes one request at a time, and
never passes native objects to the parent. The protocol carries bounded JSON headers and little-endian
RGB32f payloads: maximum input 1 GiB, maximum output 128 megapixels, dimensions at most 65536 each.
Allocations and multiplication are checked; nonfinite pixels and inconsistent replies are rejected.
The client API's cancellation token or a 120-second request deadline kills the worker. Existing
file-loader callers use the deadline; cancelling a UI task during its file load can wait for that
decode to finish before the task acknowledges cancellation. A later request starts a new worker;
the failed input is never retried automatically. A missing helper or graphics-session access produces
an actionable error, not a Rust fallback. `LIGHTCRAFT_RAW_WORKER` can locate the matching helper in a
developer build. Normally it lives beside the executable, including inside `Contents/MacOS`.

Recipe `coreimage-linear-rec2020-v1`: full-resolution Apple development at as-shot WB, exposure,
baseline exposure, shadow bias and boost zero; draft, optional noise reduction/detail/sharpening,
local tone, gamut mapping, lens correction and optional highlight recovery disabled. Extended dynamic
range amount is 1; output is extended linear Rec.2020 with finite negative values and values above 1
preserved. Core Image applies orientation once; a top-down bitmap enters LightCraft without a second
EXIF transform. Smaller sources use the same full decode followed by LightCraft box resampling.

Apple supplies camera colour and demosaic. LightCraft keeps its WB adjustment, tone, RGB detail,
Tri-X 400/800/1600, Amount, grain and export pipeline. The as-shot neutral xy is converted into
LightCraft's Kelvin/tint scale. The old camera fit, matrices, profile tone and sensor highlight
reconstruction are bypassed. Embedded sensor lens opcodes/mattes are not transferred across Apple's
crop geometry in this first version; ordinary manual geometry/optics and RGB masks remain available.

`Photo.raw_backend`, `ProbeInfo.raw_backend` and `SourceInfo.raw_backend` carry provenance; catalog
fields are optional for old records. `photo.inspect` includes backend capabilities. RAW cache keys
include recipe, decoder and OS build. On an OS/recipe change RAW records are re-probed, including
previously supported records; settings/history are kept. Reopening an old library in this branch can
change its RAW appearance: use the isolated library, not the existing BW library.

## Capabilities

- Import, previews, exports, CLI/MCP and RAW HDR/panorama inputs use Apple RGB. Merge clipping uses a conservative observed RGB ceiling because sensor saturation is unavailable; values above 1 are not automatically treated as clipped.
- Unsupported RAWs can display labelled embedded previews; this is not RAW qualification.
- AI RAW denoise, sensor-DNG conversion/export and camera calibration cannot operate on mosaics here
  and are disabled with explanations. Original copying and rendered image exports remain available.
- Rendered/merged DNGs are decoded by Apple before saving; an unsupported result fails before the file
  is written. On the tested OS, Apple returns a zero-sized output for LightCraft linear DNGs: final
  HDR/panorama merges and rendered DNG export therefore fail with an explanation. JPEG/PNG/TIFF
  exports work. No Rust fallback is used to hide this incompatibility.
- No automatic Apple decoder-resource downloads. macOS updates can change the result even when a
  decoder's public version string stays the same; its OS build remains part of cache identity.
  If decoder resources change without an OS build change, use Reload from Disk to refresh provenance.

## Local acceptance corpus

Personal `_orfs/` samples are ignored, read-only test inputs, never committed or redistributed.
Checksums and output evidence live under `target/coreimage-raw/`.

| Camera | Files | Apple-oriented dimensions |
|---|---|---|
| Original E-M1 | `B6010001.ORF`, `P6130523.ORF` | 4608×3456; 3456×4608 |
| OM-1 | `_7190788.ORF`, `_C031132.ORF`, `_C031132.ORI` | 3888×5184; 8160×6120; 5184×3888 |
| E-M1X | `_X220064.ORF` | 3888×5184 |
| Panasonic DC-GX9 | `_1014216.RW2` | 5184×3888 |
| Original E-M10 | Fixture missing | Pending; no verified-support claim |

All seven provided files completed full float decoding on macOS 27.0.1 build 26A434, Apple decoder 8.
Native debug-app acceptance passed all seven imports and Tri-X JPEG exports, plus OM-1 preset
hover/apply, grain controls, Amount 0/49/50/100/200, undo/redo, PNG export and exact catalog reopen.
Fit and 100% native screenshots were inspected; Metal was active without fallback. Release acceptance
and the full quality-gate result are recorded in the local deployment document. E-PL7 is not an
acceptance target. The unchanged Rust sensor regression suite remains useful for the other platforms.

## Build, tests and delivery

Build the app/CLI **and** helper, with release fonts and optional HEIF as usual:

```sh
cargo build --release -p lightcraft -p lightcraft-cli -p lightcraft-raw-worker \
  --features lightcraft/heif,lightcraft-cli/heif
cargo test -p lightcraft-engine --test coreimage
cargo xtask ci
```

The existing engine unit fixtures intentionally exercise the Rust reference implementation on every
host. The new macOS integration tests link the normal production engine (strict Apple policy), exercise
the personal samples when present, and verify full/preview equivalence, orientation/dimensions,
provenance, HDR range and sensor capability rejection. Worker protocol tests cover malformed/truncated
packets, missing/crashed/hung helpers, restart, cancellation, allocation limits and float preservation.
All eight `cargo xtask ci` gates passed on 2026-10-21 (fmt, clippy, HEIF, tests, parity, layers,
assets and WASM). There were 22 explicit optional-data skips, including the missing original E-M10;
none of the seven supplied-file or GPU acceptance checks skipped.
Native graphics tests need a normal macOS graphics session; the build sandbox may deny CIContext creation.

The bundle is `Lightcraft BW Core Image.app`, identifier `com.diffico.lightcraft.bw.coreimage`.
Defaults are `~/Pictures/LightCraft BW Core Image Library` and
`~/Library/Application Support/LightCraft BW Core Image`. Explicit command-line/environment overrides
still work. Installation is side by side at `~/Applications/Lightcraft BW Core Image.app`; the existing
BW app, configuration and library must remain unchanged.

API references: Apple documents [baseline exposure zero as linear response](https://developer.apple.com/documentation/coreimage/cirawfilter/baselineexposure) and [EDR amount 1 as its default EDR](https://developer.apple.com/documentation/coreimage/cirawfilter/extendeddynamicrangeamount).

## Revision history

| Date | Task | Change |
|---|---|---|
| 2026-10-21 | CI.1 | Strict isolated Apple RAW backend, local camera corpus and separate macOS identity |
