use anchor_lang::prelude::*;

declare_id!("EmvRvUmV4X7rGai4WQWt9bWLHf2BYBi9aKNTowCwKpUz");

#[program]
pub mod zero_fractal {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
