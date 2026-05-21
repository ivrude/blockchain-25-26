mod error;
mod instructions;
mod state;

use anchor_lang::prelude::*;

declare_id!("3k4bP9igsmCH3es6vUeHLbsch59Z2jXQGoo7VJTFFuLJ");

#[program]
pub mod ip_protection {
    use super::*;

    pub fn register_ip(
        ctx: Context<RegisterIp>,
        content_hash: [u8; 32],
        title: String,
        expires_at: i64,
    ) -> Result<()> {
        let ip_record = &mut ctx.accounts.ip_record;
        let clock = Clock::get()?;

        ip_record.owner = ctx.accounts.owner.key();
        ip_record.content_hash = content_hash;
        ip_record.title = title;
        ip_record.created_at = clock.unix_timestamp;
        ip_record.expires_at = expires_at;
        ip_record.bump = ctx.bumps.ip_record;

        msg!("IP Registered: {}", ip_record.title);
        Ok(())
    }

    pub fn grant_license(
        ctx: Context<GrantLicense>,
        _content_hash: [u8; 32], 
        expires_at: i64,        
    ) -> Result<()> {
        let license = &mut ctx.accounts.license_record;
        let clock = Clock::get()?;

        license.ip_record = ctx.accounts.ip_record.key();
        license.licensee = ctx.accounts.licensee.key();
        license.granted_at = clock.unix_timestamp;
        license.expires_at = expires_at;

        msg!("License granted to: {}", license.licensee);
        Ok(())
    }

    pub fn delete_ip(ctx: Context<DeleteIp>, _content_hash: [u8; 32]) -> Result<()> {
        let ip_record = &ctx.accounts.ip_record;
        let clock = Clock::get()?;
        let current_wallet = ctx.accounts.signer.key();

        if current_wallet != ip_record.owner {
            require!(ip_record.expires_at > 0, IpError::CannotDeleteYet);
            require!(clock.unix_timestamp > ip_record.expires_at, IpError::CannotDeleteYet);
        }

        msg!("IP Record deleted. Rent funds returned.");
        Ok(())
    }
}
