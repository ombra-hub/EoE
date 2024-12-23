use crate::{
    fips202::*, ntt::*, params::*, reduce::*, rounding::*, symmetric::*,
};

#[derive(Clone)]
pub struct Artifact {
    pub elements: Box<[i32]>,
}

impl Artifact {
    pub fn new(size: usize) -> Self {
        Artifact {
            elements: vec![0; size].into_boxed_slice(),
        }
    }
}

impl Default for Artifact {
    fn default() -> Self {
        Artifact::new(ELEMENTS)
    }
}

/// In-place reduction of all artifact elements to a range [0, 2*QUANTA].
pub fn artifact_reduce(a: &mut Artifact) {
    for i in 0..a.elements.len() {
        a.elements[i] = reduce32(a.elements[i]);
    }
}

/// Adjust all artifact elements by adding QUANTA if the element is negative.
pub fn artifact_caddq(a: &mut Artifact) {
    for i in 0..a.elements.len() {
        a.elements[i] = caddq(a.elements[i]);
    }
}

/// Add two artifacts. No modular reduction is performed.
pub fn artifact_add(c: &mut Artifact, b: &Artifact) {
    for i in 0..c.elements.len() {
        c.elements[i] = c.elements[i] + b.elements[i];
    }
}

/// Subtract one artifact from another. Assumes elements of the second input
/// artifact are less than 2*QUANTA. No modular reduction is performed.
pub fn artifact_sub(c: &mut Artifact, b: &Artifact) {
    for i in 0..c.elements.len() {
        c.elements[i] = c.elements[i] - b.elements[i];
    }
}

/// Multiply artifact elements by 2^DEPTH without modular reduction.
/// Assumes input elements are less than 2^{32-DEPTH}.
pub fn artifact_shiftl(a: &mut Artifact) {
    for i in 0..a.elements.len() {
        a.elements[i] <<= DEPTH;
    }
}

/// In-place forward transformation in the NTT domain.
/// Output elements can be up to 16*QUANTA larger than input elements.
pub fn artifact_ntt(a: &mut Artifact) {
    ntt(&mut a.elements);
}

/// In-place inverse NTT and scaling by 2^{32}.
/// Input elements need to be less than 2*QUANTA.
/// Output elements are less than 2*QUANTA.
pub fn artifact_invntt_tomont(a: &mut Artifact) {
    invntt_tomont(&mut a.elements);
}

/// Pointwise multiplication of artifacts in the NTT domain,
/// followed by scaling by 2^{-32}. Output elements are less than 2*QUANTA
/// if input elements are less than 22*QUANTA.
pub fn artifact_pointwise_montgomery(
    c: &mut Artifact,
    a: &Artifact,
    b: &Artifact,
) {
    for i in 0..ELEMENTS {
        c.elements[i] = montgomery_reduce((a.elements[i] as i64) * b.elements[i] as i64);
    }
}

/// Use a hint artifact to correct the high bits of another artifact.
pub fn artifact_use_hint(corrected: &mut Artifact, hint: &Artifact) {
    for i in 0..ELEMENTS {
        corrected.elements[i] = use_hint(corrected.elements[i], hint.elements[i] as u8);
    }
}

/// Check if the infinity norm of an artifact is within the given bound.
/// Returns `0` if norm is smaller than `bound`, and `1` otherwise.
pub fn artifact_chknorm(a: &Artifact, bound: i32) -> u8 {
    if bound > (QUANTA_I32 - 1) / 8 {
        return 1;
    }
    for i in 0..ELEMENTS {
        let mut t = a.elements[i] >> 31;
        t = a.elements[i] - (t & 2 * a.elements[i]);
        if t >= bound {
            return 1;
        }
    }
    0
}

/// Sample uniformly random artifact elements in the range [0, QUANTA-1].
pub fn artifact_sample_uniform(
    elements: &mut [i32],
    len: u32,
    buf: &[u8],
    buflen: usize,
) -> u32 {
    let (mut count, mut pos) = (0usize, 0usize);
    while count < len as usize && pos + 3 <= buflen {
        let mut value = buf[pos] as u32;
        pos += 1;
        value |= (buf[pos] as u32) << 8;
        pos += 1;
        value |= (buf[pos] as u32) << 16;
        pos += 1;
        value &= 0x7FFFFF;
        if value < QUANTA as u32 {
            elements[count] = value as i32;
            count += 1;
        }
    }
    count as u32
}

/// Generate an artifact with uniformly random elements in [0, QUANTA-1].
pub fn artifact_uniform(a: &mut Artifact, seed: &[u8], nonce: u16) {
    let mut buflen = POLY_UNIFORM_NBLOCKS * STREAM128_BLOCKBYTES;
    let mut buf = [0u8; POLY_UNIFORM_NBLOCKS * STREAM128_BLOCKBYTES + 2];
    let mut state = Stream128State::default();
    stream128_init(&mut state, seed, nonce);
    stream128_squeezeblocks(&mut buf, POLY_UNIFORM_NBLOCKS as u64, &mut state);
    let mut count = artifact_sample_uniform(&mut a.elements, ELEMENTS_U32, &buf, buflen);
    let mut offset;
    while count < ELEMENTS_U32 {
        offset = buflen % 3;
        for i in 0..offset {
            buf[i] = buf[buflen - offset + i];
        }
        buflen = STREAM128_BLOCKBYTES + offset;
        stream128_squeezeblocks(&mut buf[offset..], 1, &mut state);
        count += artifact_sample_uniform(&mut a.elements[(count as usize)..], ELEMENTS_U32 - count, &buf, buflen);
    }
}

