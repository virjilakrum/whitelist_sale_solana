use anchor_lang::prelude::*;
use std::collections::HashMap;

declare_id!("GwiGatU1wasA7wF7LtcrxjnZJD1AdwD4yE4FTBZN4k5o");

#[program]
pub mod whitelist_sale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, price: u64, limit: u64) -> Result<()> {
        let sale_account = &mut ctx.accounts.sale_account;
        sale_account.price = price;
        sale_account.limit = limit;
        sale_account.whitelisted_users = Vec::new();
        sale_account.purchased = HashMap::new();
        Ok(())
    }

#[derive(Accounts)]
pub struct Initialize {}
