# areion

An `aarch64` implementation of the [Areion permutations](https://eprint.iacr.org/2023/794.pdf) and
some potential uses.

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

In general, it's slower than properly-accelerated SHA256 on both aarch64 and x86_64. On the M1, the
MMO hash is ~1.9 GB/sec, but SHA256 (using the `sha2` feature) is ~2.1 GB/sec. On x86_64, it's much
slower.

## License

Copyright © 2023 Coda Hale

Distributed under the Apache License 2.0 or MIT License.
