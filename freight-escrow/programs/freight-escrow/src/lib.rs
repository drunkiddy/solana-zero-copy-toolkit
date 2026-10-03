use anchor_lang::prelude::*;
use anchor_spl::token::{self, Mint, Token, TokenAccount, TransferChecked};

// Development placeholder; synchronize with a new deployment key before deployment.
declare_id!("Fn1eqswyWzygRwt83wwuGycqUQvUFmPDoMTGB3oeaYwx");

#[program]
pub mod freight_escrow {
    use super::*;

    pub fn fund(ctx: Context<Fund>, order_id: u64, amount: u64,
        deadline: i64, agreement_hash: [u8; 32]) -> Result<()> {
        require!(amount > 0, EscrowError::InvalidAmount);
        require!(deadline > Clock::get()?.unix_timestamp, EscrowError::Deadline);
        require!(ctx.accounts.mint.decimals == 6, EscrowError::MintDecimals);
        let shipper = ctx.accounts.shipper.key();
        let carrier = ctx.accounts.carrier.key();
        let arbitrator = ctx.accounts.arbitrator.key();
        require!(shipper != carrier && arbitrator != carrier && arbitrator != shipper,
            EscrowError::DistinctRoles);
        let order = &mut ctx.accounts.order;
        order.shipper = shipper;
        order.carrier = carrier;
        order.arbitrator = arbitrator;
        order.mint = ctx.accounts.mint.key();
        order.order_id = order_id;
        order.amount = amount;
        order.deadline = deadline;
        order.agreement_hash = agreement_hash;
        order.bump = ctx.bumps.order;
        order.status = Status::Funded;
        token::transfer_checked(CpiContext::new(ctx.accounts.token_program.to_account_info(),
            TransferChecked {
                from: ctx.accounts.source.to_account_info(),
                to: ctx.accounts.vault.to_account_info(),
                authority: ctx.accounts.shipper.to_account_info(),
                mint: ctx.accounts.mint.to_account_info(),
            }), amount, ctx.accounts.mint.decimals)?;
        Ok(())
    }

    pub fn accept(ctx: Context<Act>) -> Result<()> {
        let order = &mut ctx.accounts.order;
        require_keys_eq!(ctx.accounts.actor.key(), order.carrier, EscrowError::Unauthorized);
        require!(order.status == Status::Funded, EscrowError::InvalidState);
        require!(Clock::get()?.unix_timestamp < order.deadline, EscrowError::Deadline);
        order.status = Status::Accepted;
        Ok(())
    }

    pub fn dispute(ctx: Context<Act>, evidence_hash: [u8; 32]) -> Result<()> {
        let order = &mut ctx.accounts.order;
        let actor = ctx.accounts.actor.key();
        require!(actor == order.shipper || actor == order.carrier, EscrowError::Unauthorized);
        require!(order.status == Status::Accepted, EscrowError::InvalidState);
        order.evidence_hash = evidence_hash;
        order.status = Status::Disputed;
        Ok(())
    }

    pub fn release(ctx: Context<Settle>) -> Result<()> {
        require_keys_eq!(ctx.accounts.actor.key(), ctx.accounts.order.shipper, EscrowError::Unauthorized);
        require!(ctx.accounts.order.status == Status::Accepted, EscrowError::InvalidState);
        let amount = ctx.accounts.order.amount;
        settle(&ctx.accounts, amount)?;
        ctx.accounts.order.status = Status::Settled;
        Ok(())
    }

    pub fn refund_expired(ctx: Context<Settle>) -> Result<()> {
        require_keys_eq!(ctx.accounts.actor.key(), ctx.accounts.order.shipper, EscrowError::Unauthorized);
        require!(ctx.accounts.order.status == Status::Funded, EscrowError::InvalidState);
        require!(Clock::get()?.unix_timestamp >= ctx.accounts.order.deadline, EscrowError::Deadline);
        settle(&ctx.accounts, 0)?;
        ctx.accounts.order.status = Status::Refunded;
        Ok(())
    }

    pub fn resolve(ctx: Context<Settle>, carrier_amount: u64) -> Result<()> {
        require_keys_eq!(ctx.accounts.actor.key(), ctx.accounts.order.arbitrator, EscrowError::Unauthorized);
        require!(ctx.accounts.order.status == Status::Disputed, EscrowError::InvalidState);
        require!(carrier_amount <= ctx.accounts.order.amount, EscrowError::InvalidAmount);
        settle(&ctx.accounts, carrier_amount)?;
        ctx.accounts.order.status = Status::Settled;
        Ok(())
    }
}

fn settle(accounts: &Settle, carrier_amount: u64) -> Result<()> {
    let order = &accounts.order;
    require!(accounts.vault.amount >= order.amount, EscrowError::Underfunded);
    let id = order.order_id.to_le_bytes();
    let bump = [order.bump];
    let seeds: &[&[u8]] = &[b"order", order.shipper.as_ref(), &id, &bump];
    let signers = &[seeds];
    // Any unsolicited surplus goes to the shipper. It cannot block settlement.
    let shipper_amount = accounts.vault.amount.checked_sub(carrier_amount)
        .ok_or(EscrowError::InvalidAmount)?;
    for (destination, amount) in [(&accounts.carrier_tokens, carrier_amount),
        (&accounts.shipper_tokens, shipper_amount)] {
        if amount > 0 {
            token::transfer_checked(CpiContext::new_with_signer(
                accounts.token_program.to_account_info(), TransferChecked {
                    from: accounts.vault.to_account_info(),
                    to: destination.to_account_info(),
                    authority: accounts.order.to_account_info(),
                    mint: accounts.mint.to_account_info(),
                }, signers), amount, accounts.mint.decimals)?;
        }
    }
    // Retain order and vault to prevent order ID replay. Post-settlement donations
    // have no recovery instruction in this development version.
    Ok(())
}

#[derive(Accounts)]
#[instruction(order_id: u64)]
pub struct Fund<'info> {
    #[account(mut)]
    pub shipper: Signer<'info>,
    /// CHECK: Immutable recipient identity; never dereferenced.
    pub carrier: UncheckedAccount<'info>,
    /// CHECK: Immutable arbitrator identity; never dereferenced.
    pub arbitrator: UncheckedAccount<'info>,
    // Development permits a six-decimal test token; production needs a fixed USDC allowlist.
    pub mint: Account<'info, Mint>,
    #[account(init, payer = shipper, space = 8 + Order::INIT_SPACE,
        seeds = [b"order", shipper.key().as_ref(), &order_id.to_le_bytes()], bump)]
    pub order: Account<'info, Order>,
    #[account(init, payer = shipper, seeds = [b"vault", order.key().as_ref()], bump,
        token::mint = mint, token::authority = order)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint, token::authority = shipper)]
    pub source: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Act<'info> {
    pub actor: Signer<'info>,
    #[account(mut, seeds = [b"order", order.shipper.as_ref(),
        &order.order_id.to_le_bytes()], bump = order.bump)]
    pub order: Account<'info, Order>,
}

#[derive(Accounts)]
pub struct Settle<'info> {
    pub actor: Signer<'info>,
    #[account(mut, seeds = [b"order", order.shipper.as_ref(),
        &order.order_id.to_le_bytes()], bump = order.bump, has_one = mint)]
    pub order: Account<'info, Order>,
    pub mint: Account<'info, Mint>,
    #[account(mut, seeds = [b"vault", order.key().as_ref()], bump,
        token::mint = mint, token::authority = order)]
    pub vault: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint, constraint = carrier_tokens.owner == order.carrier @ EscrowError::Unauthorized)]
    pub carrier_tokens: Account<'info, TokenAccount>,
    #[account(mut, token::mint = mint, constraint = shipper_tokens.owner == order.shipper @ EscrowError::Unauthorized)]
    pub shipper_tokens: Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
}

#[account]
#[derive(InitSpace)]
pub struct Order {
    pub shipper: Pubkey,
    pub carrier: Pubkey,
    pub arbitrator: Pubkey,
    pub mint: Pubkey,
    pub order_id: u64,
    pub amount: u64,
    pub deadline: i64,
    pub agreement_hash: [u8; 32],
    pub evidence_hash: [u8; 32],
    pub bump: u8,
    pub status: Status,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum Status { Funded, Accepted, Disputed, Settled, Refunded }

#[error_code]
pub enum EscrowError {
    #[msg("Actor does not have the required role")]
    Unauthorized,
    #[msg("Invalid state transition")]
    InvalidState,
    #[msg("Amount must be positive and within reserved principal")]
    InvalidAmount,
    #[msg("Acceptance/refund deadline condition failed")]
    Deadline,
    #[msg("Vault does not cover reserved principal")]
    Underfunded,
    #[msg("Shipper, carrier and arbitrator must be distinct")]
    DistinctRoles,
    #[msg("Development token must have six decimals")]
    MintDecimals,
}
