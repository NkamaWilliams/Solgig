use crate::{errors::Errors, state::Job};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(seed: u64)]

pub struct Withdraw<'info> {
    #[account(mut)]
    pub developer: Signer<'info>,
    pub maker: SystemAccount<'info>,
    #[account(
        mut,
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

impl<'info> Withdraw<'info> {
    pub fn withdraw(&mut self) -> Result<()> {
        require!(
            self.job_state.milestone_completed > 0,
            Errors::TaskIncomplete
        );
        require!(
            self.job_state.developer == self.developer.key(),
            Errors::InvalidAccount
        );

        let program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.developer.to_account_info(),
        };
        let seeds = &[
            b"vault",
            self.job_state.to_account_info().key.as_ref(),
            &[self.job_state.vault_bump],
        ];
        let signer_seeds = &[&seeds[..]];
        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);
        let pay =
            self.job_state.pay_per_milestone * (u64::from(self.job_state.milestone_completed));

        transfer(ctx, pay)?;

        //Adjust milestone and milestone_completed values to prevent continuous withdrawal
        self.job_state.milestones -= self.job_state.milestone_completed;
        self.job_state.milestone_completed = 0;
        Ok(())
    }
}
