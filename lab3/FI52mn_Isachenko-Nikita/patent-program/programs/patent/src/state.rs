
#[account]
pub struct IpRecord {
    pub owner: Pubkey,          // Власник
    pub content_hash: [u8; 32], // Хеш файлу
    pub title: String,          // Назва
    pub created_at: i64,        // Дата створення
    pub expires_at: i64,        // Дата завершення (0 якщо немає)
    pub bump: u8,               // Збережений bump для перевірок
}

#[account]
pub struct LicenseRecord {
    pub ip_record: Pubkey, // Лінк на оригінальний запис IP
    pub licensee: Pubkey,  // Гаманець користувача, якому дали дозвіл
    pub granted_at: i64,   // Дата видачі
    pub expires_at: i64,   // Дата закінчення ліцензії
}
