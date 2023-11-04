use core::arch::aarch64::*;
use core::arch::asm;

pub use core::arch::aarch64::uint8x16_t as Block;

#[inline]
pub fn load_64x2(a: u64, b: u64) -> Block {
    unsafe { vreinterpretq_u8_u64(vsetq_lane_u64(b, vmovq_n_u64(a), 1)) }
}

#[inline]
pub fn load_32x4(a: u32, b: u32, c: u32, d: u32) -> uint8x16_t {
    unsafe {
        vreinterpretq_u8_u32(vsetq_lane_u32::<3>(
            d,
            vsetq_lane_u32::<2>(c, vsetq_lane_u32::<1>(b, vmovq_n_u32(a))),
        ))
    }
}

#[inline]
pub fn zero() -> uint8x16_t {
    unsafe { vmovq_n_u8(0) }
}

#[inline]
pub fn load(bytes: &[u8]) -> uint8x16_t {
    unsafe { vld1q_u8(bytes.as_ptr()) }
}

#[inline]
pub fn store(bytes: &mut [u8], block: uint8x16_t) {
    unsafe { vst1q_u8(bytes.as_mut_ptr(), block) };
}

#[inline]
pub fn xor(a: uint8x16_t, b: uint8x16_t) -> uint8x16_t {
    unsafe { veorq_u8(a, b) }
}

#[inline]
pub fn xor3(a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
    // TODO replace with veor3q_u8 intrinsic when that's stable
    #[target_feature(enable = "sha3")]
    unsafe fn veor3q_u8(mut a: uint8x16_t, b: uint8x16_t, c: uint8x16_t) -> uint8x16_t {
        asm!(
            "EOR3 {0:v}.16B, {0:v}.16B, {1:v}.16B, {2:v}.16B",
            inlateout(vreg) a, in(vreg) b, in(vreg) c,
            options(pure, nomem, nostack, preserves_flags)
        );
        a
    }
    unsafe { veor3q_u8(a, b, c) }
}

/// Perform one AES round on the given state using the given round key.
#[inline]
pub fn enc(state: Block, round_key: Block) -> Block {
    unsafe { veorq_u8(vaesmcq_u8(vaeseq_u8(state, zero())), round_key) }
}

#[inline]
pub fn enc_last(state: uint8x16_t, round_key: uint8x16_t) -> uint8x16_t {
    unsafe { xor(vaeseq_u8(state, zero()), round_key) }
}

#[inline]
pub fn dec_last(state: uint8x16_t, round_key: uint8x16_t) -> uint8x16_t {
    unsafe { xor(vaesdq_u8(state, zero()), round_key) }
}

#[inline]
pub fn inv_mix(state: uint8x16_t) -> uint8x16_t {
    unsafe { vaesimcq_u8(state) }
}
