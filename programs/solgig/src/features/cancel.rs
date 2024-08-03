use crate::{errors::Errors, state::Job};
use anchor_lang::{
    prelude::*,
    system_program::{transfer, Transfer},
};

#[derive(Accounts)]
#[instruction(seed:u64)]
pub struct Cancel<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,
    #[account(
        mut,
        close = maker,
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

impl<'info> Cancel<'info> {
    pub fn cancel(&mut self) -> Result<()> {
        require!(
            self.job_state.milestone_completed == 0,
            Errors::DeveloperFundsUnwithdrawn
        );
        let program = self.system_program.to_account_info();
        let accounts = Transfer {
            from: self.vault.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let seed = &[
            b"vault",
            self.job_state.to_account_info().key.as_ref(),
            &[self.job_state.vault_bump],
        ];
        let signer_seeds = &[&seed[..]];

        let ctx = CpiContext::new_with_signer(program, accounts, signer_seeds);

        transfer(ctx, self.vault.lamports())?;
        Ok(())
    }
}
