use anchor_lang::prelude::*;

mod constants;
mod errors;
mod features;
mod state;

use features::*;

declare_id!("9WFWMjBDF32pGEEjP6XHaGUkuQXqcgx1eFjRqLSh2dbA");

#[program]
pub mod solgig {
    use super::*;

    pub fn initialize(ctx: Context<Create>, seed: u64, milestones: u8) -> Result<()> {
        ctx.accounts
            .initialize(seed, ctx.bumps.job_state, ctx.bumps.vault, milestones)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, seed: u64, amount: u64) -> Result<()> {
        ctx.accounts.deposit(amount)?;
        ctx.accounts.update_milestone_payments()?;
        Ok(())
    }

    pub fn assign(ctx: Context<Assign>, seed: u64) -> Result<()> {
        ctx.accounts.assign()?;
        Ok(())
    }

    pub fn submit(ctx: Context<Submit>, seed: u64) -> Result<()> {
        ctx.accounts.submit_milestone()?;
        Ok(())
    }

    pub fn accept_submission(ctx: Context<Handler>, seed: u64) -> Result<()> {
        ctx.accounts.accept_submission()?;
        Ok(())
    }

    pub fn reject_submission(ctx: Context<Handler>, seed: u64) -> Result<()> {
        ctx.accounts.reject_submission()?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, seed: u64) -> Result<()> {
        ctx.accounts.withdraw()?;
        Ok(())
    }

    pub fn cancel(ctx: Context<Cancel>, seed: u64) -> Result<()> {
        ctx.accounts.cancel()?;
        Ok(())
    }
}

// 9WFWMjBDF32pGEEjP6XHaGUkuQXqcgx1eFjRqLSh2dbA
