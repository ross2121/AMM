use anchor_lang::prelude::*;

declare_id!("AwovFVc8D64taLRrHjmg4ZeNSh6xnZGTbd2Arv6kbcwd");
pub mod instructions;
pub use instructions::*;
pub mod state;
pub use state::*;
pub mod error;
pub use error::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        seed: u64,
        fee: u16,
        authority: Option<Pubkey>
    ) -> Result<()> {
        ctx.accounts.initialize(seed, fee, authority, &ctx.bumps)
    }

    pub fn deposit(
        ctx: Context<Deposit>,
        amount: u64,
        max_x: u64,
        max_y: u64
    ) -> Result<()> {
        ctx.accounts.deposit(amount, max_x, max_y)
    }

    pub fn swap(
        ctx: Context<Swap>,
        amount: u64,
        is_x: bool,
        min_receive: u64
    ) -> Result<()> {
        ctx.accounts.swap(amount, is_x, min_receive)
    }

    pub fn withdraw(
        ctx: Context<Withdraw>,
        amount: u64,
        min_x: u64,
        min_y: u64
    ) -> Result<()> {
        ctx.accounts.withdraw(amount, min_x, min_y)
    }
}

