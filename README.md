<!-- Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md. -->
# Lightcorf

An independent BW-focused fork of [LightCraft](https://github.com/storytold/lightcraft),
originally developed by the ArtCraft team and LightCraft contributors. This fork is maintained
by [@tcholewa](https://github.com/tcholewa); it is not an official ArtCraft build or endorsed by ArtCraft.

[简体中文](README.zh-hans.md) · [繁體中文](README.zh-hant.md)

## What this fork adds

- Olympus / OM System ORF development, with original pure-Rust lossless decoding.
- An original **Tri-X 400 — Push +2 (Inspired)** preset and optional advanced grain, including
  ISO-inspired 400/800/1600 starting points. These are creative interpretations, not measured film-stock matches.
- On macOS, a supervised Apple Core Image RAW worker alongside the LightCraft Native Rust decoder.
  Choose the processor under **Settings → Performance → RAW processing**, then restart.
- Sharpening **15** for new RAW imports on macOS. Existing edits and Reset baselines are preserved.

The publication baseline is verified source `377095c2`, built on upstream `31ee2643`.
The default branch is `lightcorf`; the fork's `main` branch is the upstream snapshot created by GitHub.
Local development history and original authorship are retained. Publication changes cover branding,
attribution and project links, without changing the RAW algorithms.

This is an alpha application. See [Core Image behavior and limits](docs/coreimage-raw.md),
[ORF support](docs/orf.md), [grain and Tri-X](docs/analog-grain.md), and the [roadmap](ROADMAP.md).
Core Image qualification covers supplied OM-1, E-M1X, original E-M1 and GX9 samples; an original
E-M10 sample is still missing. Other platforms retain native Rust RAW processing.

## Build from source

This first publication provides **source only**, with no downloadable Lightcorf app release.
Upstream LightCraft downloads do not contain these fork modifications.

Use Rust 1.95 (the locally verified toolchain; the manifest declares a minimum of 1.90), then:

```sh
git clone --branch lightcorf https://github.com/tcholewa/lightcorf.git
cd lightcorf
git clone https://github.com/storytold/craft-fonts.git craft-fonts
git -C craft-fonts checkout 3d9ab3a132ed0eff457e424226a435a901da30d3
CRAFT_FONTS_DIR="$PWD/craft-fonts" cargo build --release --locked \
  -p lightcraft -p lightcraft-cli -p lightcraft-raw-worker \
  --features lightcraft/heif,lightcraft-cli/heif
./target/release/lightcraft
```

On macOS, Core Image requires macOS 12 or newer and the `lightcraft-raw-worker` executable beside
the app executable. Build all three binaries above. Apple framework implementations are supplied
by macOS, not this repository. Other platforms do not use Core Image.

Fonts are optional for development; the pinned build input above provides additional scripts and
must accompany any future release with its OFL notices. Platform packaging details remain in
[the packaging documentation](docs/releasing.md). Automated release publishing is disabled for this fork.

The local macOS app is named **Lightcorf.app**, with Lightcorf window and menu labels.
For compatibility, the macOS bundle identifier remains `com.diffico.lightcraft.bw.coreimage`, with the existing
LightCraft BW Core Image configuration and library paths. Internal crate names, CLI names and command
IDs remain stable. Catalog format 10 cannot be opened by older builds. Use a separate test library
when evaluating the app; [local deployment records](docs/local-deployment.md) describe prior testing.

## Development and support

Report fork issues at [Lightcorf issues](https://github.com/tcholewa/lightcorf/issues).
For upstream work, see the [original LightCraft repository](https://github.com/storytold/lightcraft).

```sh
CRAFT_FONTS_DIR="$PWD/craft-fonts" cargo xtask ci
./target/release/lightcraft --control 7980
```

The quality gate checks formatting, clippy, HEIF, tests, feature references, layering, asset
attribution and WASM. [AGENTS.md](AGENTS.md) contains the development agreements.
[Control protocol](docs/control-protocol.md) and [MCP](docs/mcp.md) document automation.
Do not commit private RAWs, libraries, model weights or build output.

## Licence and credits

The workspace and original fork additions retain **MIT OR Apache-2.0**, at your option:
[MIT](LICENSE-MIT), [Apache-2.0](LICENSE-APACHE).
Copyright (c) 2026 ArtCraft Team and the LightCraft contributors. Original copyright and
attribution notices are retained in [NOTICE](NOTICE) and the source history.

This is not a blanket licence for every component: `crates/segment` remains **Apache-2.0 only**;
fonts, photographs, icons and data retain their own terms in [assets/ATTRIBUTION.md](assets/ATTRIBUTION.md).
Optional model weights are not included and retain their separate licences. No Apple framework
code or GPL RAW decoder code is distributed by this fork.

Restricted ArtCraft logos/marks have been removed from the publication working tree; their
[original terms](docs/brand/LICENSE-brand.txt) remain for historical attribution. The separately
MIT/Apache-licensed LightCraft application icon is retained unchanged. Tri-X is referenced only
to describe the inspired look; no affiliation with its owner is claimed.

See the [fork licence audit](docs/fork-license-audit.md) for the checked baseline, dependency
licences, provenance evidence and limits of the review.
