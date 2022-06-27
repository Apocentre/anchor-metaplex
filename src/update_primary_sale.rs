use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::{
  instruction
};


#[derive(Accounts)]
pub struct UpdatePrimarySaleHappenedViaToken<'info> {
  pub metadata_account: AccountInfo<'info>,
  pub owner: Signer<'info>,
  pub token: AccountInfo<'info>,
}

pub fn update_primary_sale_happened_via_token<'a, 'b, 'c, 'info>(
  accounts: UpdatePrimarySaleHappenedViaToken<'info>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let ix = instruction::update_primary_sale_happened_via_token(
    mpl_token_metadata::ID,
    accounts.metadata_account.key(),
    accounts.owner.key(),
    accounts.token.key(),
  );

  solana_program::program::invoke_signed(
    &ix,
    &[
      accounts.metadata_account,
      accounts.owner.to_account_info(),
      accounts.token,
    ],
    signer_seeds,
  ).map_err(Into::into)  
}
