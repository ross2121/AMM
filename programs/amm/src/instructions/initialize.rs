use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token::{Mint, Token, TokenAccount}};

use crate::config;
#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Initialize<'info>{
#[account(mut)]
    pub signer:Signer<'info>,
 pub mintx:Account<'info,Mint>,
pub minty:Account<'info,Mint>,
#[account(init,seeds=[b"lp",config.key().as_ref()],bump,mint::decimals=6,mint::authority=config,payer=signer)]
pub lp_token:Account<'info,Mint>,
#[account(init,associated_token::mint=mintx,associated_token::authority=config,payer=signer)]
pub vault_a:Account<'info,TokenAccount>,
#[account(init,associated_token::mint=minty,associated_token::authority=config,payer=signer)]
pub vault_b:Account<'info,TokenAccount>,
#[account(init,seeds=[b"config",seeds.to_le_bytes().as_ref()],bump,payer=signer,space=8+config::INIT_SPACE)]
pub config:Account<'info,config>,
pub system_program:Program<'info,System>,
pub token_program:Program<'info,Token>,
pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info>  Initialize <'info>{
    pub fn initialize(&mut self,seed:u64,fee:u16,authority:Option<Pubkey>,bump:&InitializeBumps)->Result<()>{
        self.config.set_inner(config { seed, authority, mintx:self.mintx.key(), minty:self.minty.key(), fee, locked:false, config_bump:bump.config, lp_bump:bump.lp_token});
        Ok(())
    }
}