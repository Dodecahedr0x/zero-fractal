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
            owner: ctx.accounts.owner.key(),
            verified: true,
            center_x,
            center_y,
            zoom,
            img: create_fractal_pixels(size, center_x, center_y, zoom),
        });

        Ok(())
    }

    pub fn create_zk_fractal(
        ctx: Context<CreateZkFractal>,
        size: u64,
        center_x: u64,
        center_y: u64,
        zoom: u64,
    ) -> Result<()> {
        let mut img = Vec::new();
        img.resize((size * size) as usize, [0, 0, 0]);

        ctx.accounts.fractal.set_inner(Fractal {
            owner: ctx.accounts.owner.key(),
            verified: false,
            center_x,
            center_y,
            zoom,
            img,
        });

        Ok(())
    }

    pub fn update_zk_fractal(
        ctx: Context<UpdateZkFractal>,
        offset: u64,
        pixels: Vec<[u8; 3]>,
    ) -> Result<()> {
        let offset = offset as usize;
        ctx.accounts.fractal.img[offset..(offset + pixels.len())].copy_from_slice(&pixels);

        Ok(())
    }

    pub fn verify_zk_fractal(ctx: Context<VerifyZkFractal>, proof: Vec<u8>) -> Result<()> {
        unimplemented!();
    }
}

#[derive(Accounts)]
#[instruction(size: u64)]
pub struct CreateFractalOnChain<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 57 + 4 + 3 * (size * size) as usize,
    )]
    pub fractal: Account<'info, Fractal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(size: u64)]
pub struct CreateZkFractal<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 57 + 4 + 3 * (size * size) as usize,
    )]
    pub fractal: Account<'info, Fractal>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateZkFractal<'info> {
    #[account(
        mut,
        has_one = owner,
        constraint = !fractal.verified,
    )]
    pub fractal: Account<'info, Fractal>,

    #[account(mut)]
    pub owner: Signer<'info>,
}

#[derive(Accounts)]
pub struct VerifyZkFractal<'info> {
    #[account(
        mut,
        constraint = !fractal.verified,
    )]
    pub fractal: Account<'info, Fractal>,
}

#[account]
pub struct Fractal {
    pub owner: Pubkey,
    pub verified: bool,
    pub center_x: u64,
    pub center_y: u64,
    pub zoom: u64,
    pub img: Vec<[u8; 3]>,
}
