use anchor_lang::{prelude::*};
use anchor_spl::{associated_token::AssociatedToken, token::{burn, mint_to, transfer, transfer_checked, Burn, Mint, MintTo, Token, TokenAccount, Transfer}};
use constant_product_curve::ConstantProduct;

use crate::{config, error::AmmError};


#[derive(Accounts)]
#[instruction(seeds:u64)]
pub struct Withdraw<'info>{
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
impl<'info>  Withdraw <'info>{
    pub fn withdraw(&mut self,amount:u64,min_x:u64,min_y:u64)->Result<()>{
        require!(self.config.locked == false, AmmError::PoolLocked);
       require!(amount!=0,AmmError::InvalidAmount);
     let amounts=ConstantProduct::xy_withdraw_amounts_from_l(self.vault_x.amount, self.vault_y.amount, self.lp_token.supply, amount, 6).map_err(|_|AmmError::CurveError)?;
       require!(amounts.x>=min_x && amounts.y>=min_y,AmmError::SlippageExceded);
       self.withdrawtoken(true, amounts.x)?;
       self.withdrawtoken(false,amounts.y)?;
       self.burn(amount)
    
    }
    pub fn withdrawtoken(&self,is_x:bool,amount:u64)->Result<()>{
      let (to,from)=match is_x {
          true=>(&self.user_x,&self.vault_x),
          false=>(&self.user_y,&self.vault_y)
      };
      let seeds = &[
        b"config".as_ref(),
        &self.config.seed.to_le_bytes(),
        &[self.config.config_bump]
    ];
    let signer_seeds = &[&seeds[..]];
      let account=Transfer{
        from:from.to_account_info(),
        to:to.to_account_info(),
        authority:self.config.to_account_info()
      };
     let cpi_context=CpiContext::new_with_signer(self.token_program.to_account_info(), account,signer_seeds);
      transfer(cpi_context, amount)

    }
    pub fn burn(&self,amount:u64)->Result<()>{
        let mint=Burn{mint:self.lp_token.to_account_info(),from:self.user_lp.to_account_info(),authority:self.signer.to_account_info()};
        let cpi_context=CpiContext::new(self.token_program.to_account_info(),mint);
       burn(cpi_context, amount)
    }
}