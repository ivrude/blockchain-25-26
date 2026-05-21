#[error_code]
pub mod IpError {
    #[msg("Термін дії запису ще не минув, і ви не є його власником.")]
    CannotDeleteYet,
}