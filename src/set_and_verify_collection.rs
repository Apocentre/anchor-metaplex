use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::{Mint};
use mpl_token_metadata::{
  state::{Uses, Creator, Collection},
  instruction::*,
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
  
) -> Result<()> {
  todo!()
}
