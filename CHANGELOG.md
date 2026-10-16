# Changelog

## v1.4.0 — 2026-10-16

### Added

- Add manf, the .manf container crate, as a second workspace crate.

### Changed

- manf: a chunk read from a slice borrows the slice, so it can outlive the Reader.

## v1.3.0 — 2026-10-16

### Changed

- Credit pcod from pito-codecs for the README's GIFs.
- Key light, camera, particle, finish, haze, sun and sky values over time.

## v1.2.0 — 2026-10-16

### Added

- Add fabric, anisotropy and scatter material keys.

## v1.1.0 — 2026-10-16

### Added

- Add spot and parented lights, particles, flare, motion blur, outdoor haze, wet materials, LUT files, focus keys and a scene sound file.
- Count the new finish key and the ortho camera's focus_keys in the tests.

### Changed

- Show the PITO MD mark at the README's licence section.
- Use the PITO MD wordmark at the licence section.
- Let a light or particle system ride a glTF node inside its parent.

## v1.0.1 — 2026-10-14

### Added

- Every release carries its documentation as `docs-vX.Y.Z.tar.gz`: eight guides with real commands and output, the format, the changelog and the API reference.

## v1.0.0 — 2026-10-14

The first stable release. Format 1 and the API are the same as in v0.1.0, so moving to this tag needs no change.

### Added

- Three more examples: `scene-tree` prints a resolved scene, `scene-write` moves, renames, sets a material or migrates a file, and `scene-bench` times checking, resolving and editing scenes of any size.

### Changed

- The README shows each feature with a clip, and its measured speed: a 1,000-object scene resolves in 25 ms.
- Every push is tested on Linux, Windows and macOS.
- The brand reads PITO MD in the README, NOTICE.md and TRADEMARKS.md.

### Fixed

- The crate builds with no warnings on the newest Rust lints.

## v0.1.0

The first public release.

- Format 1 for scenes, prefabs, material libraries, proxies and `project.toml`, specified in [docs/format.md](docs/format.md): typed tables, ids, references, prefabs with overrides, includes, tunables, collision layers, rigid bodies, character controllers, triggers, camera rigs, animation and tile layers.
- `Project` opens a project, checks it, resolves a scene and reads one file as typed data; `check` checks an unsaved buffer. Every finding has a file, line, column, span and a stable code.
- `SceneEdit` edits a scene's files keeping their comments, spacing and order, with undo, redo and dry runs; `Project::fix` fills missing ids and `Project::migrate` rewrites format 0 files as format 1.
- Includes and prefabs nest at most 64 deep, and a cycle or a deeper chain is refused with its chain of files, however deep a project goes.
- The `scene-check` example and a demo project in `examples/`.
- Before this release the crate was called pito-scene, with the Rust path `pito_scene`. A dependency on it becomes `pfx-scene = { git = "https://github.com/gmrdad82/pfx-scene", tag = "v0.1.0" }`, and `use pito_scene::…` becomes `use pfx_scene::…`; nothing else changes.
