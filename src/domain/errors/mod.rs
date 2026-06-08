use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompressionError {
    #[error("Error de I/O: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Error de algoritmo: {0}")]
    Algorithm(String),
    
    #[error("Formato inválido: {0}")]
    InvalidFormat(String),
    
    #[error("Timeout: {0}ms")]
    Timeout(u64),
}

pub type Result<T> = std::result::Result<T, CompressionError>;
