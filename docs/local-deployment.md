# Local macOS installations

> **Last updated:** 2026-10-21 · **Change:** minor (SYNC.2: source updated to upstream 0.10.0; installed BW app remains 0.8.0)


## Lightcraft BW (current desktop installation)

The local 0.8.0 app is `~/Applications/Lightcraft BW.app`, built from the verified
ORF/grain/Tri-X integration `53540224`. Its bundle name and display name are
**Lightcraft BW**, with identifier `ai.storyteller.lightcraft.bw`. This is a local
ARM64 build with HEIF support, Inter and release-pinned craft-fonts
`3d9ab3a132ed0eff457e424226a435a901da30d3`, signed ad hoc for this Mac.

The bundle's `LSEnvironment` deliberately retains the existing paths:

| Data | Path |
|---|---|
| Library | `~/Pictures/LightCraft PR1162 Library` |
| Preferences / logs / models | `~/Library/Application Support/LightCraft PR1162` |

Keeping these paths preserves saved edits and preferences without migrating or
renaming user data. The executable inside the bundle remains `LightCraft`;
upstream product labels remain LightCraft. Launch normally with:

```sh
open "$HOME/Applications/Lightcraft BW.app"
```

The PR1162 app and its `.before-analog-grain` / `.before-tri-x-inspired` app backups
are removed from `~/Applications` after the replacement passes acceptance. Recovery
copies live only in gitignored `target/lightcraft-bw/retired/`. The everyday
`/Applications/LightCraft.app` and existing CLI packages are outside this app update.

Build with the packaging procedure below, then copy its signed app into a separate
stage, set the BW bundle metadata and retained data paths, include Inter's OFL,
and sign the executable and bundle again. Require strict signature and plist checks
before installing. Test staged and installed executables against fresh scratch
libraries/configurations through the control channel: E-PL7 sensor import, Tri-X
hover/apply, ISO grain controls, undo/redo, PNG/JPEG export, and exact settings after
reopening. Inspect fit/100% captures and GPU/performance state. Only then retire
the old application bundles. Hash the existing installation and saved-data files
before and after to verify preservation.

Build logs, scripts, signatures, acceptance summaries, captures and the final
source/hash manifest belong in `target/lightcraft-bw/`. Installation validation passed
on 2026-10-16: both staged and installed native Metal runs preserved exact settings
after reopening; their PNG/JPEG exports were byte-identical. Fit and 100% native
captures were inspected. Installed settled UI update was 0.66 ms; the measured
render was 19.8 ms, with no GPU fallback or pending work. Strict signatures and
all eight CI gates passed. Existing saved-data, everyday-app and CLI hashes were
unchanged. The three retired app bundles use `.app-backup` recovery names.

The PR1162 sections below
record the earlier deployment and its recovery procedure.

## Previous PR1162 installation

This deployment combines main `25ef65c81eef81833a50ff5b61f8676d80611b06` (0.7.0)
with PR #1162, `60df6a30e12aabd67c4c0033a924369de9ea718e`, on local branch
`local/pr1162-deploy`. The PR adds lossless Olympus / OM System ORF development,
including PEN E-PL7. Its verified coverage and colour limitations are in [orf.md](orf.md).
The branch and deployment are local; this procedure does not merge or publish on GitHub.
Full-corpus verification also exposed a pre-existing Sony ARW6 sparse-probe mismatch.
This integration makes ARW6 import probes read the whole file, with a synthetic
regression, while retaining sparse reads for formats whose header readers support them.
See [arw6-compression.md](arw6-compression.md).

## Installed components

| Component | Location |
|---|---|
| Desktop app | `~/Applications/LightCraft PR1162.app` |
| CLI package | `~/.local/share/lightcraft-pr1162/` |
| CLI launcher | `~/.local/bin/lightcraft-cli-pr1162` |
| Test library | `~/Pictures/LightCraft PR1162 Library` |
| Preferences, logs and default model folders | `~/Library/Application Support/LightCraft PR1162/` |

The app uses bundle identifier `ai.storyteller.lightcraft.pr1162` and display name
`LightCraft PR1162`. The generated bundle's `LSEnvironment` sets `LIGHTCRAFT_LIBRARY`
and `LIGHTCRAFT_CONFIG_DIR` to the absolute test paths above. These settings apply
when Finder, the Dock or `open` launches the bundle. Direct executable launches must
set the same environment explicitly. The CLI launcher sets those variables too;
persistent CLI commands still take an explicit `--library` argument.

The existing `/Applications/LightCraft.app`, its library and its preferences are
not replaced. A fresh test library starts with procedural demo photos. Verification
imports CC0 corpus samples, not personal photos. The control server is enabled only
for acceptance runs, on loopback port 18162.

## Build inputs and verification

Use Rust 1.95.0 with rustfmt, Clippy and the WASM target. If Rust is newly installed,
source its environment without changing the shell's startup files:

```sh
. "$HOME/.cargo/env"
rustup toolchain install 1.95.0 --profile minimal --component rustfmt --component clippy
rustup target add --toolchain 1.95.0 wasm32-unknown-unknown
```

The optional font checkout is required for this deployment. The pinned revision
matches the release workflow; `craft-fonts/` is gitignored:

```sh
git clone https://github.com/storytold/craft-fonts.git craft-fonts
git -C craft-fonts checkout --detach 3d9ab3a132ed0eff457e424226a435a901da30d3
export CRAFT_FONTS_DIR="$PWD/craft-fonts"
export CRAFT_FONTS_REQUIRED=1
```

Before committing the local integration, download the pinned corpus and require
all eight CI steps to pass:

```sh
cargo +1.95.0 xtask corpus --download
cargo +1.95.0 test -p lightcraft-raw --test orf_corpus -- --nocapture
cargo +1.95.0 test -p lightcraft-raw --lib vendor::orf
cargo +1.95.0 test -p lightcraft-raw --test vendor_robust
cargo +1.95.0 test -p lightcraft-engine --lib corpus_epl7_raw_import_and_reload_keep_sensor_development -- --nocapture
cargo +1.95.0 xtask ci
```

The ORF corpus log must report **23 ORF files checked**. Missing fixtures cause
individual tests to skip, so an exit status of zero alone is insufficient. The
engine regression must run with the E-PL7 fixture present. It checks raw import,
finite render pixels, camera-colour fitting and Reload from Disk preserving edits
while upgrading a preview-only entry. Malformed-input tests must return errors
without panicking.

## Packaging and installing

Package the verified, committed source with the existing release script:

```sh
RUSTUP_TOOLCHAIN=1.95.0 MACOS_SIGN_IDENTITY=- packaging/macos/package.sh --arch aarch64
```

It builds both binaries with HEIF support and `--locked`, includes font licences,
and checks signatures in the finished DMG and an install copy. Outputs are under
`dist/release/`; the assembled app is `target/macos-package/LightCraft.app`.
The default ad-hoc signature is for local testing on this Mac; no notarization
or release credentials are used.

Make a separate staging copy of that app. Set its display name and bundle ID as
above and add the two absolute paths under `LSEnvironment`. Preserve the original
executable name, icon, version and `LightCraftBuildCommit` metadata. Re-sign this
new local variant with the existing macOS entitlements, then require
`codesign --verify --deep --strict` and `plutil -lint` to pass. Do not modify the
signed release artifacts themselves.

Use `ditto` to install the verified variant into `~/Applications/LightCraft PR1162.app`.
Copy the CLI package's binary and licence files into `~/.local/share/lightcraft-pr1162/`.
Install the executable shell launcher `~/.local/bin/lightcraft-cli-pr1162`, exporting
the isolated configuration/library paths before executing that package's CLI.
Do not replace any existing component at these test paths without keeping its
previous copy. Record the final merge commit and SHA-256 hashes of the installed
executables and release artifacts in a gitignored deployment manifest.

## Acceptance and use

Launch the installed app normally:

```sh
open "$HOME/Applications/LightCraft PR1162.app"
"$HOME/.local/bin/lightcraft-cli-pr1162" --version
"$HOME/.local/bin/lightcraft-cli-pr1162" render corpus/raw/orf-olympus-epl7.orf \
  -o /tmp/lightcraft-pr1162-epl7.jpg --set light.exposure=0.7
```

For an acceptance run, quit only the test app first and launch it with the
control server:

```sh
open "$HOME/Applications/LightCraft PR1162.app" --args --control 18162
```

Drive it through [the control protocol](control-protocol.md), not OS keystrokes.
Import the E-PL7 sample, select it, change exposure, export JPEG, then quit and
reopen. Verify the photo and edit persist, the app uses the isolated paths, and
the active view renders from sensor data. Wait for `ui.inspect` to show the
expected photo, no pending render and empty queues before `ui.screenshot`; inspect
the image and `perf` data. A native screenshot must not report `rendering headlessly`.
Headless snapshots provide an additional check when the display is asleep, but
do not establish native-window acceptance. Also check a CLI export from the
installed binary and the installed app's code signature.

Use explicit library paths for persistent CLI work, with the desktop app closed:

```sh
"$HOME/.local/bin/lightcraft-cli-pr1162" run \
  --library "$HOME/Pictures/LightCraft PR1162 Library" catalog.query
```

Acceptance logs, screenshots, exports and the deployment manifest stay in
gitignored `target/local-pr1162/` or private temporary storage; no media is committed.

## Recovery and removal

If a staged or installed acceptance check fails, keep the existing everyday app
and diagnose the test app's log in its isolated settings folder. Fix source
problems, re-run the required CI gates, commit the green fix, then rebuild and
verify the test variant again.

To remove the deployment, quit the test app and remove only its named app, CLI
launcher and CLI package. Keep the test library and settings unless their deletion
is explicitly requested. The everyday app needs no restoration.

## Revision history

| Date | Change | Summary |
|---|---|---|
| 2026-10-21 | minor | SYNC.2: merge upstream 0.10.0 with local ORF/grain/Tri-X; installed components unchanged |
| 2026-10-16 | minor | BW.1: build/install Lightcraft BW; retire PR1162-named app bundles; retain existing catalog, preferences and CLI |
| 2026-10-16 | minor | SYNC.1: source updated to upstream main 0da998f2; installed app and CLI retain their earlier build |
| 2026-10-12 | minor | GRAIN.1: update the isolated app/CLI while retaining catalog/preferences and rollback copies |
| 2026-10-12 | minor | Pinned PR1162 integration, isolated ARM64 packaging, corpus and native acceptance, recovery instructions |

## GRAIN.1 update of the isolated test installation

`feature/analog-grain` starts at the ORF-enabled `8b5a0fcf`. After a green full CI run and
commit, build the same ARM64 HEIF/font package and preserve the PR1162 bundle identity and
`LSEnvironment` paths above. The existing test library and preferences remain in use.

Before replacing the test app, CLI package or launcher, quit only the PR1162 app through its
control channel. Keep timestamped rollback copies of those three components and snapshots
of its closed library/catalog and configuration under `target/analog-grain/rollback/`.
Validate staged signatures, then replace the isolated components. Do not overlay a running
bundle. Verify signatures again, CLI command availability, and source-commit metadata.

Acceptance uses a named virtual copy of the existing CC0 E-PL7 sample: click each ISO button,
toggle Advanced, check hover/undo/custom/reset behavior, render/export, and quit/reopen to
verify persistence. Inspect the Grain panel screenshot and `ui.inspect` performance/GPU
fields. Store evidence and the deployment manifest under `target/analog-grain/`, which is
ignored by Git. Preserve the original test photo's settings.

To roll back, close the test app and restore the saved app, CLI package and launcher. The old
build ignores `grain.advanced`; use the matching saved test library/configuration if exact
pre-update rendering/state is needed. Never restore over edits made since the backup without
first retaining them. The everyday `/Applications/LightCraft.app` is outside this update.

## SYNC.1 source update (2026-10-16)

`feature/tri-x-inspired` merges upstream main `0da998f2` (0.8.0) while retaining
PR #1162, `GRAIN.1` and `FILM.1`. The original local tip `2e76f189` is kept at
`backup/pr1162-grain-before-update-20261015`. The merge preserves both histories;
it does not push or publish them. The ORF import/reload regression follows the
new preview-version stamp, and preset Amount handling retains upstream's borrowed
100% path together with Advanced grain's default-off interpolation.

This source update does not install a new app or CLI. The installed isolated
components still correspond to the earlier deployment manifests. Build, package
and verify them with the procedure above when a deployment is requested.

The updated upstream source requires the release font input at
`3d9ab3a132ed0eff457e424226a435a901da30d3`, including Noto Sans Thai.
The original deployment used `8dcdacd5153e64560d109541a47d806f26f048c0`;
its installed artifacts are unaffected by updating the gitignored build input.

Verification for SYNC.1 passed all eight `cargo +1.95.0 xtask ci` gates on this
Mac with the release font pin above. The Apple M1 Max Metal adapter ran the
advanced-grain equivalence and Tri-X window comparisons; all 23 ORF reference
fixtures were present and the exact sensor comparison passed. E-PL7 import/reload,
grain Auto Sync and undo, and preset Amount tests passed. A headless UI run on the
E-PL7 raw passed 13 requests (Tri-X, clicking ISO 800, settings, undo and captures),
with no pending render at completion. Screenshots were inspected.

The CI report records 21 optional checks skipped for absent corpus files (10),
models (8) or extra font scripts (3); none is an ORF or GPU check. Detailed reports,
fixture inventory and headless evidence are in gitignored
`target/upstream-sync-20261016/`. No installed components or personal library data
were changed by this source update.

## SYNC.2 source update (2026-10-21)

`feature/tri-x-inspired` merges upstream main `abb49bdd` (0.10.0, 108 new
commits since SYNC.1). The previous tip `99f4ede0` is retained at
`backup/orf-trix-before-update-20261021`. Both histories are preserved.

The merge retains PR #1162 compressed ORF decoding, the 400/800/1600 profiles,
Advanced CPU/Metal grain, and `lc.bw-trix400-push2`. Upstream settings validation
now clamps values at the merge boundary; the local preset Amount interpolation
and explicit default-off Advanced handling remain intact. Upstream raw preview,
metadata, export, print and security fixes are included. README, Portuguese
translations and progress-document conflicts retain both sets of changes.

The release font pin remains `3d9ab3a132ed0eff457e424226a435a901da30d3`.
Detailed verification evidence is kept in `target/upstream-sync-20261021/`.
This is a source integration only: the installed Lightcraft BW app remains the
verified 0.8.0 build described above. No app/CLI installation, personal-library
change, remote push or publication is part of this update.

Verification: all eight `cargo +1.95.0 xtask ci` gates pass on the M1 Max.
The 23 ORF fixtures are present and the exact sensor-pixel comparison passes;
E-PL7 import/reload, Tri-X Amount/batch undo/correction preservation and CPU/Metal
window comparisons, and Advanced grain Auto Sync and GPU equivalence pass.
The report lists 21 optional missing-corpus/model/font skips, none ORF or GPU.
The coverage check and regenerated parity summary pass with unchanged totals.

A native scratch-library run passes E-PL7 import, Tri-X hover/apply, ISO grain
controls, Amount 0/49/50/100/200%, undo/redo, PNG/JPEG export and exact catalog
reopen. Fit and 100% screenshots were inspected and used native capture. Metal
rendering took 21.7 ms; settled UI update was 0.68 ms, with no GPU fallback or
pending render work. The test app was closed after acceptance.
