use crate::{errors::Errors, state::Job};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u64)]

pub struct Submit<'info> {
    #[account(
        mut,
        constraint = job_state.developer == developer.key() @ Errors::NotDeveloper,
    )]
    pub developer: Signer<'info>,

    pub maker: SystemAccount<'info>,

    #[account(
        mut,
        constraint = job_state.maker == maker.key() @ Errors::NotCreator,
        seeds = [b"job", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = job_state.state_bump,
    )]
    pub job_state: Account<'info, Job>,
    pub system_program: Program<'info, System>,
}

impl<'info> Submit<'info> {
    pub fn submit_milestone(&mut self) -> Result<()> {
        //There must be no pending submissions
        require!(
            !self.job_state.pending_submission,
            Errors::PendingSubmission
        );
        //There must still be milestones left to complete
        require!(
            self.job_state.milestone_completed < self.job_state.milestones,
            Errors::MilestonesCompleted
        );
        self.job_state.pending_submission = true;
        Ok(())
    }
}
