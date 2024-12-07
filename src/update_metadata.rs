use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::instructions::UpdateMetadataAccountV2InstructionArgs;
use mpl_token_metadata::{
  types::DataV2,
  instructions::UpdateMetadataAccountV2
};


#[derive(Accounts)]
pub struct UpdateMetadata<'info> {
  pub metadata_account: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
}

pub fn update_metadata<'a, 'b, 'c, 'info>(
  accounts: UpdateMetadata<'info>,
  signer_seeds: &[&[&[u8]]],
  new_update_authority: Option<Pubkey>,
  data: Option<DataV2>,
  primary_sale_happened: Option<bool>,
  is_mutable: Option<bool>,
) -> Result<()> {
  let ix = UpdateMetadataAccountV2 {
    metadata: accounts.metadata_account.key(),
    update_authority: accounts.update_authority.to_account_info().key(),
  };

  solana_program::program::invoke_signed(
    &ix.instruction(UpdateMetadataAccountV2InstructionArgs {
      data,
      new_update_authority,
      primary_sale_happened,
      is_mutable,
    }),
    &[],
    signer_seeds,
  ).map_err(Into::into)  
}
