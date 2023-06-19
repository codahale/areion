use areion::{areion256, areion256_dm, areion512, areion512_dm, areion512_md, areion512_mmo};
use sha2::{Digest, Sha256};
use std::arch::aarch64::vmovq_n_u8;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};

fn perm(c: &mut Criterion) {
    unsafe {
        let x0 = vmovq_n_u8(0);
        let x1 = vmovq_n_u8(0);
        let x2 = vmovq_n_u8(0);
        let x3 = vmovq_n_u8(0);
        c.bench_function("perm256", |b| b.iter(|| areion256(x0, x1)));
        c.bench_function("perm512", |b| b.iter(|| areion512(x0, x1, x2, x3)));
    }
}

fn dm(c: &mut Criterion) {
    unsafe {
        let x0 = vmovq_n_u8(0);
        let x1 = vmovq_n_u8(0);
        let x2 = vmovq_n_u8(0);
        let x3 = vmovq_n_u8(0);
        c.bench_function("areion256-dm", |b| b.iter(|| areion256_dm(x0, x1)));
        c.bench_function("areion512-dm", |b| b.iter(|| areion512_dm(x0, x1, x2, x3)));
    }
}

fn md(c: &mut Criterion) {
    let mut g = c.benchmark_group("areion512-md");
    for size in [64, 512, 1024, 1024 * 10, 1024 * 1024] {
        g.throughput(criterion::Throughput::Bytes(size as u64));
        g.bench_function(BenchmarkId::from_parameter(size), |b| {
            let data = vec![0u8; size];
            b.iter(|| areion512_md(&data))
        });
    }
    g.finish()
}

fn mmo(c: &mut Criterion) {
    let mut g = c.benchmark_group("areion512-mmo");
    for size in [64, 512, 1024, 1024 * 10, 1024 * 1024] {
        g.throughput(criterion::Throughput::Bytes(size as u64));
        g.bench_function(BenchmarkId::from_parameter(size), |b| {
            let data = vec![0u8; size];
            b.iter(|| areion512_mmo(&data))
        });
    }
    g.finish()
}

fn sha256(c: &mut Criterion) {
    let mut g = c.benchmark_group("sha256");
    for size in [64, 512, 1024, 1024 * 10, 1024 * 1024] {
        g.throughput(criterion::Throughput::Bytes(size as u64));
        g.bench_function(BenchmarkId::from_parameter(size), |b| {
            let data = vec![0u8; size];
            b.iter(|| Sha256::new().chain_update(&data).finalize())
        });
    }
    g.finish()
}

criterion_group!(all, perm, dm, md, sha256, mmo);
criterion_main!(all);
