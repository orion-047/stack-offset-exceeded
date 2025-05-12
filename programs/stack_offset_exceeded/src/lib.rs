use anchor_lang::prelude::*;

declare_id!("7Dy5JJRKeHm2QdfiHyBtQJjq9W71mta3tuUrqZY1ML1s");

#[program]
pub mod stack_offset_exceeded {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
