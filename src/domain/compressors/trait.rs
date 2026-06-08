use std::io::{Read, Write};
use crate::domain::errors::CompressionError;

#[derive(Debug, Clone)]
pub struct CompressionResult {
    pub original_size: u64,
    pub compressed_size: u64,
    pub compression_ratio: f64,
    pub time_ms: u64,
    pub algorithm: String,
}

pub trait Compressor: Send + Sync {
    fn name(&self) -> &str;
    fn level(&self) -> u8;
    fn compress<R: Read, W: Write>(
        &self,
        source: &mut R,
        destination: &mut W,
    ) -> Result<CompressionResult, CompressionError>;
    fn decompress<R: Read, W: Write>(
        &self,
        source: &mut R,
        destination: &mut W,
    ) -> Result<CompressionResult, CompressionError>;
    fn dictionary_size(&self) -> u64;
}
