use crate::{errors::Errors, state::Job};
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[instruction(seed: u64)]

pub struct Assign<'info> {
    #[account()]
    pub maker: Signer<'info>,
    pub developer: SystemAccount<'info>,
    #[account(
        mut,
        constraint = job_state.maker == maker.key() @ Errors::NotCreator,
        seeds = [b"job", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        bump = job_state.state_bump,
    )]
    pub job_state: Account<'info, Job>,
    pub system_program: Program<'info, System>,
}

impl<'info> Assign<'info> {
    pub fn assign(&mut self) -> Result<()> {
        if !self.job_state.task_assigned {
            self.job_state.developer = self.developer.key();
            self.job_state.task_assigned = true;
        }
        Ok(())
    }
}
