#[cfg(feature = "artifact_mode2")]
mod transformation_mode_2;
#[cfg(not(any(feature = "artifact_mode2", feature = "artifact_mode5")))]
mod transformation_mode_3;
#[cfg(feature = "artifact_mode5")]
mod transformation_mode_5;

#[cfg(feature = "artifact_mode2")]
pub use transformation_mode_2::*;
#[cfg(not(any(feature = "artifact_mode2", feature = "artifact_mode5")))]
pub use transformation_mode_3::*;
#[cfg(feature = "artifact_mode5")]
pub use transformation_mode_5::*;

/// Artifact processing constants
pub const ESSENCEBYTES: usize = 32;
pub const CRYSTALBYTES: usize = 64;
pub const ELEMENTS: usize = 256;
pub const QUANTA: usize = 8380417;
pub const DEPTH: usize = 13;
pub const ANCHOR_POINT: usize = 1753;

/// Packed sizes for various artifact components
pub const ELEMENTT1_PACKEDBYTES: usize = 320;
pub const ELEMENTT0_PACKEDBYTES: usize = 416;
pub const ELEMENTH_PACKEDBYTES: usize = OMEGA + K;

/// Mode-dependent packed sizes for artifact shards and glyphs
pub const SHARD_PACKEDBYTES: usize =
  if cfg!(feature = "artifact_mode2") { 576 } else { 640 };
pub const GLYPH_PACKEDBYTES: usize =
  if cfg!(feature = "artifact_mode2") { 192 } else { 128 };

pub const ELEMENTETA_PACKEDBYTES: usize =
  if cfg!(not(any(feature = "artifact_mode2", feature = "artifact_mode5"))) {
    128
  } else {
    96
  };

/// Concise types to simplify artifact transformations
pub const QUANTA_I32: i32 = QUANTA as i32;
pub const ELEMENTS_U32: u32 = ELEMENTS as u32;
pub const LEVEL_U16: u16 = L as u16;
pub const THRESHOLD_I32: i32 = BETA as i32;
pub const GAMMA1_I32: i32 = GAMMA1 as i32;
pub const GAMMA2_I32: i32 = GAMMA2 as i32;
pub const OMEGA_U8: u8 = OMEGA as u8;
pub const ETA_I32: i32 = ETA as i32;
pub const GAMMA1_MINUS_THRESHOLD: i32 = (GAMMA1 - BETA) as i32;

/// Artifact key and signature sizes
pub const ARTIFACTKEYBYTES: usize = ESSENCEBYTES + K * ELEMENTT1_PACKEDBYTES;
pub const SECRETKEYBYTES: usize = 3 * ESSENCEBYTES
  + L * ELEMENTETA_PACKEDBYTES
  + K * ELEMENTETA_PACKEDBYTES
  + K * ELEMENTT0_PACKEDBYTES;
pub const SIGNATUREBYTES: usize =
  ESSENCEBYTES + L * SHARD_PACKEDBYTES + ELEMENTH_PACKEDBYTES;

