use areion::{digest::Digest, Areion512Md, Areion512Mmo};
use areion::{Areion256Sponge, AreionHaifa512};
use criterion::{criterion_group, criterion_main, BatchSize, Criterion, Throughput};
use sha2::{Sha256, Sha512};

fn areion256(c: &mut Criterion) {
    c.bench_function("areion256", |b| {
        b.iter_batched(
            || (areion::load(&[0u8; 16]), areion::load(&[0u8; 16])),
            |(x0, x1)| areion::areion256(x0, x1),
            BatchSize::SmallInput,
        );
    });
}

fn areion512(c: &mut Criterion) {
    c.bench_function("areion512", |b| {
        b.iter_batched(
            || {
                (
                    areion::load(&[0u8; 16]),
                    areion::load(&[0u8; 16]),
                    areion::load(&[0u8; 16]),
                    areion::load(&[0u8; 16]),
                )
            },
            |(x0, x1, x2, x3)| areion::areion512(x0, x1, x2, x3),
            BatchSize::SmallInput,
        );
    });
}

fn areion256_dm(c: &mut Criterion) {
    c.bench_function("areion256_dm", |b| {
        b.iter_batched(
            || (areion::load(&[0u8; 16]), areion::load(&[0u8; 16])),
            |(x0, x1)| areion::areion256_dm(x0, x1),
            BatchSize::SmallInput,
        );
    });
}

fn areion512_dm(c: &mut Criterion) {
    c.bench_function("areion512_dm", |b| {
        b.iter_batched(
            || {
                (
                    areion::load(&[0u8; 16]),
                    areion::load(&[0u8; 16]),
                    areion::load(&[0u8; 16]),
                    areion::load(&[0u8; 16]),
                )
            },
            |(x0, x1, x2, x3)| areion::areion512_dm(x0, x1, x2, x3),
            BatchSize::SmallInput,
        );
    });
}

const LENS: &[(usize, &str)] =
    &[(16, "16B"), (256, "256B"), (1024, "1KiB"), (16 * 1024, "16KiB"), (1024 * 1024, "1MiB")];

fn areion512_md(c: &mut Criterion) {
    let mut g = c.benchmark_group("areion512_md");
    for &(len, id) in LENS {
        let input = vec![0u8; len];
        g.throughput(Throughput::Bytes(len as u64));
        g.bench_function(id, |b| {
            b.iter(|| Areion512Md::default().chain_update(&input).finalize());
        });
    }
    g.finish();
}

fn areion512_mmo(c: &mut Criterion) {
    let mut g = c.benchmark_group("areion512_mmo");
    for &(len, id) in LENS {
        let input = vec![0u8; len];
        g.throughput(Throughput::Bytes(len as u64));
        g.bench_function(id, |b| {
            b.iter(|| Areion512Mmo::default().chain_update(&input).finalize());
        });
    }
    g.finish();
}

fn areion256_sponge(c: &mut Criterion) {
    let mut g = c.benchmark_group("areion256_sponge");
    for &(len, id) in LENS {
        let input = vec![0u8; len];
        g.throughput(Throughput::Bytes(len as u64));
        g.bench_function(id, |b| {
            b.iter(|| Areion256Sponge::default().chain_update(&input).finalize());
        });
    }
    g.finish();
}

fn areion512_haifa(c: &mut Criterion) {
    let mut g = c.benchmark_group("areion512_haifa");
    for &(len, id) in LENS {
        let input = vec![0u8; len];
        g.throughput(Throughput::Bytes(len as u64));
        g.bench_function(id, |b| {
            b.iter(|| AreionHaifa512::default().chain_update(&input).finalize());
        });
    }
    g.finish();
}

fn sha256(c: &mut Criterion) {
    let mut g = c.benchmark_group("sha256");
    for &(len, id) in LENS {
        let input = vec![0u8; len];
        g.throughput(Throughput::Bytes(len as u64));
        g.bench_function(id, |b| {
            b.iter(|| Sha256::default().chain_update(&input).finalize());
        });
    }
    g.finish();
}

fn sha512(c: &mut Criterion) {
    let mut g = c.benchmark_group("sha512");
    for &(len, id) in LENS {
        let input = vec![0u8; len];
        g.throughput(Throughput::Bytes(len as u64));
        g.bench_function(id, |b| {
            b.iter(|| Sha512::default().chain_update(&input).finalize());
        });
    }
    g.finish();
}

fn blake3(c: &mut Criterion) {
    let mut g = c.benchmark_group("blake3");
    for &(len, id) in LENS {
        let input = vec![0u8; len];
        g.throughput(Throughput::Bytes(len as u64));
        g.bench_function(id, |b| {
            b.iter(|| ::blake3::hash(&input));
        });
    }
    g.finish();
}

criterion_group!(
    benches,
    areion256,
    areion512,
    areion256_dm,
    areion512_dm,
    areion512_md,
    areion512_mmo,
    areion256_sponge,
    areion512_haifa,
    sha256,
    sha512,
    blake3,
);
criterion_main!(benches);
