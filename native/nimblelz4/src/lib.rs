extern crate lz4_flex;
extern crate rustler;

use std::fs;
use std::fs::File;
use std::path::Path;
use std::sync::Mutex;
use rustler::types::atom;
use rustler::types::binary::{Binary, OwnedBinary};
use rustler::{Encoder, Env, Error, Resource, ResourceArc, Term};

// Resource wrapper for the frame compressor
struct FrameCompressorResource {
    encoder: Mutex<Option<lz4_flex::frame::FrameEncoder<File>>>,
}

impl Resource for FrameCompressorResource {}


#[rustler::nif(schedule = "DirtyCpu")]
fn compress<'a>(env: Env<'a>, iolist_to_compress: Term<'a>) -> Result<Term<'a>, Error> {
    let binary_to_compress: Binary = Binary::from_iolist(iolist_to_compress).unwrap();
    let compressed_slice = lz4_flex::block::compress(binary_to_compress.as_slice());

    let mut erl_bin: OwnedBinary = OwnedBinary::new(compressed_slice.len()).unwrap();

    erl_bin
        .as_mut_slice()
        .copy_from_slice(compressed_slice.as_slice());

    Ok(erl_bin.release(env).encode(env))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn compress_frame<'a>(env: Env<'a>, iolist_to_compress: Term<'a>) -> Result<Term<'a>, Error> {
    let binary_to_compress: Binary = Binary::from_iolist(iolist_to_compress).unwrap();

    let mut compressor = lz4_flex::frame::FrameEncoder::new(Vec::new());
    std::io::Write::write(&mut compressor, binary_to_compress.as_slice()).unwrap();
    let compressed = compressor.finish().unwrap();

    let mut erl_bin: OwnedBinary = OwnedBinary::new(compressed.len()).unwrap();

    erl_bin
        .as_mut_slice()
        .copy_from_slice(compressed.as_slice());

    Ok(erl_bin.release(env).encode(env))
}

#[rustler::nif(schedule = "DirtyCpu")]
fn decompress<'a>(
    env: Env<'a>,
    binary_to_decompress: Binary,
    uncompressed_size: usize,
) -> Result<Term<'a>, Error> {
    match lz4_flex::block::decompress(binary_to_decompress.as_slice(), uncompressed_size) {
        Ok(decompressed_vec) => {
            let mut erl_bin: OwnedBinary = OwnedBinary::new(decompressed_vec.len()).unwrap();
            erl_bin
                .as_mut_slice()
                .copy_from_slice(decompressed_vec.as_slice());

            Ok((atom::ok(), erl_bin.release(env)).encode(env))
        }
        Err(decompress_err) => Ok((atom::error(), decompress_err.to_string()).encode(env)),
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn decompress_frame<'a>(env: Env<'a>, binary_to_decompress: Binary) -> Result<Term<'a>, Error> {
    let mut decompressed_buf: Vec<u8> = Vec::new();
    let mut decompressor = lz4_flex::frame::FrameDecoder::new(binary_to_decompress.as_slice());

    match std::io::Read::read_to_end(&mut decompressor, &mut decompressed_buf) {
        Ok(_) => {
            let mut erl_bin: OwnedBinary = OwnedBinary::new(decompressed_buf.len()).unwrap();
            erl_bin
                .as_mut_slice()
                .copy_from_slice(decompressed_buf.as_slice());

            Ok((atom::ok(), erl_bin.release(env)).encode(env))
        }
        Err(e) => Ok((atom::error(), e.to_string()).encode(env)),
    }
}

// New NIF functions for resource-based frame compression
#[rustler::nif]
fn create_frame_compressor_with_file_output<'a>(
    env: Env<'a>,
    output_path: &str,
) -> Result<Term<'a>, Error> {
    // Remove file if it exists
    if Path::new(output_path).exists() {
        if let Err(e) = fs::remove_file(output_path) {
            return Ok((atom::error(), format!("Failed to remove existing file: {}", e)).encode(env));
        }
    }

    // Create new file
    match File::create_new(output_path) {
        Ok(output) => {
            let encoder = lz4_flex::frame::FrameEncoder::new(output);
            let resource = ResourceArc::new(FrameCompressorResource {
                encoder: Mutex::new(Some(encoder)),
            });
            Ok((atom::ok(), resource).encode(env))
        }
        Err(e) => Ok((atom::error(), format!("Failed to create file: {}", e)).encode(env)),
    }
}

#[rustler::nif]
fn write_to_frame<'a>(
    env: Env<'a>,
    resource: ResourceArc<FrameCompressorResource>,
    chunk: Binary,
) -> Result<Term<'a>, Error> {
    let mut encoder_guard = resource.encoder.lock().unwrap();

    if let Some(ref mut encoder) = encoder_guard.as_mut() {
        match std::io::Write::write(encoder, chunk.as_slice()) {
            Ok(_) => Ok(atom::ok().encode(env)),
            Err(e) => Ok((atom::error(), format!("Write failed: {}", e)).encode(env)),
        }
    } else {
        Ok((atom::error(), "Compressor already finished or not initialized").encode(env))
    }
}

#[rustler::nif]
fn finish_frame<'a>(
    env: Env<'a>,
    resource: ResourceArc<FrameCompressorResource>,
) -> Result<Term<'a>, Error> {
    let mut encoder_guard = resource.encoder.lock().unwrap();

    if let Some(encoder) = encoder_guard.take() {
        match encoder.finish() {
            Ok(_) => Ok(atom::ok().encode(env)),
            Err(e) => Ok((atom::error(), format!("Finish failed: {}", e)).encode(env)),
        }
    } else {
        Ok((atom::error(), "Compressor already finished or not initialized").encode(env))
    }
}

fn load(env: Env, _: Term) -> bool {
    env.register::<FrameCompressorResource>().is_ok()
}

rustler::init!(
    "Elixir.NimbleLZ4",
    load = load,
    on_load = on_load
);