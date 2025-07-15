use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken},
    token_interface::{TokenAccount,Mint,TokenInterface,TransferChecked,transfer_checked}
};
use constant_product_curve::ConstantProduct;


use crate::{constants::*, state::*, error::*};

#[derive(Accounts)]
pub struct Deposit<'info>{
     #[account(mut)]
    pub initializer:Signer<'info>,
    pub mint_x:InterfaceAccount<'info,TokenAccount>,
    pub mint_y:InterfaceAccount<'info,TokenAccount>,

    #[account(
        has_one = mint_x,
        has_one = mint_y,
        payer = initializer,
        seeds = [b"config",config.seed.to_le_bytes().as_ref()],
        bump  = config.config_bump,
    )]
    pub config: Account<'info,Config>,

    #[account(
       mut,
        seeds = [b"lp",config.key().as_ref()],
        bump = config.lp_bump,
    )]
    pub lp_mint:Account<'info,Mint>,

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
        associated_token::mint  = mint_x,
        associated_token::authority = user_x
    )]
    pub vault_x:InterfaceAccount<'info,TokenAccount>,

    #[account(
        mut,
        associated_token::mint  = mint_y,
        associated_token::authority = user_y
    )]
    pub vault_y:InterfaceAccount<'info,TokenAccount>,

    
    #[account(
        init_if_needed,
        payer = initializer,
        associated_token::mint = lp_mint,
        associated_token::authority =  initializer
    )]
    pub user_lp:InterfaceAccount<'info,TokenAccount>,


    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    
}

impl<'info> Deposit<'info>{
    pub fn deposit(&mut self, amount:u64, max_x:u64,max_y:u64)->Result<()>{
            require(amount>0,AmmError::InvalidAmount);

            require(self.config.is_locked(),AmmError::PoolLocked);

            let(amount_x,amount_y) = match self.mint_lp.supply==0&&
            self.user_x.amount==0&&self.user_y.amount==0{
                    (true)=>(
                        max_x,
                        max_y,
                    ),

                    (false)=>{
                       let amount = ConstantProduct::xy_deposit_amounts_from_l(
                           self.vault_x.amount,
                           self.vault_y.amount,
                           self.lp_mint.supply,
                           amount,6
                       ).unwrap();
                       
                       (amount.x,amount.y)
                    }

            };

            require!(amount_x<=max_x && amount_y<=max_y,AmmError::SlippageExceeded);

            self.deposit_token(true,amount_x)?;
            self.deposit_token(false,amount_y)?;

            self.mint_lp(amount)?;

    }

    pub fn deposit_token(&mut self,is_x:bool,amount:u64)->Result<()>{
        let (from,to) = match(is_x){
            true=>(
                self.user_x.to_account_info(),
                self.vault_x.to_account_info(),
            ),
            false=>(
                self.user_y.to_account_info(),
                self.vault_y.to_account_info(),
            )
        };

        let cpi_program = self.token_program.to_account_info();
        
        let cpi_accounts = TransferChecked{
            from,
            to,
            authority: self.initializer.to_account_info(),
        };

        let ctx = CpiContext::new(cpi_program,cpi_accounts);
        transfer_checked(ctx,amount)?;
    }

    pub fn mint_lp(&mut self,amount:u64)->Result<()>{
        let cpi_program = self.token_program.to_account_info();

        let cpi_accounts = MintTo{
            mint:self.lp_mint.to_account_info(),
            to:self.user_lp.to_account_info(),
            authority:self.initializer.to_account_info(),
        };
        
        let seeds = [
            &b"config"[..],
            &self.config.key().to_bytes(),
            &[self.config.config_bump]   
        ];

        let signer_seeds = &[&seeds[..]];

        let ctx = CpiContext::new_with_signer(cpi_program,cpi_accounts,signer_seeds);
        mint_to(ctx,amount)?;

    }
}
