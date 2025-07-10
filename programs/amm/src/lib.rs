use anchor_lang::prelude::*;

declare_id!("AwovFVc8D64taLRrHjmg4ZeNSh6xnZGTbd2Arv6kbcwd");
pub mod instructions;
pub use instruction::*;
pub mod state;
pub use state::*;

#[program]
pub mod amm {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
