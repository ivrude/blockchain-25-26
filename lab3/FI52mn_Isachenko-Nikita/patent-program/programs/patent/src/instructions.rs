
#[derive(Accounts)]
#[instruction(content_hash: [u8; 32])]
pub struct RegisterIp<'info> {
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + (4 + 64) + 8 + 8 + 1, 
        seeds = [b"ip_record", owner.key().as_ref(), content_hash.as_ref()],
        bump
    )]
    pub ip_record: Account<'info, IpRecord>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(content_hash: [u8; 32])]
pub struct GrantLicense<'info> {
    #[account(
        seeds = [b"ip_record", owner.key().as_ref(), content_hash.as_ref()],
        bump = ip_record.bump,
        has_one = owner, // Перевірка, що саме власник ІВ підписує транзакцію
    )]
    pub ip_record: Account<'info, IpRecord>,
    
    #[account(
        init,
        payer = owner,
        space = 8 + 32 + 32 + 8 + 8,
        // PDA ліцензії прив'язаний до запису ІВ та ліцензіата
        seeds = [b"license", ip_record.key().as_ref(), licensee.key().as_ref()],
        bump
    )]
    pub license_record: Account<'info, LicenseRecord>,
    
    /// Гаманець, якому надається дозвіл
    /// CHECK: Безпечно, використовується лише як public key для створення зв'язку
    pub licensee: AccountInfo<'info>,
    
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(content_hash: [u8; 32])]
pub struct DeleteIp<'info> {
    #[account(
        mut,
        // Спроба знайти запис за сидами власника. Якщо видаляє не власник, 
        // валідація пройде завдяки передачі оригінальних сидів клієнтом
        seeds = [b"ip_record", ip_record.owner.as_ref(), content_hash.as_ref()],
        bump = ip_record.bump,
        close = receiver // Закриває акаунт і повертає SOL (Ренту) на цей гаманець
    )]
    pub ip_record: Account<'info, IpRecord>,
    
    #[account(mut)]
    pub signer: Signer<'info>, // Той, хто ініціює видалення (власник або будь-хто після expiry)
    
    #[account(mut)]
    /// CHECK: Отримувач поверненої ренти (зазвичай збігається з власником IP)
    pub receiver: AccountInfo<'info>,
}