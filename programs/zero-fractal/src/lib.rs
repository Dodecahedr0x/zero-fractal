use anchor_lang::prelude::*;
use fractal::create_fractal_pixels;

declare_id!("EmvRvUmV4X7rGai4WQWt9bWLHf2BYBi9aKNTowCwKpUz");

#[program]
pub mod zero_fractal {
    use super::*;

    pub fn create_fractal_on_chain(
        ctx: Context<CreateFractalOnChain>,
        size: u64,
        center_x: u64,
        center_y: u64,
        zoom: u64,
    ) -> Result<()> {
        ctx.accounts.fractal.set_inner(Fractal {
            center_x,
            center_y,
            zoom,
            img: create_fractal_pixels(size, center_x, center_y, zoom),
        });

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(size: u64)]
pub struct CreateFractalOnChain<'info> {
    #[account(
        init,
        payer = payer,
        space = 8 + 24 + 4 + 3 * (size * size) as usize,
    )]
    pub fractal: Account<'info, Fractal>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct Fractal {
    pub center_x: u64,
    pub center_y: u64,
    pub zoom: u64,
    pub img: Vec<[u8; 3]>,
}
