use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::{
  instruction,
};

#[derive(Accounts)]
pub struct VerifyCreator<'info> {
  pub metadata_account: AccountInfo<'info>,
  pub creator: Signer<'info>,
}

pub fn verify_creator<'a, 'b, 'c, 'info>(
  accounts: VerifyCreator<'info>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let ix = instruction::sign_metadata(
    mpl_token_metadata::ID,
    accounts.metadata_account.key(),
    accounts.creator.key(),
  );

  solana_program::program::invoke_signed(
    &ix,
    &[
      accounts.metadata_account,
      accounts.creator.to_account_info(),
    ],
    signer_seeds,
  ).map_err(Into::into)  
}
