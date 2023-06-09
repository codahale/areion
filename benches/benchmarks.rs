use areion::{perm256, perm512};
use std::arch::aarch64::vmovq_n_u8;

use criterion::{criterion_group, criterion_main, Criterion};

fn perm(c: &mut Criterion) {
    unsafe {
        let mut x0 = vmovq_n_u8(0);
        let mut x1 = vmovq_n_u8(0);
        let mut x2 = vmovq_n_u8(0);
        let mut x3 = vmovq_n_u8(0);
        c.bench_function("perm256", |b| b.iter(|| perm256(&mut x0, &mut x1)));
        c.bench_function("perm512", |b| {
            b.iter(|| perm512(&mut x0, &mut x1, &mut x2, &mut x3))
        });
    }
}

criterion_group!(all, perm);
criterion_main!(all);
