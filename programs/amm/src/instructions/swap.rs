use anchor_lang::{prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{ transfer, transfer_checked, Mint, MintTo, Token, TokenAccount, Transfer}};
use constant_product_curve::{ConstantProduct, LiquidityPair};

use crate::{config, error::AmmError};

#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Swap<'info>{
#[account(mut)]
pub signer:Signer<'info>,
    
pub mintx:Account<'info,Mint>,
pub minty:Account<'info,Mint>,
#[account(mut)]
pub user_x:Account<'info,TokenAccount>,
#[account(mut)]
pub user_y:Account<'info,TokenAccount>,
#[account(init_if_needed,associated_token::mint=lp_token,associated_token::authority=signer,payer=signer)]
pub user_lp:Account<'info,TokenAccount>,
#[account(mut,seeds=[b"lp",config.key().as_ref()],bump=config.lp_bump)]
pub lp_token:Account<'info,Mint>,
#[account(mut,associated_token::mint=mintx,associated_token::authority=config)]
pub vault_x:Account<'info,TokenAccount>,
#[account(mut,associated_token::mint=minty,associated_token::authority=config)]
pub vault_y:Account<'info,TokenAccount>,
#[account(mut,seeds=[b"config",config.seed.to_le_bytes().as_ref()],bump=config.config_bump)]
pub config:Account<'info,config>,
pub system_program:Program<'info,System>,
pub token_program:Program<'info,Token>,
pub associated_token_program:Program<'info,AssociatedToken>
}
impl<'info>  Swap <'info>{
    pub fn swap(&mut self,amount:u64,is_x:bool,min:u64)->Result<()>{
        require!(self.config.locked == false, AmmError::PoolLocked);
       require!(amount!=0,AmmError::InvalidAmount);
       let mut curve=ConstantProduct::init(self.vault_x.amount,self.vault_y.amount, self.lp_token.supply, self.config.fee, None).map_err(|_| AmmError::CurveError)?;
         let isx=   match  is_x {
             true=>LiquidityPair::X,
             false=>LiquidityPair::Y
         };
         let swaps=curve.swap(isx, amount, min).map_err(|_| AmmError::CurveError)?;
        require!(swaps.deposit!=0||swaps.withdraw!=0,AmmError::InvalidAmount);
       
        self.deposittoken(is_x, swaps.deposit)?;
        self.withdrawtoken(!is_x, swaps.withdraw)

    
    }
    pub fn deposittoken(&self,is_x:bool,amount:u64)->Result<()>{
      let (from,to)=match is_x {
          true=>(&self.user_x,&self.vault_x),
          false=>(&self.user_y,&self.vault_y)
      };
      let account=Transfer{
        from:from.to_account_info(),
        to:to.to_account_info(),
        authority:self.signer.to_account_info()
      };
     let cpi_context=CpiContext::new(self.token_program.to_account_info(), account);
      transfer(cpi_context, amount)

    }
    pub fn withdrawtoken(&self,is_x:bool,amount:u64)->Result<()>{
        let (from,to)=match is_x {
            true=>(&self.vault_x,&self.user_x),
            false=>(&self.vault_y,&self.user_y)
        };
        let account=Transfer{
            from:from.to_account_info(),
            to:to.to_account_info(),
            authority:self.config.to_account_info()
        };
        let seeds = &[
            b"config".as_ref(),
            &self.config.seed.to_le_bytes(),
            &[self.config.config_bump]
        ];
        let signer_seeds = &[&seeds[..]];
        let cpi_context=CpiContext::new_with_signer(self.token_program.to_account_info(),account, signer_seeds);
        transfer(cpi_context,amount)
    }
}