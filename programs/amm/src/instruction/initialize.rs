use anchor_lang::prelude::*;

use anchor_spl::{
    associated_token::{AssociatedToken},
    token_interface::{TokenAccount,Mint,TokenInterface,TransferChecked,transfer_checked}
};

use crate::{constants::*, state::*, error::*};

#[derive(Accounts)]
#[instruction(seed:u8)]
pub struct Initialize<'info> {

    #[account(mut)]
    pub Initializer:Signer<'info>,
    pub mint_x:InterfaceAccount<'info,TokenAccount>,
    pub mint_y:InterfaceAccount<'info,TokenAccount>,

    #[account(
        init,
        space = ANCHOR_DISCRIMINATOR + INIT_SPACE::Config,
        payer = Initializer,
        seeds = [b"config",seed.to_le_bytes().as_ref()],
        bump,
    )]
    pub config: Account<'info,Config>,

    #[account(
        init,
        payer = Initializer,
        seeds = [b"lp",config.key().as_ref()],
        bump,
        mint::decimals = 6,
        mint::authority = config,
    )]
    pub lp_mint:Account<'info,Mint>,

    pub system_program:Program<'info,System>,
    pub token_program:Interface<'info,TokenInterface>,
    pub associated_token_program:Program<'info,AssociatedToken>,
}

impl <'info> Initialize<'info>{
    pub fn initialize(&mut self,seed:u8,fee:u16, authority:Option<Pubkey>, bump:InitializeBumps)->Result<()>{
        require!(fee<=10000,AmmError::InvalidFee);

        self.config.set_inner(Config{
            seed,
            mint_x:self.mint_x.key(),
            mint_y:self.mint_y.key(),
            fee:fee,
            authority:authority,
            config_bump:bump.config,
            lp_bump:bump.lp_mint,

        });

        Ok(())

    }
}