use anchor_lang::prelude::*;

declare_id!("Fn1eqswyWzygRwt83wwuGycqUQvUFmPDoMTGB3oeaYwx");

#[program]
pub mod solana_zero_copy_toolkit {
    use super::*;

    pub fn initialize_state(ctx: Context<InitializeState>, data: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.data = data;
        state.owner = *ctx.accounts.owner.key;
        Ok(())
    }

    pub fn update_state(ctx: Context<UpdateState>, new_data: u64) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.data = new_data;
        Ok(())
    }
}

#[account(zero_copy)]
pub struct ToolkitState {
    pub data: u64,
    pub owner: Pubkey,
}

#[derive(Accounts)]
pub struct InitializeState<'info> {
    #[account(init, payer = owner, space = 8 + std::mem::size_of::<ToolkitState>())]
    pub state: Account<'info, ToolkitState>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateState<'info> {
    #[account(mut)]
    pub state: Account<'info, ToolkitState>,
    pub owner: Signer<'info>,
}
