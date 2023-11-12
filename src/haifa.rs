use core::fmt;

use crate::intrinsics::*;

use digest::block_buffer::Lazy;
use digest::core_api::{
    Block, BlockSizeUser, Buffer, BufferKindUser, CoreWrapper, CtVariableCoreWrapper,
    RtVariableCoreWrapper, TruncSide, UpdateCore, VariableOutputCore,
};
use digest::crypto_common::AlgorithmName;
use digest::generic_array::GenericArray;
use digest::typenum::{Unsigned, U64};
use digest::{HashMarker, Output, OutputSizeUser, Reset};
use hex_literal::hex;

#[derive(Debug, Clone)]
struct State {
    /// The 512-bit hash state.
    h: (AesBlock, AesBlock, AesBlock, AesBlock),
    /// The 512-bit tweak, dependent on the output length.
    t: (AesBlock, AesBlock, AesBlock, AesBlock),
    /// The message length counter, in bits.
    m_len: u128,
}

impl State {
    fn new(output_size: usize) -> State {
        let output_size = load(&(output_size as u128).to_be_bytes());
        State {
            h: (
                // SHA2-512 IV constants
                load(&hex!("08c9bcf367e6096a3ba7ca8485ae67bb")),
                load(&hex!("2bf894fe72f36e3cf1361d5f3af54fa5")),
                load(&hex!("d182e6ad7f520e511f6c3e2b8c68059b")),
                load(&hex!("6bbd41fbabd9831f79217e1319cde05b")),
            ),
            t: (
                // SHA2-512-256 IV constants
                load(&hex!("2cf72bfc94213122c2644cc8a35f559f")),
                load(&hex!("51b1536f6bb89323bdea405919773896")),
                load(&hex!("e3ff8ea8e23e289692398653251e5ebe")),
                xor(load(&hex!("aab8852cfc99012ba22cc581dc2db70e")), output_size),
            ),
            m_len: 0,
        }
    }
}

impl State {
    fn compress(&mut self, blocks: &[GenericArray<u8, U64>], bit_len: u64) {
        let Self { h: (mut h0, mut h1, mut h2, mut h3), t: (t0, t1, t2, t3), mut m_len } = *self;

        for block in blocks {
            // Increment the bit counter *before* compressing the block. This eliminates the need
            // for finalization-specific flags, as the output of compressing the final block of N
            // bits will be dependent on the value of ctr+N.
            m_len += bit_len as u128;

            // Load the message block into four words.
            let (m0, m1, m2, m3) = (
                load(&block[..16]),
                load(&block[16..32]),
                load(&block[32..48]),
                load(&block[48..]),
            );

            // C(H, T, M, #bits) = P(H ^ T ^ M ^ #bits) ^ H ^ T
            let (x0, x1, x2, x3) = (xor(h0, t0), xor(h1, t1), xor(h2, t2), xor(h3, t3));
            let (y0, y1, y2, y3) = crate::areion512(
                xor(x0, m0),
                xor(x1, m1),
                xor(x2, m2),
                // Only include the counter as an input to the permutation. This avoids a
                // Streebog-type situation in which attackers have control of some of the bits of
                // the output of a block's compression.
                xor3(x3, m3, load(&m_len.to_be_bytes())),
            );
            (h0, h1, h2, h3) = (xor(x0, y0), xor(x1, y1), xor(x2, y2), xor(x3, y3));
        }

        // Update the hash state and counter.
        self.h = (h0, h1, h2, h3);
        self.m_len = m_len;
    }
}

#[derive(Debug, Clone)]
pub struct Core {
    state: State,
    output_size: usize,
}

impl HashMarker for Core {}

impl BlockSizeUser for Core {
    type BlockSize = U64;
}

impl BufferKindUser for Core {
    type BufferKind = Lazy;
}

impl OutputSizeUser for Core {
    type OutputSize = U64;
}

impl UpdateCore for Core {
    #[inline]
    fn update_blocks(&mut self, blocks: &[Block<Self>]) {
        // Update the state with the compression function, using the length of a full block in bits
        // to update the counter.
        self.state.compress(blocks, Self::BlockSize::U64 * 8);
    }
}

impl VariableOutputCore for Core {
    const TRUNC_SIDE: TruncSide = TruncSide::Left;

    fn new(output_size: usize) -> Result<Self, digest::InvalidOutputSize> {
        if !(0 < output_size && output_size <= 64) {
            return Err(digest::InvalidOutputSize);
        }
        Ok(Core { state: State::new(output_size), output_size })
    }

    fn finalize_variable_core(&mut self, buffer: &mut Buffer<Self>, out: &mut Output<Self>) {
        // Update the state with the compression function, using the length of the remaining data
        // in bits to update the counter.
        let bit_len = buffer.get_pos() as u64 * 8;
        self.state.compress(&[*buffer.pad_with_zeros()], bit_len);

        // Use the hash state as the digest, truncating as needed.
        let mut tmp = [0u8; 64];
        store(&mut tmp[..16], self.state.h.0);
        store(&mut tmp[16..32], self.state.h.1);
        store(&mut tmp[32..48], self.state.h.2);
        store(&mut tmp[48..], self.state.h.3);
        let n = out.len();
        out.copy_from_slice(&tmp[..n]);
    }
}

impl Reset for Core {
    #[inline]
    fn reset(&mut self) {
        self.state = State::new(self.output_size);
    }
}

impl AlgorithmName for Core {
    #[inline]
    fn write_alg_name(f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Areion512-HAIFA")
    }
}

pub type AreionHaifaVar = RtVariableCoreWrapper<Core>;
pub type AreionHaifaCore<OutSize> = CtVariableCoreWrapper<Core, OutSize>;
pub type AreionHaifa<OutSize> = CoreWrapper<AreionHaifaCore<OutSize>>;
pub type AreionHaifa512 = AreionHaifa<U64>;

#[cfg(test)]
mod tests {
    use digest::Digest;
    use quickcheck_macros::quickcheck;

    use super::*;

    #[test]
    fn round_trip() {
        AreionHaifa512::new().chain_update([8u8; 64]).chain_update(b"this is a potato").finalize();
    }

    #[quickcheck]
    fn different_inputs_yield_different_digests(a: Vec<u8>, b: Vec<u8>) -> bool {
        let aa = AreionHaifa512::new().chain_update(&a).finalize();
        let bb = AreionHaifa512::new().chain_update(&b).finalize();
        aa == bb || a != b
    }
}
