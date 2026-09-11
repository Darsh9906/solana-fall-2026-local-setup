use anchor_lang::prelude::*;

declare_id!("HBm4C3VuaCyyEhQBGgPj1Dr4a1FhPzYYj4p6igcJXmcr");

#[program]
pub mod hello_solana {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
