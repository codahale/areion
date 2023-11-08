# areion

An implementation of the [Areion permutations](https://eprint.iacr.org/2023/794.pdf) and some
potential uses.

## ⚠️ WARNING: You should not use this. ⚠️

This implementation passes test vectors, but Areion is a brand-new algorithm with no third-party
cryptanalysis to date.

## Correctness

This implementation is compliant with the test vectors in the updated version of the paper
(2023-09-21).

## Other Fun Stuff

This repository also includes a few different hash algorithms based on the Areion512 permutation.

### Areion-256-Sponge

Areion-256-Sponge uses the Areion512 permutation in a sponge construction with a 256-bit rate. On
x86_64 platforms, it's roughly as fast as vectorized SHA-256; on Apple Silicon it's about half as
fast.

### Areion-512-MMO

Areion-512-MMO is a Matyas-Meyer-Oseas hash function using a single-key Even-Mansour block cipher
based on the Areion-512 permutation.

The [single-key Even-Mansour](https://eprint.iacr.org/2011/541.pdf) scheme uses a public permutation
`P` to construct a block cipher for key `K` and plaintext block `M`:

```text
SEM(K, M) = P(M ^ K) ^ K
```

The Matyas-Meyer-Oseas mode uses a block cipher `E(K, M)` to construct a hash function, calculating
the current hash state `H_i` given the previous hash state `H_{i-1}` and current message block
`M_i`:

```text
MMO(H_{i-1}, M_i) = E(H_{i-1}, M_i) ^ M_i
```

These can be combined into a single form:

```text
SEM-MMO(H_{i-1}, M_i) = P(M_i ^ H_{i-1}) ^ H_{i-1} ^ M_i
```

Areion-512-MMO uses four 128-bit words, initialized with the same constants as SHA-512:

```text
H_0 = (0x6a09e667f3bcc908bb67ae8584caa73b, 0x3c6ef372fe94f82ba54ff53a5f1d36f1,
       0x510e527fade682d19b05688c2b3e6c1f, 0x1f83d9abfb41bd6b5be0cd19137e2179)
```

It then iterates through the message in 512-bit blocks, updating the state words using the `MMO-SEM`
compression function. To produce a final digest, the same padding as SHA-512 is used (i.e. appending
an`0x80` byte, padding to the nearest block, and appending a 128-bit big-endian counter of the
message length in bits), and the final state words are serialized in big-endian form.

The resulting hash function offers 256 bits of collision resistance if the permutation `P` (i.e.
Areion-512) is indistinguishable from a random permutation. Untruncated digests are vulnerable to
length-extension attacks. With dedicated AES and 128-bit vector instructions, performance is ~1.7x
that of vectorized SHA-256 on x86_64 processors and ~1.1x that of fully-accelerated SHA-256 on Apple
Silicon processors.

### Areion-512-HAIFA

Areion-512-HAIFA is a HAIFA-style hash function based on the Areion512 permutation, allowing for
variable digest lengths (0..64 bytes), and immune to length extension attacks.

Areion-512-MMO uses four 128-bit words, initialized with the same constants as SHA-512, with the
final word XORed with the output size in bits:

```text
H_0 = (0x6a09e667f3bcc908bb67ae8584caa73b, 0x3c6ef372fe94f82ba54ff53a5f1d36f1,
       0x510e527fade682d19b05688c2b3e6c1f, 0x1f83d9abfb41bd6b5be0cd19137e2179 ^ output_size)
```

Its compression function uses a 128-bit counter of the number of bits which have been processed,
including the current block:

```text
C(H, M, #bits) = P(H ^ M ^ #bits) ^ H ^ M
```

The resulting hash function offers 256 bits of collision resistance if the permutation `P` (i.e.
Areion-512) is indistinguishable from a random permutation. Untruncated digests are vulnerable to
length-extension attacks. With dedicated AES and 128-bit vector instructions, performance is ~1.7x
that of vectorized SHA-256 on x86_64 processors and ~1.2x that of fully-accelerated SHA-256 on Apple
Silicon processors.

## License

Copyright © 2023 Coda Hale

Distributed under the Apache License 2.0 or MIT License.
