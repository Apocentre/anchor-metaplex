Anchor Metaplex
===

Example

```rust
  use anchor_metaplex::{CreateMetadata, create_metadata};

 pub fn Initialize(
    ctx: Context<Initialize>,
    name: String,
    symbol: String,
    uri: String,
 ) -> ProgramResult {
   let seeds: &[&[u8]] = &[
     b"mint_authority", name.as_bytes(), symbol.as_bytes(),
     &[state.bumps.mint_authority]
   ];
   let signer_seeds:&[&[&[u8]]] = &[&seeds[..]];
   let cpi_accounts = CreateMetadata {
     mint: *ctx.accounts.index_token.clone(),
     mint_authority: ctx.accounts.mint_authority.clone(),
     metadata_account: ctx.accounts.metadata_account.clone(),
     payer: (*ctx.accounts.deployer).to_account_info(),
     update_authority: ctx.accounts.mint_authority.to_account_info(),
     system_program: ctx.accounts.system_program.to_account_info(),
     rent: ctx.accounts.rent.to_account_info(),
   };
   
   create_metadata(
     cpi_accounts,
     signer_seeds,
     name.clone(),
     symbol.clone(),
     uri,
     None,
     0,
     true,
     true,
     None,
     None
    )
 }
```
