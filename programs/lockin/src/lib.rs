use anchor_lang::prelude::*;

declare_id!("CsgFGJrJ1feCy1m2yXmD1TEKZNXSfZivY1oEvu7GCW81");

#[program]
pub mod lockin {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
