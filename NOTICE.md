# Notices

The scene format and the first version of this crate's types, checks, writer and tests come from the scene loader of the PITO MD game engine, by the same owner, at commit `81b2b0b9`: its format draft, its serde layer, its loader's checks and messages, its scene editor's patch model and tests, the material and finish keys, and the neutral test meshes and image in `tests/fixtures/`. They were copied here, rewritten for format 1, and are MIT licensed in this repository. Nothing here reads the engine at build, test or run time.

The plates keys (`[plates]`, the derived `[[object]] dynamic`, `[text.*] dynamic` and the proxies' box and hull shapes) follow the same engine's plates loader at commit `f40592e9`.

The `[trace]` keys `clamp_indirect` and `filter_glossy` follow the same engine's path tracer at commit `2246989b`.

The commits named here are from the engine's development history before its public release.

The demo project's meshes in `examples/demo/assets/` are copies of the test meshes in `tests/fixtures/assets/`.

The README's clips, `docs/images/check.gif`, `docs/images/tree.gif`, `docs/images/write.gif` and `docs/images/migrate.gif`, are recordings of the `scene-check`, `scene-tree` and `scene-write` examples on copies of the demo project (the migration clip on `render/terminal/room-format0.scene.toml`), made from this repository with the recipe in `render/terminal.toml`. The chart `docs/images/speed.png` is drawn from `render/charts/speed.toml`, which holds the `scene-bench` example's measurements. All are MIT licensed with the code.

The mark at the README's licence section, `docs/images/pitomd.svg`, is the PITO MD mark, copied by `bin/marks` from the PITO MD brand masters at `fe97f221c8c071389283144c416f4fcde87b3958` (its note is `docs/images/SOURCE`). The PITO MD name and its mark are © Catalin Ilinca, all rights reserved and not covered by the MIT licence ([TRADEMARKS.md](TRADEMARKS.md)).
