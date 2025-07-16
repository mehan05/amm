#[allow(unexpected_cfgs)]
use anchor_lang::prelude::*;

pub mod constants;
pub use constants::*;

pub mod error;
pub use error::*;

pub mod state;
pub use state::*;

pub mod instructions;
pub use instructions::{deposit::*, initialize::*, swap::*, withdraw::*};
declare_id!("F7S6FFRN54sShysxBpBZ5XKWRSuKeucxx8bLzihVAbx1");


#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, seed:u8, authority:Option<Pubkey>, fee:u16) -> Result<()> {
        ctx.accounts.init(seed,authority,fee,ctx.bumps)?;
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount:u64, max_amount_x:u64, max_amount_y:u64) -> Result<()> {
        ctx.accounts.deposit(amount,max_amount_x,max_amount_y)?;
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount:u64, min_x:u64, min_y:u64) -> Result<()> {
        ctx.accounts.withdraw(amount,min_x,min_y)?;
        Ok(())
    }

    pub fn swap(ctx: Context<Swap>, amount:u64, is_x:bool,min:u64) -> Result<()> {
        ctx.accounts.swap(amount,is_x,min)?;
        Ok(())
    }


   
}


