use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("GwiGatU1wasA7wF7LtcrxjnZJD1AdwD4yE4FTBZN4k5o");

#[program]
pub mod whitelist_sale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
