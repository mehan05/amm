use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken},
    token_interface::{TokenAccount,Mint,TokenInterface,TransferChecked,transfer_checked}
};

use crate::{constants::*, state::*, error::*};

#[derive(Accounts)]
pub struct Withdraw<'info>{
    #[account(mut)]
    pub initializer:Signer<'info>,
    pub mint_x:InterfaceAccount<'info,TokenAccount>,
    pub mint_y:InterfaceAccount<'info,TokenAccount>,

     #[account(
        has_one = mint_x,
        has_one = mint_y,
        payer = initializer,
        seeds = [b"config",config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump,
    )]
    pub config: Account<'info,Config>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = config,
    )]
    pub vault_x:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = config,
    )]
    pub vault_y:InterfaceAccount<'info,TokenAccount>,


    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = initializer,
    )]
    pub user_x:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = initializer,
    )]
    pub user_y:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        seeds=[b"lp",config.key().as_ref()],
        bump = config.lp_bump,
        
    )]
    pub lp_mint:InterfaceAccount<'info,Mint>,


    #[account(
        mut,
        associated_token::mint = lp_mint,
        associated_token::authority = initializer,
    )]
    pub user_lp:InterfaceAccount<'info,TokenAccount>,

    pub system_program:Program<'info,System>,
    pub token_program:Program<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,

}

impl<'info> Withdraw<'info>{

    pub fn withdraw(&mut self, amount:u64, min_x:u64, min_y:u64)->Result<()>{
        require!(amount>0,AmmError::InvalidAmount);
        require!(self.config.is_locked,AmmError::PoolLocked);

        let (amount_x,amount_y) = 

    }



    pub fn withdraw_token(&self,isX:bool,amount:u64)->Result<()>{
        let (from ,to) = match (is_x){
            true => (self.vault_x.to_account_info(),self.user_x.to_account_info()),
            false => (self.vault_y.to_account_info(),self.user_y.to_account_info()),
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = TransferChecked{
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

        transfer_checked(ctx,amount)?;

    }


    pub fn burn(& mut self,amount:u64)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts  =Burn{
            from:self.user_lp.to_account_info(),
            mint:self.mint_lp.to_account_info(),
            authority:self.initializer.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program,cpi_accounts);
        burn(ctx,amount)?;
    }

}

