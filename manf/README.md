# manf

The `.manf` container: one file holding named chunks, each checksummed and either stored as it is or compressed, so a reader opens the table and decodes only the chunks it needs. Manfred writes its documents in it and pfx reads them. Std only, no `unsafe`; its one dependency is miniz_oxide (raw deflate). Works on Linux, Windows and macOS.

## Layout

Little-endian throughout.

| Bytes | Holds |
| --- | --- |
| 8 | magic `MANFRED\0` |
| 4 | format version, `u32` (1) |
| 4 | chunk count, `u32` (at most 65,536) |
| 45 × count | the chunk table, uncompressed, one entry per chunk in file order |
| sum of name lengths | the name block, uncompressed UTF-8 |
| up to 7 | zero padding to an 8-byte boundary |
| … | the payloads, each starting on an 8-byte boundary, zero padded between |

A table entry:

| Bytes | Field |
| --- | --- |
| 4 | tag, `[u8; 4]` |
| 4 | flags, `u32`: bit 0 compressed, bit 1 stored as is; exactly one is set, the others are 0 |
| 1 | codec: 0 none, 1 raw deflate (RFC 1951, no zlib header), 2 LZ4 block (no frame, no size prefix) |
| 8 | payload offset from the start of the file, a multiple of 8 |
| 8 | stored length |
| 8 | raw length (at most 4 GiB) |
| 4 | CRC-32 (IEEE, as zlib's `crc32`) of the raw bytes |
| 4 | name offset into the name block |
| 4 | name length (0 for an unnamed chunk, at most 1024) |

A name is a relative `/` path in UTF-8: no leading `/`, no `\`, no `:` (so no drive), no empty, `.` or `..` part, no control character, at most 1024 bytes, and unique among the chunks of its tag.

A reader refuses a file that is not `.manf` (the magic), a newer format version than its own, and a file whose table does not hold: a chunk count over the cap, a name out of the block or invalid, flags that disagree with the codec, a stored chunk whose stored and raw lengths differ, a raw length over 4 GiB or over what its stored bytes can expand to (1,032 times for deflate, 256 times for LZ4, plus 64), and a payload that is misaligned, overlaps the table or runs past the end. A chunk decodes into exactly its raw length, never more, and its checksum must match. Unknown tags are skipped: the reader lists them and decodes them only when asked. Older format versions read.

## Tags

The crate names the well-known tags; every other tag is opaque to it.

| Tag | Holds |
| --- | --- |
| `GLTF` | the glTF JSON |
| `GBIN` | the glTF binary buffer, so a reader hands both straight to its glTF path |
| `SCNE` | a pfx scene TOML file, repeatable, named by its path (`main.toml`, `materials/wood.toml`) |
| `SMET` | the scene's metadata, unnamed, stored TOML: `format`, `entry`, `recipe` and `scene` |
| `RCPE` | Manfred's recipe TOML |
| `FILE` | a referenced file's bytes alone, repeatable, named by its relative path |
| `THMB` | a PNG thumbnail, stored |
| `FACT` | Manfred's facts, TOML |
| `NODE` | Manfred's object and piece names to glTF node indices |
| `OVRD` | Manfred's overrides of edits made outside it |

## The hashes and the rules for editing

`SMET`'s `recipe` is the lowercase hex SHA-256 of `RCPE`'s raw bytes, and its `scene` the lowercase hex SHA-256 over the `SCNE` chunks in table order, each as its name, a 0x00 byte, then its raw bytes, concatenated. `manf::scene_hash(&reader)` computes the second, `manf::sha256::sha256` and `manf::sha256::hex` the first. Manfred writes both when it builds; on open it recomputes the scene hash, and a mismatch says the scene was edited outside Manfred.

pfx and pfx-scene change only `SCNE`, `FILE` and `SMET`'s `format` and `entry`, never its `recipe` or `scene`. They copy every other chunk through byte for byte, in order (`Writer::copy`), and add a new chunk at the end.

## Use

```rust
use manf::{Codec, Reader, Writer, GLTF, GBIN, THMB};

let bytes = Writer::new()
    .chunk(GLTF, json, Codec::Auto)
    .chunk(GBIN, bin, Codec::Auto)
    .named(manf::FILE, "maps/wood.png", png, Codec::Store)
    .to_vec()?;
let reader = Reader::from_bytes(&bytes)?;
let thumb = reader.chunk(THMB)?;
let wood = reader.named(manf::FILE, "maps/wood.png")?;
```

- `Reader::from_bytes(&[u8])` reads a slice it borrows; a stored chunk comes back borrowed, zero-copy. `Reader::open(path)` reads only the header, the table and the name block, then each chunk's range when it is asked for.
- `reader.chunk(tag)` (the first of a tag), `reader.named(tag, name)`, `reader.chunks(tag)` (every one, with its name) return `Result<Option<Cow<[u8]>>>` or a list; `reader.entries()` lists every chunk's tag, name, flags, codec, offset, stored and raw lengths and checksum; `reader.read(index)` and `reader.stored(index)` take a table index.
- `Writer::new().chunk(tag, bytes, codec)`, `.named(tag, name, bytes, codec)` and `.copy(&reader, index)?` (a chunk copied through as stored, without recompressing) build a file; `.to_vec()` gives its bytes and `.write(path)` writes it atomically (a temporary file beside it, synced, then renamed). The same chunks give the same bytes. Chunks of 1 MiB or more are encoded on their own threads.
- `Codec::Auto` stores PNG, JPEG, RIFF, gzip, zip, zstd, Ogg and FLAC data as it is, compresses everything else with LZ4 and keeps the raw bytes when that saves nothing. `Codec::Store`, `Codec::Deflate` (level 6) and `Codec::Lz4` force one.
- `manf::lz4::compress` and `manf::lz4::decompress(bytes, raw_len)` are the LZ4 block codec on its own.

## Why LZ4

Measured on 2026-10-16 with Manfred's release build at 173f033, on its GLBs split into the JSON and binary chunks; each codec ran 5 times, interleaved with the others, and the table gives the medians. Sizes are bytes, with the share of the raw size in brackets. Decode is the whole GLB's bytes; the last column reads the file, decodes `GLTF` and `GBIN` and parses them with the gltf crate. The runs shared the machine with other builds, so the times are ordinary, not clean.

| Scene | GLB bytes | Deflate 6 | Deflate 1 | LZ4 | Decode, deflate 6 | Decode, LZ4 | Open `.manf` to parsed glTF: stored, deflate 6, LZ4 |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| the coupe (`manfred/car`, sports) | 549,592 | 173,102 (0.31) | 191,365 (0.35) | 243,349 (0.44) | 0.77 ms | 0.13 ms | 0.08, 0.83, 0.18 ms |
| a building piece (`manfred/buildings`, 20 m route) | 94,580 | 14,173 (0.15) | 18,160 (0.19) | 24,451 (0.26) | 0.065 ms | 0.014 ms | 0.036, 0.12, 0.056 ms |
| the chase scene (`showcase/chase.toml`) | 6,714,268 | 1,705,823 (0.25) | 1,957,058 (0.29) | 2,786,567 (0.42) | 6.48 ms | 1.19 ms | 0.86, 7.25, 2.00 ms |

LZ4 opens the chase scene 3.6 times faster than deflate and decodes 5.4 times faster, for a file 1.6 times larger (2.8 MB against 1.7 MB); it also encodes in 6.4 ms against deflate 6's 145 ms. Deflate 1 is both larger and slower to decode than deflate 6. So `Codec::Auto` uses LZ4, and deflate stays for a writer that wants the smaller file. The port decodes as fast as lz4_flex's safe decoder and gives the same bytes.

## Credits

`src/lz4.rs` ports the LZ4 block encoder and decoder of lz4_flex 0.14.0 (https://github.com/pseitz/lz4_flex, commit 1bffdcbbf906a234b913937cb2f57c0245915038), Copyright (c) 2020 Pascal Seitz, MIT (its text is in NOTICE): the safe compressor's hash tables (4K entries, u16 positions under 64 KiB and the 5-byte hash above it), its step that grows after 32 misses, the backward match extension and the 8-byte match counting, and the safe decoder's fast path for short tokens with its 16-byte literal and 18-byte match copies. The port gives the same compressed bytes as lz4_flex on 64-bit little-endian machines, which the tests check. The block format is LZ4's, by Yann Collet (https://github.com/lz4/lz4/blob/dev/doc/lz4_Block_format.md).

`src/crc32.rs` ports the slice-by-16 CRC-32 of crc32fast 1.5.0 (https://github.com/srijs/rust-crc32fast, tag v1.5.0), Copyright (c) 2018 Sam Rijs, Alex Crichton and contributors, MIT or Apache-2.0, taken under MIT (its text is in NOTICE): the 16 lookup tables, built here at compile time from the IEEE polynomial instead of written out, and its 64-byte unrolled loop. The tests check it against crc32fast.

`src/sha256.rs` is pfx-scene's own SHA-256 (https://github.com/gmrdad82/pfx-scene, `src/sha256.rs` at commit 2024691c0ac7e18eb72fe68d13e12d80c5ba997e), MIT, with a public `hex`.

## Licence

MIT, Catalin Ilinca.
