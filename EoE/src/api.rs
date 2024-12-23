use crate::params::SIGNATURE_BYTES;
use crate::transform::*;

/// Enum representing possible errors in verification
pub enum EternityError {
  InvalidInput,
  VerificationFailed,
}

/// Verify an artifact transformation using the provided key
///
/// Example:
/// ```
/// # use echoes_of_eternity::*;
/// # let keys = KeyPair::generate();
/// # let artifact = [0u8; 32];
/// # let signature = keys.sign(&artifact);
/// let verification_result = verify_transformation(&signature, &artifact, &keys.public);
/// assert!(verification_result.is_ok());
/// ```
pub fn verify_transformation(
  signature: &[u8], 
  artifact: &[u8], 
  public_key: &[u8]
) -> Result<(), EternityError> {
  if signature.len() != SIGNATURE_BYTES {
      return Err(EternityError::InvalidInput);
  }

  // Wrapping inputs in Box for heap allocation
  let signature_box = Box::new(signature.to_vec());
  let artifact_box = Box::new(artifact.to_vec());
  let public_key_box = Box::new(public_key.to_vec());

  // Perform transformation verification
  transform_verify(&signature_box, &artifact_box, &public_key_box)
}

