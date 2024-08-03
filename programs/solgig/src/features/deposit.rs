use crate::{errors::Errors, state::Job};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        has_one = maker,
        constraint = job_state.maker == maker.key() @ Errors::NotCreator,
        seeds = [b"job", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = job_state.state_bump,
    )]
    pub job_state: Account<'info, Job>,
    #[account(
        mut,
        seeds = [b"vault", job_state.key().as_ref()],
        bump = job_state.vault_bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Deposit<'info> {
    pub fn deposit(&mut self, amount: u64) -> Result<()> {
        require!(self.maker.lamports() > amount, Errors::InsufficientFunds);
        require!(amount > 0, Errors::InvalidDeposit);

        let program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.maker.to_account_info(),
            to: self.vault.to_account_info(),
        };

        let ctx = CpiContext::new(program, accounts);
        transfer(ctx, amount)?;
        Ok(())
    }

    pub fn update_milestone_payments(&mut self) -> Result<()> {
        let balance = self.vault.lamports();

        require!(balance != 0, Errors::InsufficientBalance);

        let milestones_left = self.job_state.milestones - self.job_state.milestone_completed;
        self.job_state.pay_per_milestone = balance / u64::from(milestones_left);
        Ok(())
    }
}
