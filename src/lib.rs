//! This crate is a wrapper on the "FastCDC" content defined chunking
//! library [written in Rust by Nathan Fiedler](https://github.com/nlfiedler/fastcdc-rs)

use fastcdc::FastCDC;
use fastcdc::AVERAGE_MIN;
use fastcdc::MAXIMUM_MIN;
use fastcdc::MINIMUM_MIN;
use js_sys::RangeError;
use wasm_bindgen::prelude::*;

///
/// Compute chunks from a given slice of bytes.
///
/// The `min_size` specifies the preferred minimum chunk size,
/// `max_size` the preferred maximum chunk size; the `avg_size` is the
/// desired "normal size" of the chunks. The smallest acceptable 
/// `min_size` is 64 bytes and likewise 256 bytes and 1024 bytes 
/// for `avg_size` and respectively `max_size`
///
/// A js_sys::RangeError is returned when the above chunking specification is out of range.
/// 
/// 
/// Example:
/// 
/// ```
/// use wasm_chunking_fastcdc::compute_chunks;
/// let data: Vec<u8> = br"Lorem ipsum dolor sit amet, consectetur adipiscing elit...put more bits in here...".to_vec();
/// let slice: &[u8] = &data;
/// let min_size: u32 = 64;
/// let avg_size: u32 = 256;
/// let max_size: u32 = 1024;
/// let offsets: Vec<u32> = compute_chunks(slice, min_size, avg_size, max_size).unwrap();
/// ```
#[wasm_bindgen]
pub fn compute_chunks(
    source: &[u8],
    min_size: u32,
    avg_size: u32,
    max_size: u32,
) -> Result<Vec<u32>, RangeError> {
    if min_size < MINIMUM_MIN as u32 {
        Err(RangeError::new("min_size should be >= 64"))
    } else if avg_size < AVERAGE_MIN as u32 {
        Err(RangeError::new("avg_size should be >= 256"))
    } else if max_size < MAXIMUM_MIN as u32 {
        Err(RangeError::new("max_size should be >= 1024"))
    } else {
        Ok(compute_chunks_nocheck(source, min_size, avg_size, max_size))
    }
}


///
/// Compute chunks from a given slice of bytes.
///
/// The `min_size` specifies the preferred minimum chunk size,
/// `max_size` the preferred maximum chunk size; the `avg_size` is the
/// desired "normal size" of the chunks. The smallest acceptable 
/// `min_size` is 64 bytes and likewise 256 bytes and 1024 bytes 
/// for `avg_size` and respectively `max_size`
/// 
/// No js friendly chunking specification range validation
/// Example:
/// 
/// ```
/// use wasm_chunking_fastcdc::compute_chunks_nocheck;
/// let data: Vec<u8> = br"Lorem ipsum dolor sit amet, consectetur adipiscing elit...put more bits in here...".to_vec();
/// let slice: &[u8] = &data;
/// let min_size: u32 = 64;
/// let avg_size: u32 = 256;
/// let max_size: u32 = 1024;
/// let offsets: Vec<u32> = compute_chunks_nocheck(slice, min_size, avg_size, max_size);
/// ```
#[wasm_bindgen]
pub fn compute_chunks_nocheck(
    source: &[u8],
    min_size: u32,
    avg_size: u32,
    max_size: u32,
) -> Vec<u32> {
    let mut chunks = Vec::new();
    let chunker = FastCDC::new(
        source,
        min_size as usize,
        avg_size as usize,
        max_size as usize,
    );
    for entry in chunker {
        chunks.push(entry.offset as u32);
    }
    chunks.push(source.len() as u32);
    chunks
}
