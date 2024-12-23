use crate::params::*;

/// Decompose an artifact element into high and low fragments.
/// 
/// Splits an artifact element `a` into `low_fragment` and `high_fragment` such that:
/// `a mod^+ QUANTA = high_fragment * ALPHA + low_fragment`
/// where `-ALPHA/2 < low_fragment <= ALPHA/2`, except in special cases:
/// - If `high_fragment = (QUANTA - 1) / ALPHA`, it is set to 0, and
/// `-ALPHA/2 <= low_fragment = a mod^+ QUANTA - QUANTA < 0`.
/// 
/// Assumes `a` is a standard artifact representative.
/// 
/// Returns `high_fragment`.
pub fn artifact_decompose(low_fragment: &mut i32, a: i32) -> i32 {
    let mut high_fragment = (a + 127) >> 7;
    if GAMMA2 == (QUANTA - 1) / 32 {
        high_fragment = (high_fragment * 1025 + (1 << 21)) >> 22;
        high_fragment &= 15;
    } else if GAMMA2 == (QUANTA - 1) / 88 {
        high_fragment = (high_fragment * 11275 + (1 << 23)) >> 24;
        high_fragment ^= ((43 - high_fragment) >> 31) & high_fragment;
    }
    *low_fragment = a - high_fragment * 2 * GAMMA2_I32;
    *low_fragment -= (((QUANTA_I32 - 1) / 2 - *low_fragment) >> 31) & QUANTA_I32;
    high_fragment
}

/// Adjust the high fragments of an artifact using a hint.
///
/// Based on a provided `hint`, modifies the high fragments of an artifact element to
/// maintain alignment with the defined transformation rules.
///
/// Returns the corrected high fragments.
pub fn artifact_use_hint(a: i32, hint: u8) -> i32 {
    let mut low_fragment = 0i32;
    let high_fragment = artifact_decompose(&mut low_fragment, a);
    if hint == 0 {
        return high_fragment;
    }

    if GAMMA2 == (QUANTA - 1) / 32 {
        if low_fragment > 0 {
            return (high_fragment + 1) & 15;
        } else {
            return (high_fragment - 1) & 15;
        }
    } else {
        if low_fragment > 0 {
            if high_fragment == 43 {
                return 0;
            } else {
                return high_fragment + 1;
            }
        } else {
            if high_fragment == 0 {
                return 43;
            } else {
                return high_fragment - 1;
            }
        }
    }
}

