# Lightcorf source publication licence audit

**Last updated:** 2026-10-30 · **Task:** PUB.1 / SYNC.4 · **Scope:** source publication, not a binary release.

## Conclusion and baseline

The reviewed fork changes do **not change the code's declared licensing**. The workspace and new
original RAW crates retain `MIT OR Apache-2.0`; existing component exceptions remain intact.
This conclusion covers licence declarations, preserved notices, the documented provenance of the
local changes and the source being published. It is not a certification of every upstream file's
provenance, patent clearance, or permission to distribute every optional model or future binary.

- Upstream baseline: `31ee2643cf208a18cf780b8a63c677e87ab256f7`.
- Verified local history: `377095c2e2867f3853c9134bc067886d20997aa5`.
- Publication branch: `lightcorf`, in `tcholewa/lightcorf`, an independent fork of `storytold/lightcraft`.
- All existing local ancestry, author emails and deployment records are retained at the owner's request.
- SYNC.4 integrates upstream `445dd310` (0.15.0) into `lightcorf`, retaining the publication and
  local installation history. The fork's separate `main` remains GitHub's upstream snapshot.
- The 2026-10-22 provenance assessment below remains scoped to its stated baseline. The update
  retains the upstream licence texts and current NOTICE, the fork addendum and component exceptions;
  no restricted ArtCraft marks are restored. Updated dependencies remain pinned by Cargo.lock.

## Preserved terms

`LICENSE-MIT`, `LICENSE-APACHE` and `docs/brand/LICENSE-brand.txt` are byte-identical to the upstream
baseline. Licence fields are unchanged in all 34 existing Cargo manifests. The complete upstream `NOTICE` is retained as an unchanged prefix, with a separate fork
addendum. Existing copyright and third-party attribution are not reassigned to the fork maintainer.

| Material | Terms retained | Evidence / treatment |
|---|---|---|
| Workspace and original fork additions | MIT OR Apache-2.0 | Workspace manifest; new RAW package manifests inherit it |
| SAM segmentation implementation | Apache-2.0 only | Existing `crates/segment` manifest and NOTICE exception |
| Camera matrices and source data | Existing Apache / CC-BY attribution | Existing NOTICE and asset attribution retained |
| App icon | MIT OR Apache-2.0 | `assets/app-icon/LICENSE.txt`; icon unchanged |
| Inter and optional craft-fonts | OFL-1.1 | Existing attribution and licences; no extra font binaries added |
| Photographs and other assets | Per-asset terms | Existing asset attribution retained |
| SAM / optional RawNIND model weights | Separate upstream terms | Not committed or distributed; optional downloads remain separate |
| ArtCraft marks | Restricted trademark permission | Removed from current tree; original terms and attribution retained |

No GPL/LGPL/AGPL decoder source, Apple framework implementation, model weights, private RAW samples
or vendored binding sources are introduced in the published fork tree. This does not remove or
relicense existing independently licensed upstream dependencies.

## Core Image dependencies

Cargo.lock adds three workspace packages and `objc2-core-image` 0.3.2 relative to the baseline.
The other bindings below already appear in the upstream lockfile; this fork activates them for its
RAW helper. Registry licence expressions and the pinned upstream licensing statement were checked.

| Package | Version | Declared licence |
|---|---|---|
| lightcraft-raw-backend | 0.11.0 | MIT OR Apache-2.0 |
| lightcraft-raw-coreimage | 0.11.0 | MIT OR Apache-2.0 |
| lightcraft-raw-worker | 0.11.0 | MIT OR Apache-2.0 |
| objc2 | 0.6.4 | MIT |
| objc2-foundation | 0.3.2 | MIT |
| objc2-core-image | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| objc2-core-foundation | 0.3.2 | Zlib OR Apache-2.0 OR MIT |
| objc2-core-graphics | 0.3.2 | Zlib OR Apache-2.0 OR MIT |

The downloaded packages refer to a repository-level `LICENSE.md` instead of bundling separate
licence texts. The actual statement was retrieved at both pinned source revisions:
[bindings 7b1abfd](https://github.com/madsmtm/objc2/blob/7b1abfd750a2cacaea71d6a56ecfb83cb7de560b/LICENSE.md)
and [objc2 8852b42](https://github.com/madsmtm/objc2/blob/8852b424193ca41602281b3d7540d7c8ed51e49a/LICENSE.md).
Both statements have SHA-256 `7f976f7e9cb2d87df7230606feb932c3f21ac0e664045a775b600046ff850c54`.
Their referenced standard texts are [MIT](https://opensource.org/license/mit),
[Apache-2.0](https://www.apache.org/licenses/LICENSE-2.0) and [Zlib](https://zlib.net/zlib_license.html).

**Qualification:** the binding author explicitly notes uncertainty about redistribution of declarations
derived from Apple SDKs. This publication contains dependency references, not copies of those crates or
Apple SDKs/frameworks. It does not resolve that upstream qualification. macOS/Xcode terms still apply
to the system tooling; review the qualification and dependency notices before offering binary downloads.
Core Image FFI does not grant this fork ownership of Apple's implementation or change its licence.

## Local code provenance and notices

- ORF integration retains the original commit authors and its independent bitstream-research record
  in [ORF decoding](orf.md). The record describes patent publications and observed file data, not
  imported GPL decoder code. Patent validity or freedom to operate was not determined by this audit.
- [Grain and Tri-X](analog-grain.md) document original algorithms and a creative inspired preset.
  No commercial film profile, LUT or Kodak artwork was added; no measured stock match is claimed.
- [Core Image RAW](coreimage-raw.md) records the isolated Rust helper, Apple API usage, restart-only
  preference, catalogue migration and capability limits. These sources inherit the workspace licence.
- Modified comment-capable source/configuration/documentation files carry a change notice. The
  [modified-file inventory](fork-modified-files.txt) also records data and generated files whose formats
  do not support comments, and deleted files. Historical commits remain unrewritten.

## Branding and public interfaces

ArtCraft logo/mark files are removed from the current working tree, as required by their separate
terms for modified distributions. Historical snapshots are retained for provenance, not offered as
new binary releases. This review does not retroactively certify historical branding uses.
README variants identify the independent fork; About and Help distinguish fork support from upstream
credit. Copyright notices, the original brand terms and the permissively licensed app icon remain.

Legacy command IDs, crate names, the macOS bundle identifier `com.diffico.lightcraft.bw.coreimage`,
and personal configuration/library paths remain compatible. Legacy `app.discord` now opens fork
issues; `app.artcraft` opens the explicitly labelled upstream repository. No installed app is replaced.

## Verification and publication boundaries

The local-history scan inspected 545 reachable objects, including 330 file versions absent from
the upstream baseline. It found no matches for private-key blocks, GitHub/AWS/OpenAI/Slack token
patterns or credential-bearing HTTP URLs, and no private corpus, build, environment/key or model paths.
This is a bounded pattern/provenance check, not a guarantee that arbitrary secrets cannot exist.
Personal emails and deployment records are intentionally retained. Original samples and ignored
planning, build, font and model directories are not pushed.

Verification evidence is kept locally in `target/lightcorf-publication/`: dependency metadata,
pinned licence statements, scan results, CI report and native screenshots. Publication requires
`cargo xtask ci` and native About/Help and RAW-switch acceptance to pass on the final tree.
All eight local CI gates passed, with 22 explicit optional corpus/model/font skips. Native About/Help
checks and Core Image → Native → Core Image acceptance passed all seven samples; edits, history,
Reset 15 and virtual copies survived, samples were unchanged, and the Core Image round-trip PNG was
byte-identical. Metal was active without fallback; screenshots were inspected. Packaging identity
assertions passed, and workflow YAML/package XML parsed successfully. The staged source scan also
found no credential-pattern matches.
GitHub release and SonarCloud jobs are guarded to the upstream repository; the fork publishes source
only. No upstream PR, release tag, binary asset or backup branch is part of this task.

## Revision history

| Date | Task | Change |
|---|---|---|
| 2026-10-30 | SYNC.4 | Preserve current upstream NOTICE/licence texts, fork addendum and branding exclusions during the 0.15.0 merge; original audit scope retained |
| 2026-10-22 | PUB.1 | Initial source-fork audit, preserved code licensing and attribution, branding cleanup and explicit dependency qualification |
