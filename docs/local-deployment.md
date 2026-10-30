<!-- Modified for the Lightcorf fork (2026); original licence retained. See docs/fork-license-audit.md. -->
# Local macOS installations

> **Last updated:** 2026-10-30 · **Change:** minor (SYNC.4: integrate upstream 0.15.0 and prepare the next local update)


## SYNC.4 source update (2026-10-30)

Upstream main `445dd310` (0.15.0, 635 commits since `31ee2643`) is integrated into
`lightcorf`, preserving the three previously unpushed local commits and all earlier fork history.
Recovery branch: `backup/lightcorf-before-update-20261030` at `ec3af76e`.

Core Image/Native selection, ORF/ORI decoding, Tri-X, advanced grain, sharpening 15, saved
Reset baselines and backend-specific caches survive upstream's module split and egui 0.37
migration. Render cache version 72 invalidates both parents' cached output; smart-preview
encoding 4 and catalog format 10 are retained. Help → Check for Updates explains source
builds and links to Lightcorf; the official LightCraft updater is not included.

Build inputs: Rust 1.95, pinned Cargo.lock, ARM64 macOS, HEIF, and craft-fonts
`b076417b5880bd4154bd22f00695b1d28c6bea83`. Existing installed software and personal data
are kept intact; acceptance uses a fresh scratch library and settings directory.
Evidence and preservation hashes live in `target/upstream-sync-20261030/`.
The first integration (`655f1eb4`, upstream `77c0a6ad`) passed all eight source CI gates,
with 21 explicit optional corpus/model/font skips. Three later upstream commits add
zip/webpki dependency updates and shared CI policy; Lightcorf retains source-only updates.
The final source also passed all eight CI gates with the same 21 optional skips before packaging.
The seven supplied Core Image samples, ORF checks, processor restart tests and CPU/GPU
grain checks ran. Packaging identity, workflow YAML, WiX XML and RAW coverage checks passed.
All 2,814 preservation hashes match.

### Prepared 0.15.0 update

The ARM64 release was built from `6f303fadd54aac7c0b0e3317ae2d5dc50b51a13d`, after both
upstream merges passed CI. The app and isolated RAW helper pass strict ad-hoc signature checks;
all required font licences are bundled. This is a local Mac build, not a notarized release.
The CLI reports 0.15.0. Nothing has been installed over 0.11.0.

| Artifact | Location |
|---|---|
| Ready app | `target/macos-package/Lightcorf.app` |
| Verified archive | `target/upstream-sync-20261030/Lightcorf-0.15.0-arm64.zip` |
| Installed 0.11.0 rollback copy | `target/upstream-sync-20261030/rollback/Lightcorf-0.11.0.app-backup` |
| Build, signature, file hashes and acceptance evidence | `target/upstream-sync-20261030/` |

The archive's 24 bundle files match the staged app. Its SHA-256 is
`90fd335a36b980ced60fda5b0de777532fec95ceed971a6917927f3286f0d2b2`.
The rollback copy matches the installed bundle's files and passes strict signature verification.

Staged native acceptance passed Core Image → Native → Core Image across three launches,
using all seven supplied camera files in an isolated library. Tri-X, explicit sharpening 45,
Reset to 15, undo, virtual copies, saved edits/history/versions, processor selection/cancellation,
and JPEG/PNG exports passed. The two Core Image PNG exports are byte-identical. About, photo,
processor-settings and update-guidance screenshots were inspected; no headless fallback or panic
occurred. Metal remained active without a GPU fallback, with no queued or in-flight renders.
The final recorded Core Image render was 51.4 ms and UI update 2.15 ms; Native was 32.1 ms
and 3.10 ms. These are acceptance-run samples, not a controlled performance comparison.
The test app is closed, and all 2,814 installed-app/personal-data/sample/plan hashes remain unchanged.

For the later installation, quit Lightcorf, preserve the current catalog/preferences, verify the
prepared bundle and rollback signatures, then replace `~/Applications/Lightcorf.app`. Retain bundle
ID `com.diffico.lightcraft.bw.coreimage` and the existing `LightCraft BW Core Image` data paths.
Verify the installed copy and repeat acceptance in a scratch library before opening personal data.
The app replacement and installed-copy acceptance are not part of this preparation record.

## Lightcorf (current installation)

Current installation: **0.11.0**, source `04743d8f`, installed 2026-10-22 with HEIF and the pinned
fonts at `~/Applications/Lightcorf.app`. This is the only installed LightCraft/Lightcorf app.
Settings → Performance → RAW processing selects Apple Core Image or LightCraft Native (Rust),
applied after restarting. The bundle identifier remains `com.diffico.lightcraft.bw.coreimage`.
[NAME.2 installation record](#name2-lightcorf-installation-and-cleanup-2026-10-22) records verification and cleanup.

The records below describe earlier deployments. NAME.2 supersedes their installed-version and rollback
statements: obsolete installed, staged and backup app bundles were removed at the user's request.
Their libraries, settings, source branches and verification evidence remain. Catalog format 10 used
by the current app cannot be opened by older builds.

[CLEAN.1](#clean1-build-artifact-cleanup-2026-10-23) supersedes earlier statements that the 0.11.0
staging bundle or compiler output remains available. Rebuild from source when needed; the installed
app and CLI remain independent of these generated directories. SYNC.4 above provides the new
0.15.0 staged bundle and a fresh rollback copy of the installed app.

Initial installation at `~/Applications/Lightcraft BW Core Image.app` used source `0139d009` on
`feature/macos-coreimage-raw`. The bundle identifier is `com.diffico.lightcraft.bw.coreimage`;
minimum macOS is 12. This local ARM64 0.10.0 build includes HEIF, Inter, pinned craft-fonts
`3d9ab3a132ed0eff457e424226a435a901da30d3` and a separately signed RAW helper inside `Contents/MacOS`.
The app, helper, finished DMG and CLI package pass strict ad-hoc signature checks. It is not notarized.

The initial installation evidence below is retained as its baseline. The CI.3 source update changes
new Core Image RAW imports to sharpening 15; see [default behavior](coreimage-raw.md). Subsequent
local build, signature and native acceptance records live in `target/sharpen-default-15/`.

A normal launch was verified to use these independent defaults, with an empty new library:

| Data | Path |
|---|---|
| Library | `~/Pictures/LightCraft BW Core Image Library` |
| Preferences / logs / models | `~/Library/Application Support/LightCraft BW Core Image` |

The existing `Lightcraft BW.app`, PR1162 library/preferences and personal `_orfs` samples remain
unchanged: all 975 recorded file hashes match. The new app is installed beside BW; no old bundle
was retired. Launch with:

```sh
open "$HOME/Applications/Lightcraft BW Core Image.app"
```

Staged and installed native acceptance passed all seven supplied samples (original E-M1, E-M1X,
OM-1 standard/high-resolution/ORI and GX9 RW2). Tri-X hover/apply, grain controls, Amount
0/49/50/100/200, undo/redo, PNG/JPEG export and exact catalog reopening passed. Native fit/100%
screenshots were inspected; Metal was active without fallback. All staged/installed comparison
exports are byte-identical. A full-resolution OM-1 high-resolution TIFF was also verified as
8160×6120, 16-bit RGB. Acceptance used scratch libraries, not either personal library.

The installed app's measured cached render was 14.8 ms (a preceding warm exposure edit took
44.5 ms; staged exposure edit 14.2 ms), with a settled UI update of 0.41 ms. First loads include
full RAW development and are slower; release CLI timing/memory measurements are in
[Core Image RAW](coreimage-raw.md#local-release-measurements).

Original E-M10 qualification remains pending its missing sample. AI sensor denoise, sensor-DNG
conversion and camera calibration are unavailable. On tested macOS build 26A434, generated linear
DNGs are rejected by Apple; HDR/panorama completion and rendered-DNG export report an error before
saving. A native rendered-DNG export was verified to leave no output file. Ordinary rendered
JPEG/PNG/TIFF export works. See [backend capabilities](coreimage-raw.md).

All eight quality gates passed before the source commit and again before the deployment-record
commit (22 explicit optional-data skips including original E-M10). Evidence, signed packages,
source/installed hash manifest, camera results, defaults
check, screenshots and performance measurements are under `target/coreimage-raw/`. The app was
closed after acceptance and is ready for normal use. No push or publication was performed.

## Lightcraft BW (existing Rust desktop installation)

The local 0.10.0 app is `~/Applications/Lightcraft BW.app`, built from the verified
ORF/grain/Tri-X integration `ec987997` (upstream main `abb49bdd`). Its bundle name and display name are
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
source/hash manifest for BW.2 are in `target/lightcraft-bw-20261021/`. The ARM64
release was built with Rust 1.95.0, HEIF and the pinned fonts. The signed package,
finished DMG and install-copy signatures passed strict verification. The BW
variant was then signed and verified separately with its retained identity.

Both staged and installed native Metal runs passed E-PL7 import, Tri-X hover/apply,
ISO grain controls, Amount 0/49/50/100/200%, undo/redo, PNG/JPEG export and exact
saved settings after reopening. Fit and 100% captures were inspected; neither
run used headless screenshot fallback. The staged and installed PNG/JPEG exports
are byte-identical. Installed render time was 19.2 ms, with a settled UI update
of 0.48 ms, no GPU fallback and no pending render work. All eight
`cargo +1.95.0 xtask ci` gates passed before packaging and again before committing
the deployment record (21 optional missing-data/model/font skips, none ORF or
GPU). The final CI report is in `target/lightcraft-bw-20261021/`.

Existing catalog, saved preferences, everyday-app and CLI hashes are unchanged.
The installed bundle is byte-identical to the staged BW bundle, and macOS app
registration was refreshed. The app was closed after acceptance, ready for normal
launch. No remote push or publication was performed.

The previous 0.8.0 BW app is retained byte-for-byte at
`target/lightcraft-bw-20261021/rollback/Lightcraft BW 0.8.0.app-backup`. To roll
back, close BW and restore that bundle to `~/Applications/Lightcraft BW.app`;
never overwrite edits made since the update with old library snapshots. The
first BW installation's evidence remains in `target/lightcraft-bw/`, and the
earlier retired PR1162 bundles remain in its `retired/` directory.

The sections below record earlier deployments and the source-only SYNC updates;
BW.2 supersedes their installed-version statements.

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
| 2026-10-30 | minor | SYNC.4: upstream 0.15.0 integration, green source CI, signed ARM64 bundle/archive, native processor-restart acceptance and rollback preparation; installed app remains 0.11.0 |
| 2026-10-23 | minor | CLEAN.1: remove debug/release/cross-target caches, staged package binaries and temporary caches; retain installed software, data and verification evidence |
| 2026-10-22 | minor | NAME.2: installed renamed ARM64 app, verified both RAW processors, removed two old bundles and 13 installer archives |
| 2026-10-22 | minor | NAME.1: Lightcorf bundle/executable/window/menu names; stable library and configuration identity |
| 2026-10-22 | minor | PUB.1: public Lightcorf source publication, separate from the existing installed app |
| 2026-10-22 | minor | CI.6: install 5922963c with restart-only RAW processor selection; remove obsolete app bundles and preserve personal data |
| 2026-10-22 | minor | CI.4: install Core Image 0.11.0 from 7e33c3ae; preserve previous bundle, BW app and personal data |
| 2026-10-22 | minor | SYNC.3: upstream 0.11.0 integrated on a new Core Image branch; previous source and installed apps preserved |
| 2026-10-21 | minor | CI.2: install separate Core Image app from 0139d009; verify cameras, defaults, signatures and preservation |
| 2026-10-21 | minor | BW.2: build/install 0.10.0 from ec987997; native staged/installed acceptance and identical exports; preserve data paths, saved data and 0.8.0 rollback bundle |
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

## SYNC.3 source update (2026-10-22)

`feature/macos-coreimage-upstream-20261022` starts from upstream main `31ee2643` (0.11.0,
471 upstream commits since the preceding integration) and merges `feature/macos-coreimage-raw`
at `ffc9047c`. The previous source is retained at `backup/coreimage-before-update-20261022`.
Core Image isolation, ORF/ORI support, Tri-X, advanced grain, sharpening 15 and the bundle identifier
`com.diffico.lightcraft.bw.coreimage` are preserved. Upstream's reduced-DNG and smart-preview changes
are adapted to the Apple backend; [Core Image RAW](coreimage-raw.md#upstream-0110-integration)
describes those changes. No installed app, CLI, or personal library is replaced by this source update.
Verification logs and isolated native acceptance artifacts are in `target/upstream-sync-20261022/`.

All eight `cargo xtask ci` gates passed with Metal active and 22 explicit optional-data skips
(the original E-M10 fixture is still absent). `cargo xtask parity --write` and RAW coverage checks
passed. The HEIF-enabled debug app passed all seven supplied-camera imports and JPEG exports,
Tri-X hover/apply, grain profiles, Amount 0/49/50/100/200, undo/redo, PNG export, reload preservation
and exact catalog reopening. A separate native restart check preserved sharpening 15, an explicit
edit of 45, and Reset back to 15. Fit, 100% and default-15 screenshots were inspected. The warm
scratch-app render was 48.7 ms with a 0.48 ms settled update, Metal active and no fallback; these
are debug observations, not a release performance comparison. Sample-file and saved-plan hashes
match the pre-merge manifest. No installed app or personal library was changed.

## CI.4 Core Image 0.11.0 installation (2026-10-22)

Installed `~/Applications/Lightcraft BW Core Image.app` from `7e33c3ae` on
`feature/macos-coreimage-upstream-20261022`. ARM64 release, macOS 12 minimum, HEIF,
Inter and craft-fonts `3d9ab3a1`; bundle ID `com.diffico.lightcraft.bw.coreimage`.
The helper and app are signed inside-out with the local ad-hoc identity; strict signature
verification passes. Staged and installed bundle file hashes are identical.

The signed release passed all seven supplied-camera imports/JPEG exports, Tri-X hover/apply,
grain profiles, Amount 0/49/50/100/200, undo/redo, PNG export, reload and exact catalog reopening.
The installed copy passed native launch, sharpening 15, Tri-X preservation, Reset 15 and persistence
of both the default and an explicit edit of 45 across restarts. Screenshots were inspected; Metal
was active without fallback, with a settled installed update of 0.38 ms. All test launches used
scratch libraries and configuration directories, closed after verification.

The previous 0.10.0 Core Image app is retained at
`target/install-coreimage-20261022/previous-installed.app-backup`. The original BW app, installed
CLI, personal libraries/settings, samples and saved AI plan are unchanged: 978 recorded file hashes
match. Evidence and the installation manifest are in `target/install-coreimage-20261022/`.
Original E-M10 qualification still awaits its fixture; sensor-tool/DNG limitations remain as
recorded in [Core Image RAW](coreimage-raw.md).

All eight final `cargo xtask ci` gates passed with 22 explicit optional-data skips; reports are
kept alongside the installation evidence. No code or feature-parity counts changed in this task.

## CI.6 current build and cleanup (2026-10-22)

Installed source `5922963c` from `feature/raw-processor-setting` as ARM64 version 0.11.0 with HEIF,
Inter and pinned craft-fonts `3d9ab3a132ed0eff457e424226a435a901da30d3`. The app and isolated RAW
helper pass strict ad-hoc signature checks; staged and installed file hashes match.

Installed native acceptance passed Core Image → Native → Core Image with all seven supplied RAW
samples, JPEG/PNG exports, Tri-X, saved edits/history/versions, virtual copies, Reset sharpening 15,
and the Settings selector including cancellation of a pending change. Core Image PNG exports before
and after the round trip are byte-identical. Native Settings screenshots were inspected; Metal was
active without headless fallback. Tests used a temporary library and configuration.

Removed 21 obsolete app bundles, including the previous Core Image installation, `/Applications/LightCraft.app`,
`~/Applications/Lightcraft BW.app`, and obsolete staged/rollback app bundles beneath `target/`.
The old 0.7.0 app DMG was also removed. Only `~/Applications/Lightcraft BW Core Image.app` remains in the two Applications
folders. The current signed staging bundle is retained as build evidence. Old libraries and settings
were preserved: all 2,756 recorded personal-data/sample file hashes still match. Source branches,
CLI installations and verification records were preserved.

Evidence: `target/install-current-20261022/` contains the deployment and removal manifests,
preservation hashes, release build log, installed native acceptance results and screenshots.
All eight `cargo xtask ci` gates passed before the deployment-record commit. No push was requested.

## PUB.1 public source fork (2026-10-22)

The `lightcorf` publication branch preserves verified source `377095c2` and its local history,
with publication branding, support-link and attribution changes. See [the fork audit](fork-license-audit.md).
This task does not install another app or alter personal libraries. The installed source remains
`5922963c`, with deployment record `377095c2`; the fork's source-only publication is separate.

## NAME.1 Lightcorf app naming (2026-10-22)

The macOS package is now `Lightcorf.app`, with executable `Contents/MacOS/Lightcorf`,
Lightcorf window titles, and matching About/Quit/Hide labels. The bundle identifier remains
`com.diffico.lightcraft.bw.coreimage`; existing library and settings directories remain unchanged.
This is a display-name update, not a data migration. Packaging assertions check both names and identity.

Build the signed ARM64 app without creating a DMG or CLI archive:

```sh
CRAFT_FONTS_DIR="$PWD/craft-fonts" bash packaging/macos/package.sh --arch aarch64 --app-only
```

The installation target is `~/Applications/Lightcorf.app`. Local installation verification uses
scratch libraries for both RAW processors and preserves personal data. Older LightCraft/Lightcorf
app bundles are removed only after the replacement passes native acceptance and signature checks.

## NAME.2 Lightcorf installation and cleanup (2026-10-22)

Installed `Lightcorf.app` 0.11.0 from source `04743d8f` at `~/Applications/Lightcorf.app`.
The ARM64 release includes HEIF, the supervised RAW helper and pinned craft-fonts
`3d9ab3a132ed0eff457e424226a435a901da30d3`. App/helper strict ad-hoc signatures pass;
staged and installed files have identical hashes. This local build is not notarized.

Native acceptance passed Core Image → Native → Core Image with all seven supplied samples,
Tri-X, sharpening default/Reset 15, saved edits/history, virtual copies and exports.
The Core Image PNG round trip is byte-identical. About, photo and Settings screenshots
were inspected; Metal was active without fallback. The test app was closed afterward.

Removed the previous installed and staged BW Core Image bundles and 13 obsolete DMG/CLI
installer archives. Only Lightcorf is installed; its current staging bundle remains.
All 2,756 recorded personal library, settings and sample hashes match. Existing data paths,
source branches, installed CLI commands and verification evidence are preserved.

Evidence: `target/install-lightcorf-20261022/` contains source CI/build logs, installation
hashes, native acceptance/screenshots and removal manifests. Full `cargo xtask ci` passed
for both the source rename and deployment record (22 explicit optional-data/model/font skips).
No GitHub binary release or push was requested.

## CLEAN.1 build artifact cleanup (2026-10-23)

Removed **159.5 GiB** of allocated build output, reducing the workspace from about 165 GiB to
5.5 GiB. This includes the repository's Cargo debug, release, ARM64 and WASM output, the macOS staging bundle,
obsolete package/staging CLI binaries, Python bytecode caches, empty release directories and Finder
metadata, plus the temporary publication-audit Python environment. Historical acceptance records,
scripts, screenshots, benchmark history and scratch-library backups remain under `target/`.

The installed `~/Applications/Lightcorf.app`, installed PR1162 CLI, `_orfs`, CC0 corpus, pinned
craft-fonts and saved AI plan are retained. Cleanup uses an explicit untracked-path list, with
before/after hashes for these protected inputs and installed binaries: all **200 protected files**
are byte-identical. Libraries and settings outside the repository were not cleanup targets. The
remaining `target/` evidence occupies about 1.4 GiB.

All eight gates of `cargo +1.95.0 xtask ci` passed before removing its generated build output,
with 22 explicit optional-data/model/font skips. The cleanup manifest and CI log are retained in
`target/cleanup-20261023/`; `target/ci/` retains the gate report.
