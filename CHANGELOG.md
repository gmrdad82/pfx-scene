# Changelog

## v0.1.0

The first public release.

- Format 1 for scenes, prefabs, material libraries, proxies and `project.toml`, specified in [docs/format.md](docs/format.md): typed tables, ids, references, prefabs with overrides, includes, tunables, collision layers, rigid bodies, character controllers, triggers, camera rigs, animation and tile layers.
- `Project` opens a project, checks it, resolves a scene and reads one file as typed data; `check` checks an unsaved buffer. Every finding has a file, line, column, span and a stable code.
- `SceneEdit` edits a scene's files keeping their comments, spacing and order, with undo, redo and dry runs; `Project::fix` fills missing ids and `Project::migrate` rewrites format 0 files as format 1.
- Includes and prefabs nest at most 64 deep, and a cycle or a deeper chain is refused with its chain of files, however deep a project goes.
- The `scene-check` example and a demo project in `examples/`.
- Before this release the crate was called pito-scene, with the Rust path `pito_scene`. A dependency on it becomes `pfx-scene = { git = "https://github.com/gmrdad82/pfx-scene", tag = "v0.1.0" }`, and `use pito_scene::…` becomes `use pfx_scene::…`; nothing else changes.
