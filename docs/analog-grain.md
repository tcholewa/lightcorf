# ISO-inspired grain

> **Last updated:** 2026-10-12 · **Change:** major (FILM.1: Tri-X-inspired preset)

Effects → Grain offers original 35mm-inspired **400**, **800**, and **1600** starting points.
They suggest increasingly pronounced grain, not measured reproductions of a particular stock or
changes to the photo's recorded ISO. Amount, Size and Roughness remain editable.

| Profile | Amount | Size | Roughness |
|---|---:|---:|---:|
| 400 | 20 | 20 | 40 |
| 800 | 35 | 35 | 55 |
| 1600 | 50 | 50 | 70 |

A click sets the three sliders in one undo step. It preserves the selected renderer and seed.
Hover only explains the profiles; it does not change the image. A matching profile is highlighted;
custom values leave all three unselected. The original slider defaults remain 0 / 25 / 50.

**Advanced grain** is independent of the profiles and defaults off. It adds irregular soft clumps,
restrained colour variation on colour photos, and monochrome texture on B&W treatments/profiles.
Visibility varies smoothly across tones, with quieter highlights and clean black/white endpoints.
Coupled softening reduces fine luminance detail without changing the Detail sliders. Amount 0 or
the Effects eye disables both effects. Reset Grain also turns Advanced off.

## Tri-X-inspired image preset

Presets → **B&W → Tri-X 400 — Push +2 (Inspired)** is an original full-image look,
not a measured reproduction of Kodak film, a developer or a scanner. It uses the existing
Advanced renderer with pronounced grain, neutral B&W and deliberately crushed blacks.
The stock-inspired name and Push +2 describe the look: neither recorded ISO nor Exposure changes.

| Component | Settings at 100% |
|---|---|
| Profile / treatment | Color (`lc.color`), profile amount 100; B&W |
| Light | Contrast +45, Highlights −20, Shadows −25, Whites +10, Blacks −50 |
| Parametric curve | Shadows −60, Darks −25, Lights +10, Highlights −10 |
| B&W mix | Red +10, Orange +8, Yellow 0, Green −5, Aqua −10, Blue −15, Purple −5, Magenta 0 |
| Effects | Texture +5, Clarity +15, Dehaze 0 |
| Grain | Advanced on; Amount 50, Size 50, Roughness 70 |

The preset replaces the creative look: previous saturation/vibrance, HSL mixer, Point Color,
colour grading and master/RGB point curves reset to neutral; curve splits and Refine Saturation
return to defaults. Exposure, WB, camera calibration/local tone, HDR, Detail, vignette,
optics/geometry/crop, local edits, grain seed and process version stay unchanged. Disabled section
eyes stay disabled, so they can suppress parts of the look, including grain.

Use `preset.apply {"id":"lc.bw-trix400-push2","amount":100}` through the existing command
interface, or `lightcraft-cli render input.jpg -o output.png --preset lc.bw-trix400-push2`.
Hover preview, batch application, undo/redo, persistence and the 0–200% Amount slider follow other
built-ins. At 0% the original edit is retained; numeric settings interpolate from that edit,
while discrete values (including B&W, profile, point-curve arrays and Advanced) switch at 50%.
The 400/800/1600 Grain buttons still change only three grain sliders and preserve the renderer.

FILM.1 validation covers replacement/preservation, Amount 0/49/50/100/200%, batch undo/redo,
neutral output and CPU/Metal windows. Native control-channel acceptance verifies hover, applying,
undo/redo, PNG/JPEG export and exact catalog reopen in a scratch library. Fit and 100% captures
were inspected. CPU/Metal comparisons on an original chart and the CC0 Sony A7 III / E-PL7 raws
have maximum difference 1 level in 8-bit RGB (mean below 0.001); full-size exports were inspected.
Full `cargo +1.95.0 xtask ci` passes all eight gates. Local evidence and scripts live in
`target/tri-x-inspired/`; generated media stays untracked.

No new grain model, schema or process version is introduced. Grain softening remains coupled to
Amount and Size. The starting recipe was inspected on the CC0 Sony A7 III scene; that artistic
check does not establish film-stock fidelity. Kodak's
[Tri-X 400 description](https://www.kodak.com/en/still-film/product/professional/tri-x-400-film/)
and [technical publication F-4017](https://business.kodakmoments.com/sites/default/files/files/resources/f4017_TriX.pdf)
provide qualitative context for grain, sharpness and push processing; no external preset,
scanned grain texture or proprietary asset is included.

## Rendering

The CPU reference and WGSL use an original bounded sparse-convolution model: two jittered particle
populations, compact smooth kernels, bounded radius variation, and a repeatable integer hash.
Roughness changes radius variation and the contribution of coarser clumps. A common luminance
component dominates small decorrelated colour components. It is a perceptual approximation, not
an emulsion/chemical simulation or calibrated model of any film brand.

Particles live in normalized oriented-image coordinates, not window pixels. Their kernels widen
and their contrast decreases with pixel footprint so unresolved grains do not become full-strength
pixel noise. The renderer uses the existing stored seed; it never rolls a new pattern while panning,
reopening or exporting. Different photos with the same seed share the same coordinate field.

A separately cached Gaussian approximation of log luminance supplies fine-detail attenuation.
Its radius follows grain size, its strength follows Amount, and a contrast gate protects larger
edges. CPU and GPU caches account for the additional plane; zoom windows include its finite blur
support. This is deliberate subtle softening, not a claim that all detail below a grain is removed.

Advanced grain enters linear working RGB after the tone map and before colour processing. Colour
mixing, saturation, B&W conversion, grading and subsequent curves therefore also affect grain.
The signed perturbation has zero expected mean and vanishes at the channel endpoints. Values above
SDR white retain HDR headroom. Existing unchecked grain stays at its original finishing stage,
with unchanged equations and no additional blur work. Web and GPU fallback use the CPU model.

## Commands and saved edits

- `develop.grainPreset {"iso": 400}` (also 800 / 1600): active photo, following Auto Sync.
- `develop.merge {"settings":{"grain":{"advanced":true}},"label":"Advanced grain"}`:
  toggle without changing sliders or seed.
- `develop.resetSection {"section":"grain"}`: original numeric defaults, Advanced off, seed kept.
- Automation IDs: `button:grainPreset-400`, `button:grainPreset-800`,
  `button:grainPreset-1600`, `checkbox:grainAdvanced`.

`grain.advanced` is a boolean, missing = false. False is omitted from persisted JSON so legacy
settings hashes remain unchanged. Grain group extraction and Auto Sync explicitly carry false
when needed, so copying standard grain also disables Advanced. Older partial presets containing
only numeric grain controls preserve the current mode. The mode survives native presets,
copy/paste, history, versions, catalogs and LightCraft XMP settings. There is no new Lightroom CRS
mapping. Old LightCraft binaries ignore the new field and cannot reproduce Advanced grain.
No rendering-process upgrade is necessary: existing edits keep their old renderer.

## Evidence and reproduction

Use the existing CC0 corpus and generated charts, not Adobe textures or film presets. The guide's
principles are artistic targets; ISO, film format, development and scanning all affect real grain.

```sh
cargo run --release -p lightcraft-engine --example grain_compare -- target/analog-grain/charts
cargo run --release -p lightcraft-engine --example grain_compare -- target/analog-grain/epl7 corpus/raw/orf-olympus-epl7.orf
LIGHTCRAFT_BENCH_GRAIN=advanced cargo run --release -p lightcraft-engine --example render_bench -- corpus/raw/arw-sony-a7m3-compressed.arw
```

The comparison tool emits colour/B&W, standard/advanced, 400/800/1600 at fit size and 100% detail.
`LIGHTCRAFT_BENCH_GRAIN=legacy` benchmarks the same ISO-1600 settings using standard grain; unset
benchmarks the existing typical edit. GPU availability is printed: a skipped GPU test is not GPU
evidence. Test tolerances remain mean <0.5 / max ≤3 levels in 8-bit CPU/GPU comparison.
Generated images, installation manifests and detailed timings stay in gitignored `target/analog-grain/`.

Native control-channel acceptance verifies button clicks, hover stability, independent mode,
undo/redo, custom slider values, reset, JPEG export and catalog reopen on an E-PL7 virtual copy.
The original photo settings stay unchanged. A standard ISO-1600 E-PL7 PNG export is byte-for-byte
identical to the previously installed ORF build. Grain panel captures are kept with local evidence.

Full `cargo +1.95.0 xtask ci` passes all eight gates, including WebAssembly checks.

### Local measurements

Apple M1 Max, 24 MP Sony A7 III CC0 raw, release build, minimum of five runs, ISO 1600.
The machine was also compiling, so these are local observations rather than portable guarantees.

| Operation | Standard GPU | Advanced GPU | Advanced CPU |
|---|---:|---:|---:|
| Warm 1920×1280 exposure edit | 3.8 ms | 6.1 ms | 104.1 ms |
| Warm draft highlights edit | 3.0 ms | 3.6 ms | 38.4 ms |
| Cold 1920×1280 render | 45.4 ms | 48.6 ms | 190.0 ms |
| 6000×4000 export render | 518.3 ms | 552.1 ms | 1644.1 ms |

Advanced JPEG encoding added 62.4 ms. GPU interactive/export targets are met in this run;
CPU-only advanced rendering exceeds the interactive budget. Opt-in preserves the standard
renderer for machines where that cost is undesirable. No approximation switches between
preview and export. All 17 Metal equivalence tests pass. CPU/Metal advanced comparisons are within 1 level in 8-bit RGB, including
colour/B&W, masks and windows. The comparisons include generated ramps/patches and the E-PL7
fixture; they do not establish measured real-film or Lightroom fidelity.

## References

- [Film grain](https://en.wikipedia.org/wiki/Film_grain): optical texture, particle diversity and observation scale.
- [Vlad Moldovean's grain deep dive](https://vmoldo.com/grain-deepdive/): grain integration and colour treatment.
- [Film Grain Emulation — 2025 Guide](https://vmoldo.com/film-grain-emulation-2025-guide/): texture, tonal visibility and fine-detail relationships.

No external grain code, scanned overlays, presets or Adobe assets are used.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-12 | major | FILM.1: original Tri-X 400 Push +2 inspired B&W preset; renderer unchanged |
| 2026-10-12 | major | GRAIN.1: original ISO-inspired profiles and opt-in CPU/GPU grain |
