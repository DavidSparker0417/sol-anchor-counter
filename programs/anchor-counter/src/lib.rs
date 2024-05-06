use anchor_lang::prelude::*;

declare_id!("G2jNk58i9U7cDmQHbYQHysuBXmVhaSWCYTZEBkMeTQQ4");

#[program]
pub mod anchor_counter {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
