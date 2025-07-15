use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]

pub struct Config{
    pub seed:u8,
    pub mint_x:Pubkey,
    pub mint_y:Pubkey,
    pub config_bump:u8,
    pub lp_bump:u8,
    pub fee:u16,
    pub authority:Option<Pubkey>,
    pub is_locked:bool
}