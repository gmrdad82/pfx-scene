---
title: Install
description: Add pfx-scene to a Rust project on Linux, Windows or macOS, and try it on the demo project.
sidebar:
  order: 2
---

pfx-scene is a Rust library. It needs a stable Rust toolchain (edition 2024) and nothing else: no system library, no GPU and no C compiler. It builds and is tested the same way on Linux, Windows and macOS.

## Add it to a project

The crate is published on GitHub and pinned by tag. In your `Cargo.toml`, on any of the three systems:

```toml
[dependencies]
pfx-scene = { git = "https://github.com/gmrdad82/pfx-scene", tag = "v1.0.0" }
```

Then `use pfx_scene::…` in your code. Every tag is on the [releases page](https://github.com/gmrdad82/pfx-scene/releases).

## Try the demo

The repository holds a demo project in `examples/demo` and four small programs on the API. Clone it and check the demo:

```sh
git clone https://github.com/gmrdad82/pfx-scene
cd pfx-scene
cargo run --release --example scene-check -- examples/demo
```

```text
scenes/room.scene.toml: 6 objects, 2 prefab placements, 3 bodies, 1 light, 2 materials
no errors
```

The same commands work in PowerShell on Windows and in a terminal on Linux or macOS. The other programs are `scene-tree`, `scene-write` and `scene-bench`; the guides use each of them.

## Quick start

The shortest useful program opens a project, prints every finding and resolves its scene:

```rust
use std::path::Path;

fn main() {
    let project = pfx_scene::Project::open(Path::new("my-game")).unwrap();
    for diagnostic in project.check() {
        println!("{diagnostic}");
    }
    let scene = project.scene(Path::new("scenes/room.scene.toml")).unwrap();
    println!("{} objects", scene.objects.len());
}
```

`my-game` is a folder with a `project.toml`. [Lay out a project](../guides/projects/) says what goes in it, and [Check a project](../guides/check/) shows what the findings look like.
