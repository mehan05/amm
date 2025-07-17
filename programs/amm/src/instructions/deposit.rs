use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken},
    token::{mint_to, transfer, Mint, MintTo, Token, TokenAccount, Transfer},
};
use constant_product_curve::ConstantProduct;


use crate::{ state::*, error::*};

#[derive(Accounts)]
pub struct Deposit<'info>{

    #[account(mut)]
    pub initializer:Signer<'info>,
    pub mint_x: Account<'info,Mint>,
    pub mint_y:Account<'info,Mint>,


    #[account(
        mut,
        has_one = mint_x,
        has_one = mint_y,
        seeds=[b"config",config.seed.to_le_bytes().as_ref()],
        bump = config.config_bump
    )]
    pub config:Account<'info,Config>,


    #[account(
        mut,
        seeds=[b"lp_mint",config.key().as_ref()],
        bump = config.lp_bump
    )]
    pub lp_mint:Account<'info,Mint>,

    #[account(
        mut,
        associated_token::authority = config,
        associated_token::mint = mint_x 
    )]
    pub vault_x:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::authority = config,
        associated_token::mint = mint_y
    )]
    pub vault_y:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_x,
        associated_token::authority = initializer
    )]
    pub user_x:Account<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_y,
        associated_token::authority = initializer
    )]
    pub user_y:Account<'info,TokenAccount>,

    #[account(
        init_if_needed,
        payer  = initializer,
        associated_token::mint = lp_mint,
        associated_token::authority = initializer
    )]
    pub user_lp:Account<'info,TokenAccount>,
    
    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>
}

impl<'info> Deposit<'info>{


    pub fn deposit(&mut self, amount:u64, max_amount_x:u64, max_amount_y:u64 )->Result<()>{
        require!(!self.config.is_locked,AmmError::PoolLocked);
        require!(amount>0,AmmError::InvalidAmount);

        let (x_amount,y_amount) = match self.lp_mint.supply==0 && self.vault_x.amount==0 && self.vault_y.amount==0 {
            true=> (max_amount_x,max_amount_y),
            false=>{
                let amount = ConstantProduct::xy_deposit_amounts_from_l(
                    self.vault_x.amount,
                    self.vault_y.amount,
                    self.lp_mint.supply,
                    amount,
                    6
                ).unwrap();
                (amount.x,amount.y)
            }
        };

        require!(x_amount<=max_amount_x && y_amount<=max_amount_y,AmmError::SlippageExceeded);

        self.deposit_token(true,x_amount)?;
        self.deposit_token(false,y_amount)?;

        self.mint_lp_tokens(amount)?;

        Ok(())

    }


    pub fn deposit_token(&mut self, is_x:bool, amount:u64)->Result<()>{

        let(from, to) = match is_x {
            true => (self.user_x.to_account_info(), self.vault_x.to_account_info()),
            false => (self.user_y.to_account_info(), self.vault_y.to_account_info())
        };

        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = Transfer{
            from,
            to,
            authority:self.initializer.to_account_info()
        };

        let ctx = CpiContext::new(cpi_program,cpi_accounts);
        transfer(ctx,amount)?;

        Ok(())
    }

    pub fn mint_lp_tokens(&mut self, amount:u64)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo{
            to:self.user_lp.to_account_info(),
            mint: self.lp_mint.to_account_info(),
            authority:self.config.to_account_info()
        };


        let seeds = [
            &b"config"[..],
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump]
        ];

        let signer_seed = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seed);

        mint_to(ctx,amount)?;

        Ok(())
    }

}   