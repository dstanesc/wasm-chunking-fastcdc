## Wasm Chunking Fastcdc

This crate generates [WebAssembly](https://rustwasm.github.io/docs/book/what-is-webassembly.html) bindings to the [FastCDC](https://github.com/nlfiedler/fastcdc-rs) content defined slicing library. 


## Install for nodejs usage

```
npm install @dstanesc/wasm-chunking-fastcdc-node
```

## Install for bundler usage

```
npm install @dstanesc/wasm-chunking-fastcdc-webpack
```


## Usage TypeScript

```js
//import { compute_chunks } from "@dstanesc/wasm-chunking-fastcdc-node"
import { compute_chunks } from "@dstanesc/wasm-chunking-fastcdc-webpack"
const buf: Uint8Array  = ...
// chunking  spec: min_size 16 KB, avg size 32 KB, max size 64 KB
const offsets: Uint32Array = compute_chunks(buf, 16384, 32768, 65536)   
```


## Wasm build requirements

- [Rust toolchain](https://www.rust-lang.org/tools/install)
- [Wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)


## Wasm build

```sh
wasm-pack build --out-dir pkg/webpack --out-name chunking --target bundler --scope dstanesc
wasm-pack build --out-dir pkg/node --out-name chunking --target nodejs --scope dstanesc
```

## Wasm test

Interactive mode, eg. to check debugging info in the console
```sh
wasm-pack test --firefox
```

Headless mode
```sh
wasm-pack test --headless --firefox
```

## Pack for local usage

```
npm pack pkg/node/
npm pack pkg/webpack/
```


## Rust API

### Function wasm_chunking_fastcdc::compute_chunks

```rs
pub fn compute_chunks(
    source: &[u8], 
    min_size: u32, 
    avg_size: u32, 
    max_size: u32
) -> Result<Vec<u32>, RangeError>
```

Compute chunks from a given slice of bytes.

The min_size specifies the preferred minimum chunk size, max_size the preferred maximum chunk size; the avg_size is the desired “normal size” of the chunks. The smallest acceptable min_size is 64 bytes and likewise 256 bytes and 1024 bytes for avg_size and respectively max_size

A js_sys::RangeError is returned when the above chunking specification is out of range.

Example:

```rs
use wasm_chunking_fastcdc::compute_chunks;
let data: Vec<u8> = br"Lorem ipsum dolor sit amet, consectetur adipiscing elit...put more bits in here...".to_vec();
let slice: &[u8] = &data;
let min_size: u32 = 64;
let avg_size: u32 = 256;
let max_size: u32 = 1024;
let offsets: Vec<u32> = compute_chunks(slice, min_size, avg_size, max_size).unwrap();
```