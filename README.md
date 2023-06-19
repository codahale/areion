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

On my M2 Air:

```text
perm512                 time:   [11.859 ns 11.896 ns 11.935 ns]
areion256-dm            time:   [6.4671 ns 6.4873 ns 6.5065 ns]
areion512-dm            time:   [13.964 ns 13.985 ns 14.005 ns]
areion512-md/64         time:   [73.009 ns 73.153 ns 73.324 ns]
                        thrpt:  [832.40 MiB/s 834.35 MiB/s 836.00 MiB/s]
areion512-md/512        time:   [534.05 ns 534.85 ns 535.87 ns]
                        thrpt:  [911.19 MiB/s 912.93 MiB/s 914.30 MiB/s]
areion512-md/1024       time:   [1.0586 µs 1.0597 µs 1.0613 µs]
                        thrpt:  [920.17 MiB/s 921.55 MiB/s 922.49 MiB/s]
areion512-md/10240      time:   [10.522 µs 10.531 µs 10.544 µs]
                        thrpt:  [926.17 MiB/s 927.29 MiB/s 928.15 MiB/s]
areion512-md/1048576    time:   [1.0761 ms 1.0769 ms 1.0778 ms]
                        thrpt:  [927.81 MiB/s 928.61 MiB/s 929.28 MiB/s]
sha256/64               time:   [72.390 ns 72.642 ns 72.903 ns]
                        thrpt:  [837.21 MiB/s 840.22 MiB/s 843.14 MiB/s]
sha256/512              time:   [262.43 ns 262.74 ns 263.13 ns]
                        thrpt:  [1.8122 GiB/s 1.8149 GiB/s 1.8170 GiB/s]
sha256/1024             time:   [486.30 ns 489.50 ns 493.22 ns]
                        thrpt:  [1.9336 GiB/s 1.9482 GiB/s 1.9611 GiB/s]
sha256/10240            time:   [4.4727 µs 4.4861 µs 4.4989 µs]
                        thrpt:  [2.1198 GiB/s 2.1259 GiB/s 2.1322 GiB/s]
sha256/1048576          time:   [460.83 µs 474.19 µs 489.07 µs]
                        thrpt:  [1.9968 GiB/s 2.0594 GiB/s 2.1191 GiB/s]
```

It's not faster than a properly-accelerated SHA256 implementation on NEON, but it's much faster than
an unaccelerated implementation.

## License

Copyright © 2023 Coda Hale

Distributed under the Apache License 2.0 or MIT License.
