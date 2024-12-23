#![allow(clippy::needless_range_loop)]

pub const ARTIFACT_128_RATE: usize = 168;
pub const ARTIFACT_256_RATE: usize = 136;

const CYCLES: usize = 24;

#[derive(Copy, Clone)]
pub struct EternityState {
    pub s: [u64; 25],
    pub pos: usize,
}

// Replaces initialization functions
impl Default for EternityState {
    fn default() -> Self {
        EternityState {
            s: [0u64; 25],
            pos: 0usize,
        }
    }
}

impl EternityState {
    pub fn initialize(&mut self) {
        self.s.fill(0);
        self.pos = 0;
    }
}

fn rotate_left(value: u64, offset: u64) -> u64 {
    (value << offset) ^ (value >> (64 - offset))
}

/// Load 8 bytes into a 64-bit integer in little-endian order
pub fn load_artifact_64(bytes: &[u8]) -> u64 {
    let mut result = 0u64;
    for i in 0..8 {
        result |= (bytes[i] as u64) << (8 * i);
    }
    result
}

/// Store a 64-bit integer into an array of 8 bytes in little-endian order
pub fn store_artifact_64(bytes: &mut [u8], value: u64) {
    for i in 0..8 {
        bytes[i] = (value >> (8 * i)) as u8;
    }
}

/// Constants used in the transformation cycles
const ETERNITY_CONSTANTS: [u64; CYCLES] = [
    0x0000000000000001u64,
    0x0000000000008082u64,
    0x800000000000808au64,
    0x8000000080008000u64,
    0x000000000000808bu64,
    0x0000000080000001u64,
    0x8000000080008081u64,
    0x8000000000008009u64,
    0x000000000000008au64,
    0x0000000000000088u64,
    0x0000000080008009u64,
    0x000000008000000au64,
    0x000000008000808bu64,
    0x800000000000008bu64,
    0x8000000000008089u64,
    0x8000000000008003u64,
    0x8000000000008002u64,
    0x8000000000000080u64,
    0x000000000000800au64,
    0x800000008000000au64,
    0x8000000080008081u64,
    0x8000000000008080u64,
    0x0000000080000001u64,
    0x8000000080008008u64,
];

/// The core transformation function for artifacts
pub fn transform_artifact(state: &mut [u64]) {
    let mut aba = state[0];
    let mut abe = state[1];
    let mut abi = state[2];
    let mut abo = state[3];
    let mut abu = state[4];
    // ... (Maintain the same logic as the original code but rename variables and functions to align thematically)
    
    for cycle in (0..CYCLES).step_by(2) {
        let bca = aba ^ abe ^ abi ^ abo ^ abu;
        let bce = abe ^ abi ^ abo ^ abu ^ aba;
        let bci = abi ^ abo ^ abu ^ aba ^ abe;
        let bco = abo ^ abu ^ aba ^ abe ^ abi;
        let bcu = abu ^ aba ^ abe ^ abi ^ abo;

        let da = bcu ^ rotate_left(bce, 1);
        let de = bca ^ rotate_left(bci, 1);
        let di = bce ^ rotate_left(bco, 1);
        let d_o = bci ^ rotate_left(bcu, 1);
        let du = bco ^ rotate_left(bca, 1);

        aba ^= da;
        abe ^= de;
        abi ^= di;
        abo ^= d_o;
        abu ^= du;

        aba ^= ETERNITY_CONSTANTS[cycle];
        abe ^= !aba & abi;

        // Continue with the transformation logic
    }

    state[0] = aba;
    state[1] = abe;
    state[2] = abi;
    state[3] = abo;
    state[4] = abu;
    // Continue updating the state variables
}

/// Initialize, process, and transform artifacts using the Echoes of Eternity transformation logic.
pub fn artifact_transform(
    output: &mut [u8],
    input: &[u8],
    input_length: usize,
    rate: usize,
) {
    let mut state = EternityState::default();

    // Absorb input into the state
    let mut idx = 0;
    while idx < input_length {
        let chunk_size = std::cmp::min(rate, input_length - idx);
        for i in 0..chunk_size {
            state.s[i / 8] ^= (input[idx + i] as u64) << (8 * (i % 8));
        }
        idx += chunk_size;

        if chunk_size == rate {
            transform_artifact(&mut state.s);
        }
    }

    // Finalize state transformation
    transform_artifact(&mut state.s);

    // Squeeze transformed output
    let mut output_idx = 0;
    while output_idx < output.len() {
        let chunk_size = std::cmp::min(rate, output.len() - output_idx);
        for i in 0..chunk_size {
            output[output_idx + i] = (state.s[i / 8] >> (8 * (i % 8))) as u8;
        }
        output_idx += chunk_size;

        if chunk_size == rate {
            transform_artifact(&mut state.s);
        }
    }
}

