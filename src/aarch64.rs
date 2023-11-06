use core::arch::aarch64::*;
use core::arch::asm;

pub use core::arch::aarch64::uint8x16_t as AesBlock;

#[inline]
pub fn load_64x2(a: u64, b: u64) -> AesBlock {
    unsafe { vreinterpretq_u8_u64(vsetq_lane_u64(b, vmovq_n_u64(a), 1)) }
}

#[inline]
pub fn load_32x4(a: u32, b: u32, c: u32, d: u32) -> AesBlock {
    unsafe {
        vreinterpretq_u8_u32(vsetq_lane_u32::<3>(
            d,
            vsetq_lane_u32::<2>(c, vsetq_lane_u32::<1>(b, vmovq_n_u32(a))),
        ))
    }
}

#[inline]
pub fn zero() -> AesBlock {
    unsafe { vmovq_n_u8(0) }
}

#[inline]
pub fn load(bytes: &[u8]) -> AesBlock {
    unsafe { vld1q_u8(bytes.as_ptr()) }
}

#[inline]
pub fn store(bytes: &mut [u8], block: AesBlock) {
    unsafe { vst1q_u8(bytes.as_mut_ptr(), block) };
}

#[inline]
pub fn store_u32(bytes: &mut [u32], block: AesBlock) {
    unsafe { vst1q_u32(bytes.as_mut_ptr(), vreinterpretq_u32_u8(block)) };
}

#[inline]
pub fn xor(a: AesBlock, b: AesBlock) -> AesBlock {
    unsafe { veorq_u8(a, b) }
}

#[inline]
pub fn xor3(a: AesBlock, b: AesBlock, c: AesBlock) -> AesBlock {
    // TODO replace with veor3q_u8 intrinsic when that's stable
    #[target_feature(enable = "sha3")]
    unsafe fn veor3q_u8(mut a: AesBlock, b: AesBlock, c: AesBlock) -> AesBlock {
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
pub fn enc(state: AesBlock, round_key: AesBlock) -> AesBlock {
    unsafe { veorq_u8(vaesmcq_u8(vaeseq_u8(state, zero())), round_key) }
}

#[inline]
pub fn enc_last(state: AesBlock, round_key: AesBlock) -> AesBlock {
    unsafe { xor(vaeseq_u8(state, zero()), round_key) }
}

#[inline]
pub fn dec_last(state: AesBlock, round_key: AesBlock) -> AesBlock {
    unsafe { xor(vaesdq_u8(state, zero()), round_key) }
}

#[inline]
pub fn inv_mix(state: AesBlock) -> AesBlock {
    unsafe { vaesimcq_u8(state) }
}
