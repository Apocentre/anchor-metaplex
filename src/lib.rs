use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::{Mint};
use mpl_token_metadata::{
  state::{Uses, Creator, Collection},
  instruction::{
    create_metadata_accounts_v2,
  },
};

pub use mpl_token_metadata;

pub fn create_metadata<'a, 'b, 'c, 'info>(
  accounts: CreateMetadata<'info>,
  signer_seeds: &[&[&[u8]]],
  name: String, 
  symbol: String,
  uri: String,
  creators: Option<Vec<Creator>>, 
  seller_fee_basis_points: u16, 
  update_authority_is_signer: bool, 
  is_mutable: bool, 
  collection: Option<Collection>, 
  uses: Option<Uses>
) -> ProgramResult {
  let ix = create_metadata_accounts_v2(
    mpl_token_metadata::ID,
    accounts.metadata_account.key(),
    accounts.mint.to_account_info().key(),
    accounts.mint_authority.to_account_info().key(),
    accounts.payer.to_account_info().key(),
    accounts.update_authority.to_account_info().key(),
    name,
    symbol,
    uri,
    creators,
    seller_fee_basis_points,
    update_authority_is_signer,
    is_mutable,
    collection,
    uses,
  );

  solana_program::program::invoke_signed(
    &ix,
    &[
      accounts.metadata_account,
      accounts.mint.to_account_info(),
      accounts.mint_authority,
      accounts.payer,
      accounts.update_authority,
      accounts.system_program,
      accounts.rent,
    ],
    signer_seeds,
  )  
}

#[derive(Accounts)]
pub struct CreateMetadata<'info> {
  pub mint: Account<'info, Mint>,
  pub mint_authority: AccountInfo<'info>,
  pub metadata_account: AccountInfo<'info>,
  pub payer: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}
