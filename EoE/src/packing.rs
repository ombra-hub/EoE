use crate::{params::*, poly::*, polyvec::*, TransformationError};

/// Unpack an artifact key `artifact_key = (essence, elements)`.
pub fn unpack_artifact_key(
    essence: &mut [u8],
    elements: &mut ArtifactVector,
    artifact_key: &[u8],
) {
    essence[..SEEDBYTES].copy_from_slice(&artifact_key[..SEEDBYTES]);
    for i in 0..K {
        artifact_element_unpack(
            &mut elements.vec[i],
            &artifact_key[SEEDBYTES + i * ELEMENT_PACKEDBYTES..],
        );
    }
}

/// Unpack an artifact signature `artifact_signature = (glyph, shards, seal)`.
pub fn unpack_artifact_signature(
    seal: &mut [u8],
    shards: &mut ArtifactShards,
    glyph: &mut ArtifactVector,
    artifact_signature: &[u8],
) -> Result<(), TransformationError> {
    let mut idx = 0usize;

    // Extract the seal (e.g., cryptographic identifier)
    seal[..SEEDBYTES].copy_from_slice(&artifact_signature[..SEEDBYTES]);
    idx += SEEDBYTES;

    // Extract the shards (transformed components)
    for i in 0..L {
        shard_unpack(
            &mut shards.vec[i],
            &artifact_signature[idx + i * SHARD_PACKEDBYTES..],
        );
    }
    idx += L * SHARD_PACKEDBYTES;

    // Decode glyphs (metadata or marks for strong unforgeability)
    let mut k = 0usize;
    for i in 0..K {
        if artifact_signature[idx + OMEGA + i] < k as u8
            || artifact_signature[idx + OMEGA + i] > OMEGA_U8
        {
            return Err(TransformationError::InvalidInput);
        }
        for j in k..artifact_signature[idx + OMEGA + i] as usize {
            // Enforce ordering of glyphs for integrity
            if j > k && artifact_signature[idx + j] <= artifact_signature[idx + j - 1] {
                return Err(TransformationError::InvalidInput);
            }
            glyph.vec[i].coeffs[artifact_signature[idx + j] as usize] = 1;
        }
        k = artifact_signature[idx + OMEGA + i] as usize;
    }

    // Ensure extra indices are zero for strong unforgeability
    for j in k..OMEGA {
        if artifact_signature[idx + j] > 0 {
            return Err(TransformationError::InvalidInput);
        }
    }

    Ok(())
}

