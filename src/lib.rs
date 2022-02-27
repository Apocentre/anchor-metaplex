
//! # Anchor Metaplex
//!
//! `anchor_metaplex` is a collection of helper numeric operation functions that allows seamless
//! interaction of your Solana Anchor program with the Metaplex ecosystem
//! 
//! # Examples
//!
//! ```
//! use anchor_metaplex::{CreateMetadata, create_metadata};
//! 
//! pub fn Initialize(
//!    ctx: Context<Initialize>,
//!    name: String,
//!    symbol: String,
//!    uri: String,
//! ) -> Result<()> {
//!   let seeds: &[&[u8]] = &[
//!     b"mint_authority", name.as_bytes(), symbol.as_bytes(),
//!     &[state.bumps.mint_authority]
//!   ];
//!   let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
//!   let cpi_accounts = CreateMetadata {
//!     mint: *ctx.accounts.index_token.clone(),
//!     mint_authority: ctx.accounts.mint_authority.clone(),
//!     metadata_account: ctx.accounts.metadata_account.clone(),
//!     payer: (*ctx.accounts.deployer).to_account_info(),
//!     update_authority: ctx.accounts.mint_authority.to_account_info(),
//!     system_program: ctx.accounts.system_program.to_account_info(),
//!     rent: ctx.accounts.rent.to_account_info(),
//!   };
//!   
//!   create_metadata(
//!     cpi_accounts,
//!     signer_seeds,
//!     name.clone(),
//!     symbol.clone(),
//!     uri,
//!     None,
//!     0,
//!     true,
//!     true,
//!     None,
//!     None
//!    )
//! }
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
) -> Result<()> {
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
