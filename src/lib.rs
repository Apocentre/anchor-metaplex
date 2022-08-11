
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
pub mod create_metadata;
pub mod update_metadata;
pub mod create_master_edition;
pub mod set_and_verify_collection;
pub mod verify_creator;
pub mod update_primary_sale;
pub mod mint_new_edition_from_master_edition_via_token;

pub use mpl_token_metadata;
pub use crate::{
  create_metadata::*,
  update_metadata::*,
  create_master_edition::*,
  set_and_verify_collection::*,
  verify_creator::*,
  update_primary_sale::*,
};
