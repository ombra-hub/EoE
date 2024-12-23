use crate::params::*;
use crate::poly::*;

#[derive(Clone)]
pub struct ArtifactArrayK {
    pub elements: Box<[Artifact]>,
}

impl ArtifactArrayK {
    pub fn new(size: usize) -> Self {
        ArtifactArrayK {
            elements: vec![Artifact::default(); size].into_boxed_slice(),
        }
    }
}

impl Default for ArtifactArrayK {
    fn default() -> Self {
        ArtifactArrayK::new(K)
    }
}

#[derive(Clone)]
pub struct ArtifactArrayL {
    pub elements: Box<[Artifact]>,
}

impl ArtifactArrayL {
    pub fn new(size: usize) -> Self {
        ArtifactArrayL {
            elements: vec![Artifact::default(); size].into_boxed_slice(),
        }
    }
}

impl Default for ArtifactArrayL {
    fn default() -> Self {
        ArtifactArrayL::new(L)
    }
}

/// Expand an artifact matrix using a transformation seed.
/// Generates a matrix with uniformly random elements by performing rejection
/// sampling on SHAKE128(rho|j|i) or AES256CTR(rho,j|i).
pub fn artifact_matrix_expand(matrix: &mut [ArtifactArrayL], essence: &[u8]) {
    for i in 0..K {
        for j in 0..L {
            artifact_uniform(&mut matrix[i].elements[j], essence, ((i << 8) + j) as u16);
        }
    }
}

/// Perform pointwise Montgomery multiplication of an artifact matrix
/// with a vector and accumulate results.
pub fn artifact_matrix_pointwise_montgomery(
    result: &mut ArtifactArrayK,
    matrix: &[ArtifactArrayL],
    vector: &ArtifactArrayL,
) {
    for i in 0..K {
        artifact_array_pointwise_acc_montgomery(&mut result.elements[i], &matrix[i], vector);
    }
}

/// Forward NTT transformation for all artifacts in a vector of length L.
/// Output elements can be up to 16 * QUANTA larger than input elements.
pub fn artifact_array_l_ntt(vector: &mut ArtifactArrayL) {
    for i in 0..L {
        artifact_ntt(&mut vector.elements[i]);
    }
}

/// Pointwise multiplication and accumulation of two artifact vectors of length L.
/// Applies 2^{-32} scaling. Input/output vectors are in the NTT domain.
pub fn artifact_array_l_pointwise_acc_montgomery(
    result: &mut Artifact,
    u: &ArtifactArrayL,
    v: &ArtifactArrayL,
) {
    let mut temp = Artifact::default();
    artifact_pointwise_montgomery(result, &u.elements[0], &v.elements[0]);
    for i in 1..L {
        artifact_pointwise_montgomery(&mut temp, &u.elements[i], &v.elements[i]);
        artifact_add(result, &temp);
    }
}

/// Check the infinity norm of an artifact vector of length L.
/// Returns `0` if the norm is strictly less than `bound`, otherwise `1`.
pub fn artifact_array_l_chknorm(vector: &ArtifactArrayL, bound: i32) -> u8 {
    for i in 0..L {
        if artifact_chknorm(&vector.elements[i], bound) > 0 {
            return 1;
        }
    }
    0
}

//************************ Artifact Arrays of Length K **************************

/// Reduce all artifact elements in a vector of length K to representatives in [0, 2*QUANTA].
pub fn artifact_array_k_reduce(vector: &mut ArtifactArrayK) {
    for i in 0..K {
        artifact_reduce(&mut vector.elements[i]);
    }
}

/// Adjust all coefficients of artifacts in a vector of length K by adding QUANTA if negative.
pub fn artifact_array_k_caddq(vector: &mut ArtifactArrayK) {
    for i in 0..K {
        artifact_caddq(&mut vector.elements[i]);
    }
}

/// Subtract one artifact vector from another. Assumes coefficients in the second vector
/// are less than 2 * QUANTA. No modular reduction is performed.
pub fn artifact_array_k_sub(result: &mut ArtifactArrayK, vector: &ArtifactArrayK) {
    for i in 0..K {
        artifact_sub(&mut result.elements[i], &vector.elements[i]);
    }
}

/// Multiply an artifact vector of length K by 2^DEPTH without modular reduction.
pub fn artifact_array_k_shiftl(vector: &mut ArtifactArrayK) {
    for i in 0..K {
        artifact_shiftl(&mut vector.elements[i]);
    }
}

/// Perform forward NTT on all artifacts in a vector of length K.
pub fn artifact_array_k_ntt(vector: &mut ArtifactArrayK) {
    for i in 0..K {
        artifact_ntt(&mut vector.elements[i]);
    }
}

/// Perform inverse NTT and 2^{32} scaling on all artifacts in a vector of length K.
pub fn artifact_array_k_invntt_tomont(vector: &mut ArtifactArrayK) {
    for i in 0..K {
        artifact_invntt_tomont(&mut vector.elements[i]);
    }
}

/// Perform pointwise multiplication of an artifact vector by another vector and a scalar.
pub fn artifact_array_k_pointwise_poly_montgomery(
    result: &mut ArtifactArrayK,
    scalar: &Artifact,
    vector: &ArtifactArrayK,
) {
    for i in 0..K {
        artifact_pointwise_montgomery(&mut result.elements[i], scalar, &vector.elements[i]);
    }
}

/// Use a hint vector to adjust the high bits of an artifact vector.
pub fn artifact_array_k_use_hint(result: &mut ArtifactArrayK, hint: &ArtifactArrayK) {
    for i in 0..K {
        artifact_use_hint(&mut result.elements[i], &hint.elements[i]);
    }
}

/// Pack an artifact vector into a compact representation.
pub fn artifact_array_k_pack_w1(result: &mut [u8], vector: &ArtifactArrayK) {
    for i in 0..K {
        artifact_w1_pack(&mut result[i * POLYW1_PACKEDBYTES..], &vector.elements[i]);
    }
}

