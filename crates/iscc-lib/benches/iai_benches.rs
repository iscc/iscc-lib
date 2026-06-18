//! `iai-callgrind` instruction-count benchmarks for `iscc-lib`'s hot CPU paths.
//!
//! These benchmarks feed the v1.0.0 performance-regression gate (issue #3). Unlike the
//! wall-clock criterion suite in `benchmarks.rs`, callgrind measures instruction counts,
//! which are deterministic and stable across machines — ideal for a CI regression gate.
//!
//! The harness itself compiles without valgrind or `iai-callgrind-runner`; only *running*
//! the benches (`cargo bench --bench iai_benches`) needs valgrind, which the follow-up CI
//! slice installs on a valgrind-enabled Linux runner. Run locally with valgrind present via
//! `cargo bench -p iscc-lib --bench iai_benches`.
//!
//! Each `#[library_benchmark]` returns `black_box(...)` of the call result so the optimizer
//! cannot elide the work. Input construction lives in `#[bench::id(expr)]` argument
//! expressions, which iai-callgrind evaluates in the unmeasured setup phase — so only the
//! ISCC computation itself is counted.
//!
//! The `#[library_benchmark]` functions use plain `//` comments rather than `///` docstrings:
//! the proc macro iterates every attribute on the function and aborts on any it does not
//! recognize, and a `///` comment lowers to a `#[doc = "..."]` attribute that it rejects.
//! Helper functions (not macro-annotated) keep ordinary `///` docstrings.
//!
//! `gen_sum_code_v0` is intentionally deferred: it is a file-I/O wrapper whose instruction
//! count is syscall-dominated and would need an iai setup closure to create the temp file
//! outside the measured region. It is covered by the criterion suite for wall-clock timing.

use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use iscc_lib::{
    AudioCodeResult, DataCodeResult, ImageCodeResult, InstanceCodeResult, IsccCodeResult,
    MetaCodeResult, MixedCodeResult, TextCodeResult, VideoCodeResult, alg_cdc_chunks,
    alg_minhash_256, gen_audio_code_v0, gen_data_code_v0, gen_image_code_v0, gen_instance_code_v0,
    gen_iscc_code_v0, gen_meta_code_v0, gen_mixed_code_v0, gen_text_code_v0, gen_video_code_v0,
};
use std::hint::black_box;

/// Generate a deterministic byte buffer of the given size.
fn deterministic_bytes(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Generate a synthetic text string of approximately the given character count.
fn synthetic_text(chars: usize) -> String {
    let base = "The quick brown fox jumps over the lazy dog. ";
    base.repeat((chars / base.len()) + 1)[..chars].to_string()
}

/// Generate a spread-out feature vector of the given length for MinHash.
fn deterministic_features(count: usize) -> Vec<u32> {
    (0..count as u32)
        .map(|i| i.wrapping_mul(2_654_435_761))
        .collect()
}

// Benchmark `gen_meta_code_v0` with name-only and name+description inputs.
#[library_benchmark]
#[bench::name_only("Die Unendliche Geschichte", None)]
#[bench::name_desc("Die Unendliche Geschichte", Some("Von Michael Ende"))]
fn bench_meta_code(name: &str, description: Option<&str>) -> MetaCodeResult {
    black_box(
        gen_meta_code_v0(
            black_box(name),
            black_box(description),
            black_box(None),
            black_box(64),
        )
        .unwrap(),
    )
}

// Benchmark `gen_text_code_v0` with a ~1000-character synthetic text.
#[library_benchmark]
#[bench::chars_1000(synthetic_text(1000))]
fn bench_text_code(text: String) -> TextCodeResult {
    black_box(gen_text_code_v0(black_box(&text), black_box(64)).unwrap())
}

// Benchmark `gen_image_code_v0` with a 1024-byte gradient pixel array.
#[library_benchmark]
#[bench::gradient_1024(deterministic_bytes(1024))]
fn bench_image_code(pixels: Vec<u8>) -> ImageCodeResult {
    black_box(gen_image_code_v0(black_box(&pixels), black_box(64)).unwrap())
}

// Benchmark `gen_audio_code_v0` with a 300-element sequential feature vector.
#[library_benchmark]
#[bench::features_300((0..300).collect::<Vec<i32>>())]
fn bench_audio_code(cv: Vec<i32>) -> AudioCodeResult {
    black_box(gen_audio_code_v0(black_box(&cv), black_box(64)).unwrap())
}

// Benchmark `gen_video_code_v0` with 10 frames of 380-element vectors.
#[library_benchmark]
#[bench::frames_10x380((0..10).map(|f: i32| (0..380).map(move |i| f * 380 + i).collect::<Vec<i32>>()).collect::<Vec<Vec<i32>>>())]
fn bench_video_code(frame_sigs: Vec<Vec<i32>>) -> VideoCodeResult {
    black_box(gen_video_code_v0(black_box(&frame_sigs), black_box(64)).unwrap())
}

// Benchmark `gen_mixed_code_v0` with Content-Code strings from conformance tests.
#[library_benchmark]
#[bench::two_codes(vec!["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"])]
fn bench_mixed_code(codes: Vec<&'static str>) -> MixedCodeResult {
    black_box(gen_mixed_code_v0(black_box(&codes), black_box(64)).unwrap())
}

// Benchmark `gen_data_code_v0` at 64KB and 1MB sizes.
#[library_benchmark]
#[bench::bytes_64k(deterministic_bytes(64 * 1024))]
#[bench::bytes_1m(deterministic_bytes(1024 * 1024))]
fn bench_data_code(data: Vec<u8>) -> DataCodeResult {
    black_box(gen_data_code_v0(black_box(&data), black_box(64)).unwrap())
}

// Benchmark `gen_instance_code_v0` at 64KB and 1MB sizes.
#[library_benchmark]
#[bench::bytes_64k(deterministic_bytes(64 * 1024))]
#[bench::bytes_1m(deterministic_bytes(1024 * 1024))]
fn bench_instance_code(data: Vec<u8>) -> InstanceCodeResult {
    black_box(gen_instance_code_v0(black_box(&data), black_box(64)).unwrap())
}

// Benchmark `gen_iscc_code_v0` with 4 ISCC unit strings from conformance tests.
#[library_benchmark]
#[bench::four_units(vec![
    "AAAYPXW445FTYNJ3",
    "EAARMJLTQCUWAND2",
    "GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
    "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ",
])]
fn bench_iscc_code(codes: Vec<&'static str>) -> IsccCodeResult {
    black_box(gen_iscc_code_v0(black_box(&codes), black_box(false)).unwrap())
}

// Benchmark `alg_cdc_chunks` directly at 4KB, 64KB, and 1MB data sizes.
//
// Returns the chunk count (the `Vec<&[u8]>` borrows from `data`, so it cannot escape the
// function); the heavy chunking work is fully measured before the length is taken.
#[library_benchmark]
#[bench::bytes_4k(deterministic_bytes(4 * 1024))]
#[bench::bytes_64k(deterministic_bytes(64 * 1024))]
#[bench::bytes_1m(deterministic_bytes(1024 * 1024))]
fn bench_cdc_chunks(data: Vec<u8>) -> usize {
    black_box(
        alg_cdc_chunks(black_box(&data), black_box(false), black_box(1024))
            .unwrap()
            .len(),
    )
}

// Benchmark `alg_minhash_256` directly with a 1024-element feature vector.
#[library_benchmark]
#[bench::features_1024(deterministic_features(1024))]
fn bench_minhash_256(features: Vec<u32>) -> Vec<u8> {
    black_box(alg_minhash_256(black_box(&features)))
}

library_benchmark_group!(
    name = iscc_benches;
    benchmarks =
        bench_meta_code,
        bench_text_code,
        bench_image_code,
        bench_audio_code,
        bench_video_code,
        bench_mixed_code,
        bench_data_code,
        bench_instance_code,
        bench_iscc_code,
        bench_cdc_chunks,
        bench_minhash_256,
);

main!(library_benchmark_groups = iscc_benches);
