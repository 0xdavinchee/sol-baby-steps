use anchor_lang::error_code;
use anchor_lang::prelude::*;
use std::mem::size_of;

// TODOs
// Add permissions so that only the signer who created
// the account has permission to call the increment function on their particular
// counter data
// Clean up the folder structure

declare_id!("E9eVtZTjTM37Zv2fWYCKGRJavnpmRygMkb6hCHw1UDU2");

#[program]
pub mod counter_anchor {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        ctx.accounts.counter_account.authority = ctx.accounts.signer.key();
        Ok(())
    }

    pub fn increment(ctx: Context<UpdateData>, data: u64) -> Result<()> {
        ctx.accounts.counter_account.counter += data;

        Ok(())
    }

    pub fn decrement(ctx: Context<UpdateData>, data: u64) -> Result<()> {
        if ctx.accounts.counter_account.counter < data {
            ctx.accounts.counter_account.counter = 0;
        } else {
            ctx.accounts.counter_account.counter -= data;
        }

        Ok(())
    }

    pub fn update(ctx: Context<UpdateData>, data: u64) -> Result<()> {
        ctx.accounts.counter_account.counter = data;
        Ok(())
    }

    pub fn reset(ctx: Context<UpdateData>) -> Result<()> {
        ctx.accounts.counter_account.counter = 0;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account[
        init,
        payer = signer,
        space = size_of::<CounterAccount>() + 8
    ]]
    pub counter_account: Account<'info, CounterAccount>,

    #[account(mut)]
    pub signer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateData<'info> {
    #[account(
        mut,
        has_one = authority @ CounterError::SignerIsNotAuthority
    )]
    pub counter_account: Account<'info, CounterAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,
}

#[account]
pub struct CounterAccount {
    counter: u64,
    authority: Pubkey,
}

#[error_code]
pub enum CounterError {
    SignerIsNotAuthority,
}
