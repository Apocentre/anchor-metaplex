use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::{Mint};
use mpl_token_metadata::{
  instruction,
};

#[derive(Accounts)]
pub struct SetAndVerifyCollection<'info> {
  pub metadata_account: AccountInfo<'info>,
  pub collection_authority: AccountInfo<'info>,
  pub payer: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub collection_mint: Account<'info, Mint>,
  pub collection_metadata: AccountInfo<'info>,
  pub collection_master_edition: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

pub fn set_and_verify_collection<'a, 'b, 'c, 'info>(
  accounts: SetAndVerifyCollection<'info>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let ix = instruction::set_and_verify_collection(
    mpl_token_metadata::ID,
    accounts.metadata_account.key(),
    accounts.collection_authority.key(),
    accounts.payer.key(),
    accounts.update_authority.key(),
    accounts.collection_mint.key(),
    accounts.collection_metadata.key(),
    accounts.collection_master_edition.key(),
    None,
  );

  solana_program::program::invoke_signed(
    &ix,
    &[
      accounts.metadata_account,
      accounts.collection_authority,
      accounts.payer,
      accounts.update_authority,
      accounts.collection_mint.to_account_info(),
      accounts.collection_metadata,
      accounts.collection_master_edition,
      accounts.system_program,
      accounts.rent,
    ],
    signer_seeds,
  ).map_err(Into::into)  
}
