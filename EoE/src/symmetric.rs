use crate::fips202::*;
use crate::params::{CRHBYTES, SEEDBYTES};

/// Represents the state of the Stream of Eternity, responsible for generating artifact fragments.
pub type EternityStreamState = KeccakState;

/// Defines the block size for the Stream of Eternity's essence.
pub const STREAM_BLOCKBYTES: usize = SHAKE128_RATE;

/// Computes the Compressed Relic Hash (CRH) for an artifact.
///
/// This function generates a fixed-length digest for the provided artifact essence.
pub fn compute_crh(digest: &mut [u8], essence: &[u8], essence_length: usize) {
    shake256(digest, CRHBYTES, essence, essence_length);
}

/// Initializes the Stream of Eternity with the artifact's essence and nonce.
///
/// This prepares the stream for generating uniformly random artifact components.
pub fn eternity_stream_init(
    state: &mut EternityStreamState,
    essence: &[u8],
    relic_nonce: u16,
) {
    absorb_eternity_stream(state, essence, relic_nonce);
}

/// Extracts multiple artifact blocks from the Stream of Eternity.
///
/// This function generates a specified number of blocks containing random artifact data.
pub fn eternity_stream_squeeze_blocks(
    output: &mut [u8],
    num_blocks: u64,
    state: &mut EternityStreamState,
) {
    shake128_squeezeblocks(output, num_blocks as usize, state);
}

/// Initializes the absorption of the Stream of Eternity.
///
/// Combines the artifact's essence and relic nonce to prepare the stream for block generation.
pub fn absorb_eternity_stream(
    state: &mut KeccakState,
    essence: &[u8],
    relic_nonce: u16,
) {
    let nonce_bytes = [relic_nonce as u8, (relic_nonce >> 8) as u8];
    state.init();
    shake128_absorb(state, essence, SEEDBYTES);
    shake128_absorb(state, &nonce_bytes, 2);
    shake128_finalize(state);
}

