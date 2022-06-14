use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod trait_test {
    use super::*;

    pub fn initialize1(ctx: Context<Initialize1>) -> Result<()> {
        Ok(())
    }

    pub fn initialize2(ctx: Context<Initialize2>) -> Result<()> {
        Ok(())
    }

    pub fn name1(ctx: Context<Name1>, name: String) -> Result<()> {
        handler(&mut ctx.accounts.storage_account_1, name)
    }

    pub fn name2(ctx: Context<Name2>, name: String) -> Result<()> {
        handler(&mut ctx.accounts.storage_account_2, name)
    }
}

#[derive(Accounts)]
pub struct Initialize1<'info> {
    #[account(
        init,
        payer = payer,
        seeds = ["V1".as_bytes()],
        bump,
        space = 2000,
    )]
    pub storage_account_1: Box<Account<'info, StorageAccountV1>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}


#[derive(Accounts)]
pub struct Initialize2<'info> {
    #[account(
        init,
        payer = payer,
        seeds = ["V2".as_bytes()],
        bump,
        space = 100,
    )]
    pub storage_account_2: Box<Account<'info, StorageAccountV2>>,
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
pub struct StorageAccountV1 {
    pub lots_of_things: [u8; 512],
    pub name: String,
}

#[account]
pub struct StorageAccountV2 {
    pub lots_less_things: [u8; 32],
    pub new_name_field: String,
}

#[derive(Accounts)]
pub struct Name1<'info> {
    #[account(
        mut,
        seeds = ["V1".as_bytes()],
        bump
    )]
    pub storage_account_1: Account<'info, StorageAccountV1>
}


#[derive(Accounts)]
pub struct Name2<'info> {
    #[account(
        mut,
        seeds = ["V2".as_bytes()],
        bump
    )]
    pub storage_account_2: Account<'info, StorageAccountV2>
}

trait StorageAccount {
    fn set_name(&mut self, name: String) -> Result<()>;
    fn get_name(&self) -> String;
}
impl StorageAccount for StorageAccountV1 {
    fn set_name(&mut self, name: String) -> Result<()> {
        self.name = name;
        Ok(())
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

}

impl<'info> StorageAccount for Account<'info, StorageAccountV1> {
    fn set_name(&mut self, name: String) -> Result<()> {
        self.name = name;
        Ok(())
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }

}


impl StorageAccount for StorageAccountV2 {
    fn set_name(&mut self, name: String) -> Result<()> {
        self.new_name_field = name;
        Ok(())
    }
    fn get_name(&self) -> String {
        self.new_name_field.clone()
    }
}

impl<'info> StorageAccount for Account<'info, StorageAccountV2> {
    fn set_name(&mut self, name: String) -> Result<()> {
        self.new_name_field = name;
        Ok(())
    }
    fn get_name(&self) -> String {
        self.new_name_field.clone()
    }
}


fn handler<'info>(storage_account: &mut impl StorageAccount, name: String) -> Result<()> {
    msg!("setting name to {}", &name);
    storage_account.set_name(name)
}
