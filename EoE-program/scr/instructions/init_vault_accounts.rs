use crate::states::artifact::*;
use anchor_lang::prelude::*;
use eternity_shield::*;
extern crate eternity_shield;

/// Initializes artifact-related accounts, ensuring the signature's validity.
pub fn init_artifact_accounts(_ctx: Context<InitArtifactAccounts>, args: ArtifactVerifyArgs) -> Result<()> {
    let sig_verify = verify(&args.signature, &args.message, &args.relic_key);
    assert!(sig_verify.is_ok(), "Signature verification failed.");
    Ok(())
}

/// Context for initializing artifact-related accounts.
#[derive(Accounts)]
pub struct InitArtifactAccounts<'info> {
    /// The owner of the artifact, who will sign the transaction.
    #[account(mut)]
    pub creator: Signer<'info>,

    /// The account representing the artifact, initialized and owned by the creator.
    #[account(
        init,
        payer = creator,
        seeds = ["artifact".as_bytes(), creator.key().as_ref()],
        bump,
        space = 1000,
    )]
    pub artifact_account: Account<'info, Artifact>,

    /// The Solana system program, required for account initialization.
    pub system_program: Program<'info, System>,
}

/// Arguments for artifact verification.
#[derive(AnchorDeserialize, AnchorSerialize)]
pub struct ArtifactVerifyArgs {
    /// The signature to verify.
    pub signature: [u8; SIGNBYTES],

    /// The message associated with the artifact.
    pub message: [u8; 32],

    /// The public key of the artifact's creator.
    pub relic_key: [u8; PUBLICKEYBYTES],
}

