use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::Mint;
use mpl_token_metadata::instructions::SetAndVerifyCollection as MetaSetAndVerifyCollection;

#[derive(Accounts)]
pub struct SetAndVerifyCollection<'info> {
  pub metadata_account: AccountInfo<'info>,
  pub collection_authority: AccountInfo<'info>,
  pub payer: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub collection_mint: Account<'info, Mint>,
  pub collection_metadata: AccountInfo<'info>,
  pub collection_master_edition: AccountInfo<'info>,
  pub collection_authority_record: AccountInfo<'info>,
}

pub fn set_and_verify_collection<'a, 'b, 'c, 'info>(
  accounts: SetAndVerifyCollection<'info>,
  signer_seeds: &[&[&[u8]]],
) -> Result<()> {
  let ix = MetaSetAndVerifyCollection {
    metadata: accounts.metadata_account.key(),
    collection_authority: accounts.collection_authority.key(),
    payer: accounts.payer.key(),
    update_authority: accounts.update_authority.key(),
    collection_mint: accounts.collection_mint.key(),
    collection: accounts.collection_metadata.key(),
    collection_master_edition_account: accounts.collection_master_edition.key(),
    collection_authority_record: Some(accounts.collection_authority_record.key()),
  };

  solana_program::program::invoke_signed(
    &ix.instruction(),
    &[
      accounts.metadata_account,
      accounts.collection_authority,
      accounts.payer,
      accounts.update_authority,
      accounts.collection_mint.to_account_info(),
      accounts.collection_metadata,
      accounts.collection_master_edition,
    ],
    signer_seeds,
  ).map_err(Into::into)  
}
