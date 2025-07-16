use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken},
    token::{Burn,burn, transfer, Mint, Token, TokenAccount, Transfer},
};

use crate::{ state::*, error::*};
use constant_product_curve::ConstantProduct;

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub initializer:Signer<'info>,
    pub mint_x:Account<'info,TokenAccount>,
    pub mint_y:Account<'info,TokenAccount>,

     #[account(
        has_one = mint_x,
        has_one = mint_y,
        seeds = [b"config",config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info,Config>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y:Account<'info,TokenAccount>,


    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = initializer,
    )]
    pub user_x:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = initializer,
    )]
    pub user_y:Account<'info,TokenAccount>,

    #[account(
        mut,
        seeds=[b"lp_mint",config.key().as_ref()],
        bump = config.lp_bump,
        
    )]
    pub lp_mint:Account<'info,Mint>,


    #[account(
        mut,
        associated_token::mint = lp_mint,
        associated_token::authority = initializer,
    )]
    pub user_lp:Account<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,

}

impl<'info> Withdraw<'info>{

    pub fn withdraw(&mut self, amount:u64, min_x:u64, min_y:u64)->Result<()>{
        require!(amount>0,AmmError::InvalidAmount);
        require!(self.config.is_locked,AmmError::PoolLocked);

        let amounts = ConstantProduct::xy_withdraw_amounts_from_l(
            self.vault_x.amount,
            self.vault_y.amount,
            self.lp_mint.supply,
            amount,
            6
        ).map_err(AmmError::from)?;

        require!(amounts.x>=min_x && amounts.y>=min_y,AmmError::SlippageExceeded);
        
        self.withdraw_token(true,amounts.x)?;
        self.withdraw_token(false,amounts.y)?;

        self.burn(amount)?;
        Ok(())

    }



    pub fn withdraw_token(&self,is_x:bool,amount:u64)->Result<()>{
        let (from ,to) = match is_x {
            true => (self.vault_x.to_account_info(),self.user_x.to_account_info()),
            false => (self.vault_y.to_account_info(),self.user_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from, 
            to, authority:self.config.to_account_info(),

        };

        let seeds = [
            &b"config"[..],
            &self.config.key().to_bytes(),
            &[self.config.config_bump]   
        ];
        
        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);

        transfer(ctx,amount)?;

        Ok(())
    }


    pub fn burn(& mut self,amount:u64)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts  =Burn{
            from:self.user_lp.to_account_info(),
            mint:self.lp_mint.to_account_info(),
            authority:self.initializer.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program,cpi_accounts);
        burn(ctx,amount)?;

        Ok(())
    }

}

