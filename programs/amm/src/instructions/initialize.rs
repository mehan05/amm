use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken},
    token::{ Mint, Token, TokenAccount,},
};

use crate::{constants::*, state::*};

#[derive(Accounts)]
#[instruction(seed:u8)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub initializer:Signer<'info>,
    pub mint_x: Account<'info,TokenAccount>,
    pub mint_y:Account<'info,TokenAccount>,

    #[account(
        init,
        payer = initializer,
        space=ANCHOR_DISCRIMINATOR + Config::INIT_SPACE,
        seeds=[b"config",seed.to_le_bytes().as_ref()],
        bump
    )]
    pub config:Account<'info,Config>,

    #[account(
        init,
        payer = initializer,
        seeds=[b"lp_mint",config.key().as_ref()],
        mint::decimals = 6,
        mint::authority = config,
        bump,
    )]
    pub lp_mint:Account<'info,Mint>,

    #[account(
        init,
        payer = initializer,
        associated_token::authority = config,
        associated_token::mint = mint_x 
    )]
    pub vault_x:Account<'info,TokenAccount>,

    #[account(
        init,
        payer = initializer,
        associated_token::authority = config,
        associated_token::mint = mint_y
    )]
    pub vault_y:Account<'info,TokenAccount>,

    pub token_program:Program<'info,Token>,
    pub associated_token_program:Program<'info,AssociatedToken>,
    pub system_program:Program<'info,System>
}

impl<'info> Initialize<'info>{
    pub fn init(&mut self, seed:u8, authority:Option<Pubkey>, fee:u16,bumps:InitializeBumps)->Result<()>{

        self.config.set_inner(Config{
            seed,
            mint_x:self.mint_x.key(),
            mint_y:self.mint_y.key(),
            fee,
            authority,
            is_locked:false,
            config_bump:bumps.config,
            lp_bump:bumps.lp_mint
        });

        Ok(())

    }
}