//! Criterion benchmarks for all 9 `gen_*_v0` ISCC functions.
//!
//! Uses representative inline inputs (conformance-derived and synthetic) to
//! measure latency and throughput of the core ISCC generation functions.

use criterion::{BenchmarkId, Criterion, Throughput, black_box, criterion_group, criterion_main};
use iscc_lib::{
    DataHasher, alg_cdc_chunks, gen_audio_code_v0, gen_data_code_v0, gen_image_code_v0,
    gen_instance_code_v0, gen_iscc_code_v0, gen_meta_code_v0, gen_mixed_code_v0, gen_text_code_v0,
    gen_video_code_v0,
};

/// Generate a deterministic byte buffer of the given size.
fn deterministic_bytes(size: usize) -> Vec<u8> {
    (0..size).map(|i| (i % 256) as u8).collect()
}

/// Generate a synthetic text string of approximately the given character count.
fn synthetic_text(chars: usize) -> String {
    let base = "The quick brown fox jumps over the lazy dog. ";
    base.repeat((chars / base.len()) + 1)[..chars].to_string()
}

/// Benchmark `gen_meta_code_v0` with name-only and name+description inputs.
fn bench_meta_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_meta_code_v0");

    group.bench_function("name_only", |b| {
        b.iter(|| {
            gen_meta_code_v0(
                black_box("Die Unendliche Geschichte"),
                black_box(None),
                black_box(None),
                black_box(64),
            )
            .unwrap()
        })
    });

    group.bench_function("name+desc", |b| {
        b.iter(|| {
            gen_meta_code_v0(
                black_box("Die Unendliche Geschichte"),
                black_box(Some("Von Michael Ende")),
                black_box(None),
                black_box(64),
            )
            .unwrap()
        })
    });

    group.finish();
}

/// Benchmark `gen_text_code_v0` with a ~1000-character synthetic text.
fn bench_text_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_text_code_v0");
    let text = synthetic_text(1000);

    group.bench_function("1000_chars", |b| {
        b.iter(|| gen_text_code_v0(black_box(&text), black_box(64)).unwrap())
    });

    group.finish();
}

/// Benchmark `gen_image_code_v0` with a 1024-byte gradient pixel array.
fn bench_image_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_image_code_v0");
    let pixels: Vec<u8> = (0..1024).map(|i| (i % 256) as u8).collect();

    group.bench_function("32x32_gradient", |b| {
        b.iter(|| gen_image_code_v0(black_box(&pixels), black_box(64)).unwrap())
    });

    group.finish();
}

/// Benchmark `gen_audio_code_v0` with a 300-element sequential feature vector.
fn bench_audio_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_audio_code_v0");
    let cv: Vec<i32> = (0..300).collect();

    group.bench_function("300_features", |b| {
        b.iter(|| gen_audio_code_v0(black_box(&cv), black_box(64)).unwrap())
    });

    group.finish();
}

/// Benchmark `gen_video_code_v0` with 10 frames of 380-element vectors.
fn bench_video_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_video_code_v0");
    let frames: Vec<Vec<i32>> = (0..10)
        .map(|f: i32| (0..380).map(move |i| f * 380 + i).collect())
        .collect();

    group.bench_function("10x380_frames", |b| {
        b.iter(|| gen_video_code_v0(black_box(&frames), black_box(64)).unwrap())
    });

    group.finish();
}

/// Benchmark `gen_mixed_code_v0` with Content-Code strings from conformance tests.
fn bench_mixed_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_mixed_code_v0");
    let codes = ["EUA6GIKXN42IQV3S", "EIAUKMOUIOYZCKA5"];

    group.bench_function("2_content_codes", |b| {
        let code_refs: Vec<&str> = codes.to_vec();
        b.iter(|| gen_mixed_code_v0(black_box(&code_refs), black_box(64)).unwrap())
    });

    group.finish();
}

/// Benchmark `gen_data_code_v0` at 64KB and 1MB sizes.
fn bench_data_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_data_code_v0");

    let data_64k = deterministic_bytes(64 * 1024);
    group.throughput(Throughput::Bytes(data_64k.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("throughput", "64KB"),
        &data_64k,
        |b, data| b.iter(|| gen_data_code_v0(black_box(data), black_box(64)).unwrap()),
    );

    let data_1m = deterministic_bytes(1024 * 1024);
    group.throughput(Throughput::Bytes(data_1m.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("throughput", "1MB"),
        &data_1m,
        |b, data| b.iter(|| gen_data_code_v0(black_box(data), black_box(64)).unwrap()),
    );

    group.finish();
}

/// Benchmark `gen_instance_code_v0` at 64KB and 1MB sizes.
fn bench_instance_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_instance_code_v0");

    let data_64k = deterministic_bytes(64 * 1024);
    group.throughput(Throughput::Bytes(data_64k.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("throughput", "64KB"),
        &data_64k,
        |b, data| b.iter(|| gen_instance_code_v0(black_box(data), black_box(64)).unwrap()),
    );

    let data_1m = deterministic_bytes(1024 * 1024);
    group.throughput(Throughput::Bytes(data_1m.len() as u64));
    group.bench_with_input(
        BenchmarkId::new("throughput", "1MB"),
        &data_1m,
        |b, data| b.iter(|| gen_instance_code_v0(black_box(data), black_box(64)).unwrap()),
    );

    group.finish();
}

/// Benchmark `gen_iscc_code_v0` with 4 ISCC unit strings from conformance tests.
fn bench_iscc_code(c: &mut Criterion) {
    let mut group = c.benchmark_group("gen_iscc_code_v0");
    let codes = [
        "AAAYPXW445FTYNJ3",
        "EAARMJLTQCUWAND2",
        "GABVVC5DMJJGYKZ4ZBYVNYABFFYXG",
        "IADWIK7A7JTUAQ2D6QARX7OBEIK3OOUAM42LOBLCZ4ZOGDLRHMDL6TQ",
    ];

    group.bench_function("4_units", |b| {
        let code_refs: Vec<&str> = codes.to_vec();
        b.iter(|| gen_iscc_code_v0(black_box(&code_refs), black_box(false)).unwrap())
    });

    group.finish();
}

/// Benchmark `DataHasher` streaming with 64 KiB update chunks on 1 MB data.
fn bench_data_hasher_streaming(c: &mut Criterion) {
    let mut group = c.benchmark_group("DataHasher");
    let data = deterministic_bytes(1024 * 1024);
    let chunk_size = 64 * 1024;

    group.throughput(Throughput::Bytes(data.len() as u64));
    group.bench_function("streaming_1MB_64KB_chunks", |b| {
        b.iter(|| {
            let mut dh = DataHasher::new();
            for chunk in data.chunks(chunk_size) {
                dh.update(black_box(chunk));
            }
            dh.finalize(black_box(64)).unwrap()
        })
    });

    group.finish();
}

/// Benchmark `alg_cdc_chunks` directly at various data sizes.
fn bench_cdc_chunks(c: &mut Criterion) {
    let mut group = c.benchmark_group("alg_cdc_chunks");

    for &size in &[4 * 1024, 64 * 1024, 1024 * 1024] {
        let data = deterministic_bytes(size);
        let label = match size {
            4096 => "4KB",
            65536 => "64KB",
            1048576 => "1MB",
            _ => unreachable!(),
        };
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::new("throughput", label), &data, |b, data| {
            b.iter(|| alg_cdc_chunks(black_box(data), black_box(false), black_box(1024)))
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_meta_code,
    bench_text_code,
    bench_image_code,
    bench_audio_code,
    bench_video_code,
    bench_mixed_code,
    bench_data_code,
    bench_instance_code,
    bench_iscc_code,
    bench_data_hasher_streaming,
    bench_cdc_chunks,
);
criterion_main!(benches);
