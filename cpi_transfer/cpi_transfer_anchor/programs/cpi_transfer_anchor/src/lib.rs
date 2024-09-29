use {
    anchor_lang::prelude::*,
    anchor_lang::system_program::{transfer, Transfer},
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer as spl_transfer, Mint, Token, TokenAccount, Transfer as SplTransfer},
    },
};

declare_id!("5dDNjNAFeSTxZnzGGVToxVXyq76HwyazaaTmaeQZbrQS");

#[program]
pub mod cpi_transfer_anchor {

    use super::*;

    pub fn sol_transfer(ctx: Context<SolTransfer>, amount: u64) -> Result<()> {
        // Convert `from` property to AccountInfo type
        let from_account_info = ctx.accounts.from.to_account_info();
        // Convert `to` property to AccountInfo type
        let to_account_info = ctx.accounts.to.to_account_info();

        // Convert `system_program` property to AccountInfo type
        let system_program_account_info = ctx.accounts.system_program.to_account_info();

        // Build Cross Program Invocation Context Object
        // We pass in the `program_id` which we will be invoking: the system_program
        // Then we pass in the accounts that we will interact with via the `Transfer`
        // struct that is expected by the `transfer` function we call below.
        let cpi_context = CpiContext::new(
            system_program_account_info,
            Transfer {
                from: from_account_info,
                to: to_account_info,
            },
        );

        // a helper function provided by anchor to invoke the system_programs transfer
        // see below for what it is abstracting away
        transfer(cpi_context, amount)?;
        // Create the transfer instruction
        // let transfer_instruction =
        //     system_instruction::transfer(ctx.accounts.from.key, ctx.accounts.to.key, amount);

        // Invoke the transfer instruction
        // anchor_lang::solana_program::program::invoke_signed(
        //     &transfer_instruction,
        //     &[
        //         from_account_info,
        //         to_account_info,
        //         system_program_account_info,
        //     ],
        //     &[],
        // )?;

        Ok(())
    }

    pub fn spl_token_transfer(ctx: Context<TransferTokens>, amount: u64) -> Result<()> {
        // Build Cross Program Invocation Context Object
        // We pass in the `program_id` which we will be invoking: the token_program
        // Then we pass in the accounts that we will interact with via the `SplTransfer`
        // struct that is expected by the `token::transfer` function we call below.
        let cpi_context: CpiContext<'_, '_, '_, '_, _> = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            SplTransfer {
                from: ctx.accounts.sender_token_account.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.sender.to_account_info(),
            },
        );

        spl_transfer(cpi_context, amount)?;
        // let transfer_instruction = spl_token::instruction::transfer(
        //     &spl_token::ID,
        //     from_ata_account_info.key,
        //     to_ata_account_info.key,
        //     from_authority_account_info.key,
        //     &[],
        //     amount,
        // )?;
        // anchor_lang::solana_program::program::invoke_signed(
        //     &transfer_instruction,
        //     &[from_ata_account_info, to_ata_account_info, from_authority_account_info],
        //     ctx.signer_seeds,
        // )
        // .map_err(Into::into)

        Ok(())
    }
}

#[derive(Accounts)]
pub struct SolTransfer<'info> {
    #[account(mut)]
    from: Signer<'info>,

    #[account(mut)]
    to: SystemAccount<'info>,

    system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferTokens<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,
    pub recipient: SystemAccount<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,
    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = sender,
    )]
    pub sender_token_account: Account<'info, TokenAccount>,
    #[account(
        init_if_needed,
        payer = sender,
        associated_token::mint = mint_account,
        associated_token::authority = recipient,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}
