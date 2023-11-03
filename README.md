# areion

An implementation of the [Areion permutations](https://eprint.iacr.org/2023/794.pdf) and some
potential uses.

## ⚠️ WARNING: You should not use this. ⚠️

This implementation passes test vectors, but Areion is a brand-new algorithm with no third-party
cryptanalysis to date.

## Correctness

The Areion512-MD tests don't pass because the actual algorithm is super underspecified in the paper
and no permutation I tried of "just swap SHA256's compression function" actually passed the test
vectors.

## Other Fun Stuff

The most interesting thing here is `areion512_mmo`, which is a Matyas-Meyer-Oseas construction
using a Single-key Even-Mansour construction to build a wide block cipher out of the Areion512
permutation.

## Performance

In general, it's a little faster than properly-accelerated SHA256 on aarch64 and about 1.7x as fast
as SHA256 on x86_64.

### `x86_64` (GCP `n2-standard-4`, Intel Ice Lake, rustc 1.73 `+aes,+ssse3`)

```text
Timer precision: 16.92 ns
benchmarks        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ areion512_mmo                │               │               │               │         │
│  ├─ 16          37.21 ns      │ 2.139 µs      │ 37.65 ns      │ 37.98 ns      │ 530856  │ 16987392
│  │              429.8 MB/s    │ 7.476 MB/s    │ 424.9 MB/s    │ 421.2 MB/s    │         │
│  ├─ 256         127.5 ns      │ 3.832 µs      │ 128.9 ns      │ 129.7 ns      │ 375750  │ 6012000
│  │              2.006 GB/s    │ 66.79 MB/s    │ 1.984 GB/s    │ 1.972 GB/s    │         │
│  ├─ 1024        451.8 ns      │ 16.12 µs      │ 457.7 ns      │ 460.3 ns      │ 470297  │ 1881188
│  │              2.266 GB/s    │ 63.51 MB/s    │ 2.236 GB/s    │ 2.224 GB/s    │         │
│  ├─ 16384       5.695 µs      │ 100.4 µs      │ 6.957 µs      │ 7.012 µs      │ 138743  │ 138743
│  │              2.876 GB/s    │ 163.1 MB/s    │ 2.354 GB/s    │ 2.336 GB/s    │         │
│  ╰─ 1048576     359 µs        │ 512.6 µs      │ 444.3 µs      │ 444.7 µs      │ 2159    │ 2159
│                 2.92 GB/s     │ 2.045 GB/s    │ 2.359 GB/s    │ 2.357 GB/s    │         │
├─ sha256                       │               │               │               │         │
│  ├─ 16          83.54 ns      │ 79.86 µs      │ 88.15 ns      │ 89.19 ns      │ 5195440 │ 5195440
│  │              191.5 MB/s    │ 200.3 KB/s    │ 181.4 MB/s    │ 179.3 MB/s    │         │
│  ├─ 256         257.1 ns      │ 21.16 µs      │ 265.4 ns      │ 267.2 ns      │ 771949  │ 3087796
│  │              995.3 MB/s    │ 12.09 MB/s    │ 964.3 MB/s    │ 957.8 MB/s    │         │
│  ├─ 1024        840 ns        │ 36.18 µs      │ 859.3 ns      │ 863.7 ns      │ 522565  │ 1045130
│  │              1.218 GB/s    │ 28.29 MB/s    │ 1.191 GB/s    │ 1.185 GB/s    │         │
│  ├─ 16384       12.43 µs      │ 76.12 µs      │ 12.67 µs      │ 12.72 µs      │ 77398   │ 77398
│  │              1.317 GB/s    │ 215.2 MB/s    │ 1.292 GB/s    │ 1.287 GB/s    │         │
│  ╰─ 1048576     800.2 µs      │ 919.5 µs      │ 808 µs        │ 809.3 µs      │ 1208    │ 1208
│                 1.31 GB/s     │ 1.14 GB/s     │ 1.297 GB/s    │ 1.295 GB/s    │         │
╰─ sha512                       │               │               │               │         │
   ├─ 16          298.9 ns      │ 82.98 µs      │ 303.5 ns      │ 306 ns        │ 2455372 │ 2455372
   │              53.52 MB/s    │ 192.8 KB/s    │ 52.71 MB/s    │ 52.27 MB/s    │         │
   ├─ 256         802.7 ns      │ 73.19 µs      │ 832.7 ns      │ 840.7 ns      │ 1026310 │ 1026310
   │              318.8 MB/s    │ 3.497 MB/s    │ 307.4 MB/s    │ 304.4 MB/s    │         │
   ├─ 1024        2.295 µs      │ 22.89 µs      │ 2.35 µs       │ 2.365 µs      │ 399040  │ 399040
   │              446 MB/s      │ 44.72 MB/s    │ 435.6 MB/s    │ 432.9 MB/s    │         │
   ├─ 16384       32.15 µs      │ 172.7 µs      │ 32.51 µs      │ 32.7 µs       │ 30382   │ 30382
   │              509.5 MB/s    │ 94.82 MB/s    │ 503.9 MB/s    │ 500.8 MB/s    │         │
   ╰─ 1048576     2.054 ms      │ 2.297 ms      │ 2.068 ms      │ 2.072 ms      │ 479     │ 479
                  510.2 MB/s    │ 456.4 MB/s    │ 506.8 MB/s    │ 506 MB/s      │         │
```

### `aarch64` (Apple M2 Air 2022, rustc 1.73)

```text
Timer precision: 41.66 ns
benchmarks        fastest       │ slowest       │ median        │ mean          │ samples │ iters
├─ areion512_mmo                │               │               │               │         │
│  ├─ 16          28.17 ns      │ 208.8 ns      │ 28.66 ns      │ 28.89 ns      │ 55179   │ 14125824
│  │              567.8 MB/s    │ 76.61 MB/s    │ 558.2 MB/s    │ 553.7 MB/s    │         │
│  ├─ 256         92.13 ns      │ 948.2 ns      │ 94.09 ns      │ 95.3 ns       │ 113087  │ 7237568
│  │              2.778 GB/s    │ 269.9 MB/s    │ 2.72 GB/s     │ 2.686 GB/s    │         │
│  ├─ 1024        364.2 ns      │ 8.379 µs      │ 379.9 ns      │ 385.6 ns      │ 286476  │ 2291808
│  │              2.811 GB/s    │ 122.1 MB/s    │ 2.695 GB/s    │ 2.655 GB/s    │         │
│  ├─ 16384       5.958 µs      │ 59.12 µs      │ 6.083 µs      │ 6.141 µs      │ 157373  │ 157373
│  │              2.749 GB/s    │ 277.1 MB/s    │ 2.693 GB/s    │ 2.667 GB/s    │         │
│  ╰─ 1048576     390.2 µs      │ 622 µs        │ 390.4 µs      │ 396.4 µs      │ 2484    │ 2484
│                 2.686 GB/s    │ 1.685 GB/s    │ 2.685 GB/s    │ 2.644 GB/s    │         │
├─ sha256                       │               │               │               │         │
│  ├─ 16          23.78 ns      │ 8.746 µs      │ 30.94 ns      │ 30.07 ns      │ 106385  │ 13617280
│  │              672.8 MB/s    │ 1.829 MB/s    │ 517 MB/s      │ 531.9 MB/s    │         │
│  ├─ 256         124.6 ns      │ 1.924 µs      │ 128.5 ns      │ 135.2 ns      │ 174901  │ 5596832
│  │              2.053 GB/s    │ 133 MB/s      │ 1.99 GB/s     │ 1.893 GB/s    │         │
│  ├─ 1024        450.2 ns      │ 3.648 µs      │ 458 ns        │ 469.2 ns      │ 120514  │ 1928224
│  │              2.274 GB/s    │ 280.6 MB/s    │ 2.235 GB/s    │ 2.182 GB/s    │         │
│  ├─ 16384       6.916 µs      │ 60.74 µs      │ 7.041 µs      │ 7.117 µs      │ 136614  │ 136614
│  │              2.368 GB/s    │ 269.6 MB/s    │ 2.326 GB/s    │ 2.301 GB/s    │         │
│  ╰─ 1048576     449.3 µs      │ 534.4 µs      │ 449.7 µs      │ 455.2 µs      │ 2168    │ 2168
│                 2.333 GB/s    │ 1.961 GB/s    │ 2.331 GB/s    │ 2.303 GB/s    │         │
╰─ sha512                       │               │               │               │         │
   ├─ 16          124.6 ns      │ 36.41 µs      │ 208 ns        │ 213.7 ns      │ 3895853 │ 3895853
   │              128.3 MB/s    │ 439.3 KB/s    │ 76.91 MB/s    │ 74.84 MB/s    │         │
   ├─ 256         614.2 ns      │ 6.676 µs      │ 629.9 ns      │ 636.5 ns      │ 182638  │ 1461104
   │              416.7 MB/s    │ 38.34 MB/s    │ 406.4 MB/s    │ 402.1 MB/s    │         │
   ├─ 1024        1.791 µs      │ 57.83 µs      │ 1.916 µs      │ 1.936 µs      │ 503592  │ 503592
   │              571.6 MB/s    │ 17.7 MB/s     │ 534.3 MB/s    │ 528.8 MB/s    │         │
   ├─ 16384       26.95 µs      │ 89.29 µs      │ 27.12 µs      │ 27.43 µs      │ 36193   │ 36193
   │              607.7 MB/s    │ 183.4 MB/s    │ 604 MB/s      │ 597.2 MB/s    │         │
   ╰─ 1048576     1.72 ms       │ 1.944 ms      │ 1.732 ms      │ 1.74 ms       │ 573     │ 573
                  609.3 MB/s    │ 539.3 MB/s    │ 605.1 MB/s    │ 602.3 MB/s    │         │
```

## License

Copyright © 2023 Coda Hale

Distributed under the Apache License 2.0 or MIT License.
