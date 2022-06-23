use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::{Mint};
use mpl_token_metadata::{
  instruction,
};

#[derive(Accounts)]
pub struct CreateMasterEdition<'info> {
  pub edition: AccountInfo<'info>,
  pub mint: Account<'info, Mint>,
  pub mint_authority: AccountInfo<'info>,
  pub metadata_account: AccountInfo<'info>,
  pub payer: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

pub fn create_master_edition<'a, 'b, 'c, 'info>(
  accounts: CreateMasterEdition<'info>,
  max_supply: Option<u64>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let ix = instruction::create_master_edition_v3(
    mpl_token_metadata::ID,
    accounts.edition.key(),
    accounts.mint.to_account_info().key(),
    accounts.update_authority.to_account_info().key(),
    accounts.mint_authority.to_account_info().key(),
    accounts.metadata_account.key(),
    accounts.payer.to_account_info().key(),
    max_supply,
  );

  solana_program::program::invoke_signed(
    &ix,
    &[
      accounts.edition,
      accounts.mint.to_account_info(),
      accounts.update_authority,
      accounts.mint_authority,
      accounts.metadata_account,
      accounts.payer,
      accounts.system_program,
      accounts.rent,
    ],
    signer_seeds,
  ).map_err(Into::into)  
}
