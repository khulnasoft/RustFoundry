#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Telemetry error: {0}")]
    Telemetry(#[from] telemetry::Error),
    
    #[error("Security error: {0}")]
    Security(#[from] security::Error),
    
    #[error("Settings error: {0}")]
    Settings(#[from] settings::Error),
}

// Standard result type
pub type Result<T> = std::result::Result<T, Error>;

// Error context helper
pub trait ErrorExt<T> {
    fn context(self, ctx: &str) -> Result<T>;
}
