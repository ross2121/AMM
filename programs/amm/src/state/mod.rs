use anchor_lang::prelude::*;
#[account]
#[derive(InitSpace)]
pub struct config{
    pub seed:u64,
    pub authority:Option<Pubkey>,
    pub mintx:Pubkey,
    pub minty:Pubkey,
    pub fee:u16,
    pub  locked:bool,
    pub config_bump:u8,
    pub lp_bump:u8
}