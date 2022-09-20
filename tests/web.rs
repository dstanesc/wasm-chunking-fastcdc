//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use js_sys::JsString;
use js_sys::RangeError;
use wasm_bindgen_test::*;
use wasm_chunking_fastcdc::compute_chunks;
use wasm_chunking_fastcdc::compute_chunks_nocheck;

wasm_bindgen_test_configure!(run_in_browser);

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

fn test_data() -> Vec<u8> {
    let data: Vec<u8> = br"Lorem ipsum dolor sit amet, consectetur adipiscing elit. Ut sit amet bibendum lorem. In pharetra quis felis vel placerat. Aenean eget elementum turpis. Phasellus dolor sem, facilisis a suscipit nec, ullamcorper quis lorem. Proin faucibus purus nec diam feugiat, ac pulvinar purus venenatis. Suspendisse ultrices vestibulum tortor laoreet condimentum. Ut id sapien porta, fringilla urna vitae, congue mi. Praesent eleifend tempus justo, eu volutpat mauris vehicula at. Phasellus ornare congue tortor a tristique. Praesent tristique dolor bibendum sem tempus, ac finibus felis egestas. Nulla nec massa porta, pulvinar urna at, faucibus nisl. Nullam convallis sem eget enim convallis, vel sagittis leo hendrerit. Suspendisse hendrerit mauris faucibus, fermentum augue vel, pulvinar quam. Quisque commodo nulla vel nulla mattis porta. Maecenas fermentum dictum tempor. Praesent rutrum eu erat at euismod.".to_vec();
    data
}

#[wasm_bindgen_test]
fn compute_chunks_showcase() {
    let data: Vec<u8> = test_data();
    let slice: &[u8] = &data;
    let len = data.len() as u32;
    log!("Input buffer size: {}", len);
    let min_size: u32 = 64;
    let avg_size: u32 = 256;
    let max_size: u32 = 1024;
    let offsets: Vec<u32> = compute_chunks(slice, min_size, avg_size, max_size).unwrap();
    let mut previous_offset: u32 = 0;
    for offset in offsets {
        log!(
            "Fastcdc offsets last_offset:{}, current:{}, len:{} bytes",
            previous_offset,
            offset,
            offset - previous_offset
        );
        previous_offset = offset;
    }
    assert_eq!(len, previous_offset);
}

#[wasm_bindgen_test]
fn compute_chunks_nocheck_showcase() {
    let data: Vec<u8> = test_data();
    let slice: &[u8] = &data;
    let len = data.len() as u32;
    log!("Input buffer size: {}", len);
    let min_size: u32 = 64;
    let avg_size: u32 = 256;
    let max_size: u32 = 1024;
    let offsets: Vec<u32> = compute_chunks_nocheck(slice, min_size, avg_size, max_size);
    let mut previous_offset: u32 = 0;
    for offset in offsets {
        log!(
            "Fastcdc offsets last_offset:{}, current:{}, len:{} bytes",
            previous_offset,
            offset,
            offset - previous_offset
        );
        previous_offset = offset;
    }
    assert_eq!(len, previous_offset);
}

#[wasm_bindgen_test]
fn compute_chunks_stability_test() {
    let data: Vec<u8> = test_data();
    let slice: &[u8] = &data;
    let min_size: u32 = 64;
    let avg_size: u32 = 256;
    let max_size: u32 = 1024;
    let offsets_1: Vec<u32> = compute_chunks(slice, min_size, avg_size, max_size).unwrap();
    let data_2 = test_data();
    let slice_2 = &data_2;
    let offsets_2: Vec<u32> = compute_chunks(slice_2, min_size, avg_size, max_size).unwrap();
    assert_eq!(offsets_1, offsets_2);
}

#[wasm_bindgen_test]
fn compute_chunks_bad_input_test() {
    let data: Vec<u8> = test_data();
    let slice: &[u8] = &data;
    let min_size: u32 = 32;
    let avg_size: u32 = 256;
    let max_size: u32 = 1024;
    let result: Result<Vec<u32>, RangeError> = compute_chunks(slice, min_size, avg_size, max_size);
    assert!(result.is_err());
    let expected_message: JsString = JsString::from("min_size should be >= 64");
    let expected_name: JsString = JsString::from("RangeError");
    let error = result.unwrap_err();
    assert_eq!(expected_name, error.name());
    assert_eq!(expected_message, error.message());
}

