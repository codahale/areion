# areion

An `aarch64` implementation of the [Areion permutations](https://eprint.iacr.org/2023/794.pdf).

## ⚠️ WARNING: You should not use this. ⚠️

This implementation passes test vectors, but Areion is a brand-new algorithm with no third-party
cryptanalysis to date.

## Performance

On my M2 Air:

```text
perm256                 time:   [32.132 ns 32.164 ns 32.204 ns]
perm512                 time:   [26.095 ns 26.150 ns 26.222 ns]
```

## License

Copyright © 2023 Coda Hale

Distributed under the Apache License 2.0 or MIT License.
