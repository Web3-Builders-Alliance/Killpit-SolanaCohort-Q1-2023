use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod whitelist {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let whitelist = &mut ctx.accounts.whitelist;
        whitelist.authority = ctx.accounts.authority.key();
        Ok(())
    }

    pub fn add_whitelist(ctx: Context<AddWhiteList>) -> Result<()> {
        require!(ctx.accounts.authority.key() == ctx.accounts.whitelist.authority,
        WhitelistErrors::InvalidAuthority);
        Ok(())
    }

    pub fn remove_whitelist(ctx: Context<RemoveWhiteList>) -> Result<()> {
        require!(ctx.accounts.authority.key() == ctx.accounts.whitelist.authority,
        WhitelistErrors::InvalidAuthority);
        Ok(())
    }
}

`#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        seeds = [whitelist.authority.key().as_ref(), user_wallet.key().as_ref()],
        bump,
        space = 8,
    )]
    pub whitelist: AccountInfo<'info, WhitelistState>,
    pub whitelist_pda: Account<'info, WhitelistPDA>,
    pub authority: Signer<'info, Pubkey>,
    pub user_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
`
#[derive(Accounts)]
pub struct AddWhiteList<'info> {
    pub whitelist: AccountInfo<'info, WhitelistState>,
    #[account(
        init,
        payer = authority,
        seeds = [whitelist.authority.key().as_ref(), user_wallet.key().as_ref()],
        bump,
        space = 8,
    )]
    pub whitelist_pda: Account<'info, WhitelistPDA>,
    pub authority: Signer<'info, Pubkey>,
    pub user_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
}
#[derive(Accounts)]
struct RemoveWhiteList<'info> {
    pub whitelist: AccountInfo<'info, WhitelistState>,
    #[account(
        mut,
        close = authority
    )]
    pub authority: Signer<'info, Pubkey>,
    pub user_wallet: AccountInfo<'info>,
    pub system_program: Program<'info, System>,
    pub whitelist_pda: Account<'info, WhitelistPDA>,
}

#[account]
pub struct WhitelistState {
    authority: Pubkey,
}
#[account]
pub struct WhitelistPDA {

}

impl WhitelistState {
    const LEN: usize = 8 + 32;
}

#[error_code]
pub enum WhitelistErrors {
    #[msg("Invalid authority")]
    InvalidAuthority
}