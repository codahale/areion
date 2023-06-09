# areion

An `aarch64` implementation of the [Areion permutations](https://eprint.iacr.org/2023/794.pdf).

## ⚠️ WARNING: You should not use this. ⚠️

This implementation passes test vectors, but Areion is a brand-new algorithm with no third-party
cryptanalysis to date.

## Performance

On my M2 Air:

```text
perm256                 time:   [6.5158 ns 6.5373 ns 6.5629 ns]
perm512                 time:   [12.172 ns 12.181 ns 12.190 ns]
areion256-dm            time:   [6.5103 ns 6.5158 ns 6.5214 ns]
areion512-dm            time:   [13.866 ns 13.892 ns 13.929 ns]
```

## License

Copyright © 2023 Coda Hale

Distributed under the Apache License 2.0 or MIT License.
