pub mod domain;
pub mod application;
pub mod infrastructure;

pub use domain::compressors::r#trait::{Compressor, CompressionResult};
pub use domain::errors::{CompressionError, Result};
