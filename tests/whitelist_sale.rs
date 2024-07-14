use anchor_lang::prelude::*;
use anchor_lang::solana_program::clock::Epoch;
use anchor_lang::solana_program::pubkey::Pubkey;
use anchor_lang::solana_program::system_program;
use anchor_lang::solana_program::sysvar;

#[program]
mod whitelist_sale {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, price: u64, limit: u64) -> Result<()> {
        let sale_account = &mut ctx.accounts.sale_account;
        sale_account.price = price;
        sale_account.limit = limit;
        Ok(())
    }

    pub fn whitelist(ctx: Context<Whitelist>, user: Pubkey) -> Result<()> {
        let sale_account = &mut ctx.accounts.sale_account;
        if sale_account.whitelisted_users.contains(&user) {
            return Err(ProgramError::Custom(1)); // User already whitelisted
        }
        sale_account.whitelisted_users.push(user);
        Ok(())
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        let sale_account = &mut ctx.accounts.sale_account;
        let buyer = &ctx.accounts.buyer;

        if !sale_account.whitelisted_users.contains(&buyer.key()) {
            return Err(ProgramError::Custom(2)); // User not whitelisted
        }

        let user_bought = *sale_account.purchased.get(&buyer.key()).unwrap_or(&0);
        if user_bought + amount > sale_account.limit {
            return Err(ProgramError::Custom(3)); // Purchase limit exceeded
        }

        let total_price = sale_account.price * amount;
        // Transfer tokens logic here

        sale_account
            .purchased
            .insert(buyer.key(), user_bought + amount);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 64)]
    pub sale_account: Account<'info, SaleAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Whitelist<'info> {
    #[account(mut)]
    pub sale_account: Account<'info, SaleAccount>,
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(mut)]
    pub sale_account: Account<'info, SaleAccount>,
    pub buyer: Signer<'info>,
}

#[account]
pub struct SaleAccount {
    pub price: u64,
    pub limit: u64,
    pub whitelisted_users: Vec<Pubkey>,
    pub purchased: HashMap<Pubkey, u64>,
}
