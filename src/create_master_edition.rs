use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use anchor_spl::token::Mint;
use mpl_token_metadata::instructions::CreateMasterEditionV3;
use mpl_token_metadata::instructions::CreateMasterEditionV3InstructionArgs;

#[derive(Accounts)]
pub struct CreateMasterEdition<'info> {
  pub edition: AccountInfo<'info>,
  pub mint: Account<'info, Mint>,
  pub mint_authority: AccountInfo<'info>,
  pub metadata_account: AccountInfo<'info>,
  pub payer: AccountInfo<'info>,
  pub update_authority: AccountInfo<'info>,
  pub token_program: AccountInfo<'info>,
  pub system_program: AccountInfo<'info>,
}

pub fn create_master_edition<'a, 'b, 'c, 'info>(
  accounts: CreateMasterEdition<'info>,
  signer_seeds: &[&[&[u8]]],
  max_supply: Option<u64>,
) -> Result<()> {

  let ix = CreateMasterEditionV3 {
    edition: accounts.edition.key(),
    mint: accounts.mint.to_account_info().key(),
    update_authority: accounts.update_authority.to_account_info().key(),
    mint_authority: accounts.mint_authority.to_account_info().key(),
    payer: accounts.payer.to_account_info().key(),
    metadata: accounts.metadata_account.key(),
    token_program: accounts.token_program.key(),
    system_program: accounts.system_program.key(),
    rent: None,
  };

  solana_program::program::invoke_signed(
    &ix.instruction(CreateMasterEditionV3InstructionArgs {max_supply}),
    &[
      accounts.edition,
      accounts.mint.to_account_info(),
      accounts.update_authority,
      accounts.mint_authority,
      accounts.metadata_account,
      accounts.payer,
      accounts.system_program,
    ],
    signer_seeds,
  ).map_err(Into::into)  
}
