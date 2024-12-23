use crate::{
    fips202::*, packing::*, params::*, poly::*, polyvec::*, ValidationError,
};

/// **Stage 1**: Unpack the artifact's key and prepare essential parameters.
pub fn artifact_verify_stage1(
    signature: &[u8],
    artifact_key: &[u8],
) -> Result<(Box<[u8; SEEDBYTES]>, Box<ArtifactArrayK>), ValidationError> {
    if signature.len() != SIGNBYTES {
        return Err(ValidationError::Input);
    }

    let mut essence = Box::new([0u8; SEEDBYTES]);
    let mut elements = Box::new(ArtifactArrayK::default());

    unpack_pk(&mut *essence, &mut *elements, artifact_key);

    Ok((essence, elements))
}

/// **Stage 2**: Decompose the signature into its core components and perform checks.
pub fn artifact_verify_stage2(
    signature: &[u8],
) -> Result<(Box<[u8; SEEDBYTES]>, Box<ArtifactArrayL>, Box<ArtifactArrayK>), ValidationError> {
    let mut seal = Box::new([0u8; SEEDBYTES]);
    let mut fragments = Box::new(ArtifactArrayL::default());
    let mut glyphs = Box::new(ArtifactArrayK::default());

    unpack_sig(&mut *seal, &mut *fragments, &mut *glyphs, signature)?;

    if artifact_array_l_chknorm(&fragments, (GAMMA1 - BETA) as i32) > 0 {
        return Err(ValidationError::Input);
    }

    Ok((seal, fragments, glyphs))
}

/// **Stage 3**: Compute the compressed representation of the artifact's essence.
pub fn artifact_verify_stage3(
    artifact_key: &[u8],
    message: &[u8],
) -> Box<[u8; CRHBYTES]> {
    let mut digest = Box::new([0u8; CRHBYTES]);
    let mut state = Box::new(KeccakState::default());

    shake256(&mut *digest, SEEDBYTES, artifact_key, PUBLICKEYBYTES);
    shake256_absorb(&mut *state, &*digest, SEEDBYTES);
    shake256_absorb(&mut *state, message, message.len());
    shake256_finalize(&mut *state);
    shake256_squeeze(&mut *digest, CRHBYTES, &mut *state);

    digest
}

/// **Stage 4**: Generate a challenge artifact.
pub fn artifact_verify_stage4(
    seal: &mut [u8; SEEDBYTES],
) -> Box<Artifact> {
    let mut challenge = Box::new(Artifact::default());
    artifact_challenge(&mut *challenge, seal);
    challenge
}

/// **Stage 5**: Expand the artifact matrix.
pub fn artifact_verify_stage5(
    essence: &mut [u8; SEEDBYTES],
) -> Box<[ArtifactArrayL; 4]> {
    let mut matrix = Box::new([
        ArtifactArrayL::default(),
        ArtifactArrayL::default(),
        ArtifactArrayL::default(),
        ArtifactArrayL::default(),
    ]);

    artifact_matrix_expand(&mut *matrix, essence);
    matrix
}

/// **Stage 6**: Compute the transformed artifact array.
pub fn artifact_verify_stage6(
    challenge: Box<Artifact>,
    matrix: Box<[ArtifactArrayL; 4]>,
    fragments: Box<ArtifactArrayL>,
    elements: Box<ArtifactArrayK>,
) -> Result<Box<ArtifactArrayK>, ValidationError> {
    let mut transformed = Box::new(ArtifactArrayK::default());
    artifact_array_l_ntt(&mut *fragments);
    artifact_matrix_pointwise_montgomery(&mut *transformed, &*matrix, &*fragments);
    artifact_ntt(&mut *challenge);
    artifact_array_k_shiftl(&mut *elements);
    artifact_array_k_ntt(&mut *elements);

    let cloned_elements = Box::new((*elements).clone());
    artifact_array_k_pointwise_poly_montgomery(&mut *elements, &*challenge, &*cloned_elements);

    Ok(transformed)
}

/// **Stage 7**: Refine the transformed artifact array.
pub fn artifact_verify_stage7(
    mut transformed: Box<ArtifactArrayK>,
    elements: ArtifactArrayK,
) -> Result<Box<ArtifactArrayK>, ValidationError> {
    artifact_array_k_sub(&mut *transformed, &elements);
    artifact_array_k_reduce(&mut *transformed);
    artifact_array_k_invntt_tomont(&mut *transformed);

    Ok(transformed)
}

/// **Stage 8**: Finalize the artifact validation and ensure integrity.
pub fn artifact_verify_stage8(
    mut buffer: Box<[u8; K * POLYW1_PACKEDBYTES]>,
    mut transformed: Box<ArtifactArrayK>,
    glyphs: &ArtifactArrayK,
    digest: Box<[u8; CRHBYTES]>,
    seal: &Box<[u8; SEEDBYTES]>,
    computed_seal: &mut [u8; SEEDBYTES],
) -> Result<(), ValidationError> {
    artifact_array_k_caddq(&mut *transformed);
    artifact_array_k_use_hint(&mut *transformed, glyphs);
    artifact_array_k_pack_w1(&mut *buffer, &*transformed);

    let mut state = Box::new(KeccakState::default());
    state.init();
    shake256_absorb(&mut *state, &*digest, CRHBYTES);
    shake256_absorb(&mut *state, &*buffer, K * POLYW1_PACKEDBYTES);
    shake256_finalize(&mut *state);
    shake256_squeeze(computed_seal, SEEDBYTES, &mut *state);

    if &**seal != computed_seal {
        Err(ValidationError::Verify)
    } else {
        Ok(())
    }
}

/// **Full Artifact Verification Process**.
pub fn artifact_verify(
    signature: &[u8],
    message: &[u8],
    artifact_key: &[u8],
) -> Result<(), ValidationError> {
    let (essence, elements) = artifact_verify_stage1(signature, artifact_key)?;
    let (mut seal, fragments, glyphs) = artifact_verify_stage2(signature)?;
    let digest = artifact_verify_stage3(artifact_key, message);

    let transformed = validate_matrix(essence, &mut seal, fragments, &elements)?;
    finalize_verification(transformed, elements, glyphs, digest, seal)
}

fn validate_matrix(
    essence: Box<[u8; SEEDBYTES]>,
    seal: &mut Box<[u8; SEEDBYTES]>,
    fragments: Box<ArtifactArrayL>,
    elements: &Box<ArtifactArrayK>,
) -> Result<Box<ArtifactArrayK>, ValidationError> {
    let challenge = artifact_verify_stage4(&mut **seal);
    let matrix = artifact_verify_stage5(&mut *essence);
    artifact_verify_stage6(challenge, matrix, fragments, elements.clone())
}

fn finalize_verification(
    transformed: Box<ArtifactArrayK>,
    elements: Box<ArtifactArrayK>,
    glyphs: Box<ArtifactArrayK>,
    digest: Box<[u8; CRHBYTES]>,
    seal: Box<[u8; SEEDBYTES]>,
) -> Result<(), ValidationError> {
    let refined = artifact_verify_stage7(transformed, *elements)?;
    let buffer = Box::new([0u8; K * POLYW1_PACKEDBYTES]);
    let mut computed_seal = Box::new([0u8; SEEDBYTES]);
    artifact_verify_stage8(buffer, refined, &glyphs, digest, &seal, &mut *computed_seal)?;

    Ok(())
}

