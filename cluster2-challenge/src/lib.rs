use anchor_lang::prelude::*;

declare_id!("B5a1KZXAF1hJfefX6BtaAxA9tMReP82MExohbt6g2Hnn");

#[program]
pub mod cluster2_challenge {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
