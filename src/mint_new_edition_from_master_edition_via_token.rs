use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::{
  instruction::{
    mint_new_edition_from_master_edition_via_token,
  },
};

#[derive(Accounts)]
pub struct MintNewEditionFromMasterEditionViaToken<'info> {
  pub new_metadata: AccountInfo<'info>,
  pub new_edition: AccountInfo<'info>,
  pub master_edition: AccountInfo<'info>,
  pub new_mint: AccountInfo<'info>,
  pub edition_mark_pda: AccountInfo<'info>,
  pub new_mint_authority: AccountInfo<'info>,
  pub payer: Signer<'info>,
  pub token_account_owner: Signer<'info>,
  pub token_account: AccountInfo<'info>,
  pub new_metadata_update_authority: AccountInfo<'info>,
  pub metadata: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
  pub rent: AccountInfo<'info>,
}

pub fn mint_new_edition_from_master_edition<'a, 'b, 'c, 'info>(
  accounts: MintNewEditionFromMasterEditionViaToken<'info>,
  signer_seeds: &[&[&[u8]]],
  edition: u64,
) -> Result<()> {
  let ix = mint_new_edition_from_master_edition_via_token(
    mpl_token_metadata::ID,
    accounts.new_metadata.key(),
    accounts.new_edition.key(),
    accounts.master_edition.key(),
    accounts.new_mint.key(),
    accounts.new_mint_authority.key(),
    accounts.payer.key(),
    accounts.token_account_owner.key(),
    accounts.token_account.key(),
    accounts.new_metadata_update_authority.key(),
    accounts.metadata.key(),
    accounts.metadata.key(),
    edition,
  );

  solana_program::program::invoke_signed(
    &ix,
    &[
      accounts.new_metadata,
      accounts.new_edition,
      accounts.master_edition,
      accounts.new_mint,
      accounts.edition_mark_pda,
      accounts.new_mint_authority,
      accounts.payer.to_account_info(),
      accounts.token_account_owner.to_account_info(),
      accounts.token_account,
      accounts.new_metadata_update_authority,
      accounts.metadata,
      accounts.token_program,
      accounts.system_program,
      accounts.rent,
    ],
    signer_seeds,
  ).map_err(Into::into)  
} 
