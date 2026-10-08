# Notices

The scene format and the first version of this crate's types, checks, writer and tests come from the scene loader of the PITO game engine, by the same owner, at commit `81b2b0b9`: its format draft, its serde layer, its loader's checks and messages, its scene editor's patch model and tests, the material and finish keys, and the neutral test meshes and image in `tests/fixtures/`. They were copied here, rewritten for format 1, and are MIT licensed in this repository. Nothing here reads the engine at build, test or run time.

The plates keys (`[plates]`, the derived `[[object]] dynamic`, `[text.*] dynamic` and the proxies' box and hull shapes) follow the same engine's plates loader at commit `f40592e9`.

The `[trace]` keys `clamp_indirect` and `filter_glossy` follow the same engine's path tracer at commit `2246989b`.

The commits named here are from the engine's development history before its public release.

The demo project's meshes in `examples/demo/assets/` are copies of the test meshes in `tests/fixtures/assets/`.

`docs/check.gif`, the README's clip, is a recording of the `scene-check` example on the demo project, made from this repository with the recipe in `render/terminal.toml`, and is MIT licensed with the code.
