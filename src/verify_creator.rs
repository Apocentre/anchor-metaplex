use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::instructions::SignMetadata;

#[derive(Accounts)]
pub struct VerifyCreator<'info> {
  pub metadata_account: AccountInfo<'info>,
  pub creator: Signer<'info>,
}

pub fn verify_creator<'a, 'b, 'c, 'info>(
  accounts: VerifyCreator<'info>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let ix = SignMetadata {
    metadata: accounts.metadata_account.key(),
    creator: accounts.creator.key(),
  };

  solana_program::program::invoke_signed(
    &ix.instruction(),
    &[],
    signer_seeds,
  ).map_err(Into::into)  
}
