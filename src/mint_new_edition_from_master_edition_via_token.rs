use anchor_lang::prelude::*;
use anchor_lang::solana_program;
use mpl_token_metadata::instructions::MintNewEditionFromMasterEditionViaToken as MetaMintNewEditionFromMasterEditionViaToken;
use mpl_token_metadata::instructions::MintNewEditionFromMasterEditionViaTokenInstructionArgs;
use mpl_token_metadata::types::MintNewEditionFromMasterEditionViaTokenArgs;

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
}

pub fn mint_new_edition_from_master_edition<'a, 'b, 'c, 'info>(
  accounts: MintNewEditionFromMasterEditionViaToken<'info>,
  signer_seeds: &[&[&[u8]]],
  edition: u64,
) -> Result<()> {
  let ix = MetaMintNewEditionFromMasterEditionViaToken {
    new_metadata: accounts.new_metadata.key(),
    new_edition: accounts.new_edition.key(),
    master_edition: accounts.master_edition.key(),
    new_mint: accounts.new_mint.key(),
    edition_mark_pda: accounts.edition_mark_pda.key(),
    new_mint_authority: accounts.new_mint_authority.key(),
    payer: accounts.payer.key(),
    token_account_owner: accounts.token_account_owner.key(),
    token_account: accounts.token_account.key(),
    new_metadata_update_authority: accounts.new_metadata_update_authority.key(),
    metadata: accounts.metadata.key(),
    token_program: accounts.token_program.key(),
    system_program: accounts.system_program.key(),
    rent: None,
  };

  solana_program::program::invoke_signed(
    &ix.instruction(MintNewEditionFromMasterEditionViaTokenInstructionArgs {
        mint_new_edition_from_master_edition_via_token_args: MintNewEditionFromMasterEditionViaTokenArgs {edition}
    }),
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
    ],
    signer_seeds,
  ).map_err(Into::into)  
} 
