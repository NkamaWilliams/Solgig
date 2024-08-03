use crate::{errors::Errors, state::Job};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Create<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        init,
        payer = maker,
        seeds = [b"job", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = Job::INIT_SPACE,
        bump,
    )]
    pub job_state: Account<'info, Job>,
    #[account(
        seeds = [b"vault", job_state.key().as_ref()],
        bump,
    )]
    pub vault: SystemAccount<'info>,
    pub system_program: Program<'info, System>,
}

impl<'info> Create<'info> {
    pub fn initialize(
        &mut self,
        seed: u64,
        state_bump: u8,
        vault_bump: u8,
        milestones: u8,
    ) -> Result<()> {
        require!(milestones > 0, Errors::NotEnoughMilestones);

        self.job_state.maker = self.maker.key();
        self.job_state.seed = seed;
        self.job_state.pay_per_milestone = 0;
        self.job_state.state_bump = state_bump;
        self.job_state.vault_bump = vault_bump;
        self.job_state.milestones = milestones;
        self.job_state.milestone_completed = 0;
        self.job_state.pending_submission = false;
        self.job_state.task_assigned = false;
        Ok(())
    }
}
