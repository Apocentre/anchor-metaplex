use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::instructions::CreateMetadataAccountV3InstructionArgs;
use mpl_token_metadata::types::DataV2;
use mpl_token_metadata::{
  types::{Uses, Creator, Collection},
  instructions::CreateMetadataAccountV3,
};


#[derive(Accounts)]
pub struct CreateMetadata<'info> {
  pub mint: AccountInfo<'info>,
  pub mint_authority: AccountInfo<'info>,
  pub metadata_account: AccountInfo<'info>,
  pub payer: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
}

pub fn create_metadata<'a, 'b, 'c, 'info>(
  accounts: CreateMetadata<'info>,
  signer_seeds: &[&[&[u8]]],
  name: String, 
  symbol: String,
  uri: String,
  creators: Option<Vec<Creator>>, 
  seller_fee_basis_points: u16, 
  is_mutable: bool, 
  collection: Option<Collection>, 
  uses: Option<Uses>
) -> Result<()> {
  let ix = CreateMetadataAccountV3 {
    metadata: accounts.metadata_account.key(),
    mint: accounts.mint.to_account_info().key(),
    mint_authority: accounts.mint_authority.to_account_info().key(),
    payer: accounts.payer.key(),
    update_authority: (accounts.update_authority.key(), true),
    system_program: accounts.system_program.key(),
    rent: None,
  };

  solana_program::program::invoke_signed(
    &ix.instruction(CreateMetadataAccountV3InstructionArgs {
        data: DataV2 {
          name,
          symbol,
          uri,
          seller_fee_basis_points,
          creators,
          collection,
          uses,
        },
        is_mutable,
        collection_details: None,
    }),
    &[
      accounts.metadata_account,
      accounts.mint.to_account_info(),
      accounts.mint_authority,
      accounts.payer,
      accounts.update_authority,
      accounts.system_program,
    ],
    signer_seeds,
  ).map_err(Into::into)  
}
