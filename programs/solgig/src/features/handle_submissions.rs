use crate::{errors::Errors, state::Job};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u64)]

pub struct Handler<'info> {
    pub maker: Signer<'info>,
    #[account(
        mut,
        constraint = job_state.maker == maker.key() @ Errors::NotCreator,
        seeds = [b"job", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = job_state.state_bump
    )]
    pub job_state: Account<'info, Job>,
    pub system_program: Program<'info, System>,
}

impl<'info> Handler<'info> {
    pub fn accept_submission(&mut self) -> Result<()> {
        require!(self.job_state.pending_submission, Errors::NoSubmission);
        self.job_state.pending_submission = false;
        self.job_state.milestone_completed += 1;
        Ok(())
    }

    pub fn reject_submission(&mut self) -> Result<()> {
        require!(self.job_state.pending_submission, Errors::NoSubmission);
        self.job_state.pending_submission = false;
        Ok(())
    }
}
