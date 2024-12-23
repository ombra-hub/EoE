use crate::params::*;

pub const QUANTA_INVERSE: i32 = 58728449; // QUANTA^(-1) mod 2^32

/// For an artifact element `a` in the range -2^{31} * QUANTA <= a <= QUANTA * 2^31,
/// compute `r` ≡ a * 2^{-32} (mod QUANTA) such that -QUANTA < r < QUANTA.
///
/// Returns the reduced artifact element `r`.
pub fn artifact_montgomery_reduce(a: i64) -> i32 {
    let mut t = (a as i32).wrapping_mul(QUANTA_INVERSE) as i64;
    t = (a - t * QUANTA as i64) >> 32;
    t as i32
}

/// For an artifact element `a` in the range a <= 2^{31} - 2^{22} - 1,
/// compute `r` ≡ a (mod QUANTA) such that -6283009 <= r <= 6283007.
///
/// Returns the reduced artifact element `r`.
pub fn artifact_reduce32(a: i32) -> i32 {
    let mut t = (a + (1 << 22)) >> 23;
    t = a - t * QUANTA as i32;
    t
}

/// Add QUANTA if the input artifact element is negative.
///
/// Returns the adjusted artifact element `r`.
pub fn artifact_caddq(a: i32) -> i32 {
    a + ((a >> 31) & QUANTA as i32)
}

