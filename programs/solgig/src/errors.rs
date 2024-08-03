use anchor_lang::error_code;

#[error_code]
pub enum Errors {
    #[msg("Default error!")]
    DefaultError,
    #[msg("Insufficient funds!")]
    InsufficientFunds,
    #[msg("Cannot deposit nothing!")]
    InvalidDeposit,
    #[msg("All Milestones Completed!")]
    MilestonesCompleted,
    #[msg("Account requesting withdrawal not recognized!")]
    InvalidAccount,
    #[msg("Job not completed!")]
    TaskIncomplete,
    #[msg("Insufficient balance!")]
    InsufficientBalance,
    #[msg("Requires a minimum of 1 Milestone")]
    NotEnoughMilestones,
    #[msg("Not the creator!")]
    NotCreator,
    #[msg("Not the approved developer!")]
    NotDeveloper,
    #[msg("Developer still has unwithdrawn funds!")]
    DeveloperFundsUnwithdrawn,
    #[msg("No pending submission to approve")]
    NoSubmission,
    #[msg("There is a pending submission to approve")]
    PendingSubmission,
}
