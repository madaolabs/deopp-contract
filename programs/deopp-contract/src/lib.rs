use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("5t9e4i4E33pBR8uoWu2rqznVAhvwkhKzkixDMhQQTyQw");

#[program]
pub mod deopp_contract {
    use super::*;

    pub fn create_giveaway(ctx: Context<CreateGiveaway>, args: CreateGiveawayARG) -> Result<()> {
        ctx.accounts.giveaway_pool.receiver = args.receiver.clone();
        ctx.accounts.giveaway_pool.receive_records = Vec::new();
        ctx.accounts.giveaway_pool.amount = args.amount;
        let transfer_accounts = Transfer {
            from: ctx.accounts.from_account.to_account_info(),
            to: ctx.accounts.token_pool.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        token::transfer(transfer_accounts, args.amount);

        return Ok(());
    }

    // pub fn receive_giveaway(ctx: Context) -> Result<()> {
    //     return Ok(());
    // }
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct CreateGiveawayARG {
    giveaway_id: [u8; 20],
    receiver: Vec<Pubkey>, // 接收者
    amount: u64,           // 每个接收的token数量
}

#[derive(Accounts)]
#[instruction(args: CreateGiveawayARG)]
pub struct CreateGiveaway<'info> {
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    rent: Sysvar<'info, Rent>,
    #[account(mut)]
    payer: Signer<'info>,
    /// CHECK:
    #[account(mut, token::mint = token_mint)]
    from_account: Account<'info, TokenAccount>,
    /// CHECK:
    #[account(init, payer = payer, token::mint = token_mint, token::authority = token_pool, seeds = [&payer.key().to_bytes(), &token_mint.key().to_bytes()], bump)]
    token_pool: Account<'info, TokenAccount>,
    token_mint: Account<'info, Mint>,

    #[account(init, payer = payer, space = 8 + usize::try_from(args.receiver.len()).unwrap() * 32, seeds = [&args.giveaway_id.clone()], bump)]
    giveaway_pool: Account<'info, GiveawayPool>,
}

// PDA 账户
#[account]
pub struct GiveawayPool {
    receiver: Vec<Pubkey>,
    receive_records: Vec<Pubkey>, // receive records
    amount: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ReceiveNonPutGiveawayARG {
    giveaway_id: [u8; 20], // 红包ID
    amount: u128,
    timestamp: u64,
    signature: [u8; 65],
}

#[derive(Accounts)]
#[instruction(args: ReceiveNonPutGiveawayARG)]
pub struct ReceiveNonPutGiveawayAccount<'info> {
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut, seeds = [&args.giveaway_id.clone()], bump)]
    giveaway_pool: Account<'info, GiveawayPool>,
    /// CHECK:
    #[account(mut, token::mint = token_mint, token::authority = token_pool, seeds = [&payer.key().to_bytes(), &token_mint.key().to_bytes()], bump)]
    token_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    to_account: Account<'info, TokenAccount>,
    token_mint: Account<'info, Mint>,
}
