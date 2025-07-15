use anchor_lang::prelude::*;

declare_id!("F7S6FFRN54sShysxBpBZ5XKWRSuKeucxx8bLzihVAbx1");

pub mod constants;
pub use constants::*;
pub mod error;
pub use error::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
