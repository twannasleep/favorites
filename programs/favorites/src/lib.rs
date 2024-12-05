use anchor_lang::prelude::*;

declare_id!("FD9Mj12sxbiDR75gmJNXxpgnUBoTXnDuydjgphBpUxJP");

#[program]
pub mod favorites {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
