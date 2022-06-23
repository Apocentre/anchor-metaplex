use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::{
  state::{Uses, Creator, Collection, DataV2},
  instruction::{
    update_metadata_accounts_v2,
  },
};


#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
  pub metadata_account: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

pub fn update_metadata<'a, 'b, 'c, 'info>(
  accounts: UpdateMetadata<'info>,
  signer_seeds: &[&[&[u8]]],
  new_update_authority: Option<Pubkey>,
  name: String,
  symbol: String,
  uri: String,
  creators: Option<Vec<Creator>>,
  seller_fee_basis_points: u16,
  primary_sale_happened: Option<bool>,
  is_mutable: Option<bool>,
  collection: Option<Collection>,
  uses: Option<Uses>
) -> Result<()> {
  let ix = update_metadata_accounts_v2(
    mpl_token_metadata::ID,
    accounts.metadata_account.key(),
    accounts.update_authority.to_account_info().key(),
    new_update_authority,
    Some(DataV2 {
      name,
      symbol,
      uri,
      seller_fee_basis_points,
      creators,
      collection,
      uses,
    }),
    primary_sale_happened,
    is_mutable,
  );

  solana_program::program::invoke_signed(
    &ix,
    &[
      accounts.metadata_account,
      accounts.update_authority,
    ],
    signer_seeds,
  ).map_err(Into::into)  
}
