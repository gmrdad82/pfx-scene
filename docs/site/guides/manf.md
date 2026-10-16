---
title: Read and write a .manf file
description: The manf crate's container of named, checksummed chunks, stored or compressed with LZ4 or deflate, read one chunk at a time.
sidebar:
  order: 18
---

A `.manf` file holds named chunks: an 8-byte magic `MANFRED\0`, a version, an uncompressed table of chunks with their names, then each chunk on an 8-byte boundary, stored as it is or compressed on its own with LZ4 or raw deflate, and checksummed with CRC-32. A reader opens the table and decodes only the chunks it asks for. The `manf` crate in this repository reads and writes it; its only dependency is miniz_oxide.

## Write one

```rust
use manf::{Codec, FILE, GBIN, GLTF, Writer};

fn main() -> manf::Result<()> {
    let json = br#"{"asset":{"version":"2.0"}}"#.to_vec();
    let bin = vec![0u8; 1024];
    let png = std::fs::read("wood.png")?;
    Writer::new()
        .chunk(GLTF, json, Codec::Auto)
        .chunk(GBIN, bin, Codec::Auto)
        .named(FILE, "maps/wood.png", png, Codec::Auto)
        .write("car.manf")
}
```

`Codec::Auto` compresses with LZ4, stores PNG, JPEG and other packed data as it is, and keeps the raw bytes when compressing saves nothing. `Codec::Store`, `Codec::Lz4` and `Codec::Deflate` force one. The same chunks give the same bytes, and `write` writes a temporary file beside the target, syncs it and renames it.

## Read one

```rust
use manf::{FILE, GLTF, Reader};

fn main() -> manf::Result<()> {
    let reader = Reader::open("car.manf")?;
    for entry in reader.entries() {
        let tag = manf::tag_text(entry.tag);
        println!("{tag} {:?}: {} of {} bytes", entry.name, entry.stored, entry.raw);
    }
    let gltf = reader.chunk(GLTF)?;
    let wood = reader.named(FILE, "maps/wood.png")?;
    println!("{:?} {:?}", gltf.map(|c| c.len()), wood.map(|c| c.len()));
    Ok(())
}
```

`Reader::open` reads the header, the table and the names, then each chunk's bytes when asked. `Reader::from_bytes` reads a slice, for example a memory map the caller owns, and hands a stored chunk back borrowed, without a copy. `chunk`, `named` and `chunks` return `Ok(None)` or an empty list for a chunk that isn't there.

## What a reader refuses

A file that isn't a `.manf`, a newer version than the reader's, more than 65,536 chunks, a raw length over 4 GiB or over what its stored bytes can expand to, an invalid or repeated name, a misplaced or cut chunk, and a chunk whose checksum or decoded length is wrong all come back as an `Error`, never a panic. Unknown tags are listed and skipped.

## The tags

`GLTF` and `GBIN` hold a document's glTF JSON and binary buffer, `SCNE` its scene TOML files (named by path), `SMET` the scene's metadata, `FILE` the files it names, `THMB` a PNG thumbnail, and `RCPE`, `FACT`, `NODE` and `OVRD` belong to Manfred. The crate's [README](https://github.com/gmrdad82/pfx-scene/blob/main/manf/README.md) is the full specification: the byte layout, the hashes and the rules for tools that rewrite a file.
