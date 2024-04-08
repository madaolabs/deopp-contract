use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, Transfer};

declare_id!("5HsDyhfkCMjy3Kebufc2Fc7Zr5LhSZzh9sQP5wVMnriG");

#[program]
pub mod deopp_contract {

    use super::*;

    pub fn create_giveaway(ctx: Context<CreateGiveaway>, args: CreateGiveawayARG) -> Result<()> {
        ctx.accounts.giveaway_pool.receiver = args.receiver.clone();
        ctx.accounts.giveaway_pool.receive_records = Vec::new();
        ctx.accounts.giveaway_pool.amount = args.amount;
        let cpi_accounts = Transfer {
            from: ctx.accounts.from_account.to_account_info(),
            to: ctx.accounts.token_pool.to_account_info(),
            authority: ctx.accounts.payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let receiver_count: u64 = args.receiver.len() as u64;
        let _ = token::transfer(cpi_ctx, receiver_count * args.amount);

        return Ok(());
    }

    pub fn receive_giveaway(
        ctx: Context<ReceiveGiveawayAccount>,
        _giveaway_id: [u8; 20],
    ) -> Result<()> {
        let receive_amount = ctx.accounts.giveaway_pool.amount;
        let receive_records = &ctx.accounts.giveaway_pool.receive_records;
        let receiver = &ctx.accounts.giveaway_pool.receiver;
        let payer = ctx.accounts.payer.key();
        let mut is_receiver = false;
        for receive in receiver.iter() {
            if payer.eq(receive) {
                is_receiver = true;
            }
        }

        let mut is_received = false;
        for receive in receive_records.iter() {
            if payer.eq(receive) {
                is_received = true;
            }
        }

        require_eq!(is_receiver, true);
        require_eq!(is_received, false);

        let (_pool, bump) = Pubkey::find_program_address(
            &[&ctx.accounts.token_mint.key().to_bytes()],
            ctx.program_id,
        );

        let cpi_accounts = Transfer {
            from: ctx.accounts.token_pool.to_account_info(),
            to: ctx.accounts.to_account.to_account_info(),
            authority: ctx.accounts.token_pool.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let binding = ctx.accounts.token_mint.key();
        let binding: &[&[&[u8]]] = &[&[binding.as_ref(), &[bump]]];
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, binding);

        let _ = token::transfer(cpi_ctx, receive_amount);

        return Ok(());
    }
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
    token_mint: Account<'info, Mint>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut, token::mint = token_mint)]
    from_account: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = payer, token::mint = token_mint, token::authority = token_pool, seeds = [&token_mint.key().to_bytes()], bump)]
    token_pool: Account<'info, TokenAccount>,
    #[account(init_if_needed, payer = payer, space = 8 + usize::try_from(args.receiver.len()).unwrap() * 32 * 2 + 8, seeds = [&args.giveaway_id.clone()], bump)]
    giveaway_pool: Account<'info, GiveawayPool>,
}

// PDA 账户
#[account]
pub struct GiveawayPool {
    receiver: Vec<Pubkey>,
    receive_records: Vec<Pubkey>, // receive records
    amount: u64,
}

#[derive(Accounts)]
#[instruction(giveaway_id: [u8; 20])]
pub struct ReceiveGiveawayAccount<'info> {
    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    #[account(mut)]
    payer: Signer<'info>,
    #[account(mut, seeds = [&giveaway_id.clone()], bump)]
    giveaway_pool: Account<'info, GiveawayPool>,
    #[account(mut, token::mint = token_mint, token::authority = token_pool, seeds = [&token_mint.key().to_bytes()], bump)]
    token_pool: Account<'info, TokenAccount>,
    #[account(mut)]
    to_account: Account<'info, TokenAccount>,
    token_mint: Account<'info, Mint>,
}
