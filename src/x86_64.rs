use core::arch::x86_64::*;

pub use core::arch::x86_64::__m128i as Block;

#[inline(always)]
pub fn load(bytes: &[u8]) -> Block {
    unsafe { _mm_loadu_si128(bytes.as_ptr() as *const __m128i) }
}

#[inline(always)]
pub fn store(bytes: &mut [u8], block: Block) {
    unsafe { _mm_storeu_si128(bytes.as_mut_ptr() as *mut __m128i, block) };
}

#[inline(always)]
pub fn xor(a: Block, b: Block) -> Block {
    unsafe { _mm_xor_si128(a, b) }
}

#[inline(always)]
pub fn xor3(a: Block, b: Block, c: Block) -> Block {
    unsafe { _mm_xor_si128(a, _mm_xor_si128(b, c)) }
}

#[inline(always)]
pub fn enc(state: Block, round_key: Block) -> Block {
    unsafe { _mm_aesenc_si128(state, round_key) }
}

#[inline(always)]
pub fn enc_last(state: Block, round_key: Block) -> Block {
    unsafe { _mm_aesenclast_si128(state, round_key) }
}

#[inline(always)]
pub fn dec(state: Block, round_key: Block) -> Block {
    unsafe { _mm_aesdec_si128(state, round_key) }
}

#[inline(always)]
pub fn dec_last(state: Block, round_key: Block) -> Block {
    unsafe { _mm_aesdeclast_si128(state, round_key) }
}

#[inline(always)]
pub fn inv_mix(state: Block) -> Block {
    unsafe { _mm_aesimc_si128(state) }
}
