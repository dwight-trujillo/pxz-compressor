use std::io::{Read, Write};
use crate::domain::compressors::r#trait::{Compressor, CompressionResult};
use crate::domain::errors::{CompressionError, Result};

pub struct ZstdCompressor {
    level: u8,
    dictionary_size: u64,
}

impl ZstdCompressor {
    pub fn new(level: u8) -> Self {
        Self {
            level: level.clamp(1, 22),
            dictionary_size: 1_000_000,
        }
    }
}

impl Compressor for ZstdCompressor {
    fn name(&self) -> &str { "Zstandard" }
    fn level(&self) -> u8 { self.level }
    
    fn compress<R: Read, W: Write>(&self, source: &mut R, dest: &mut W) -> Result<CompressionResult> {
        let start = std::time::Instant::now();
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;
        let original_size = buffer.len();
        let compressed = zstd::encode_all(&buffer[..], self.level as i32)
            .map_err(|e| CompressionError::Algorithm(format!("Zstd error: {}", e)))?;
        dest.write_all(&compressed)?;
        Ok(CompressionResult {
            original_size: original_size as u64,
            compressed_size: compressed.len() as u64,
            compression_ratio: compressed.len() as f64 / original_size as f64,
            time_ms: start.elapsed().as_millis() as u64,
            algorithm: format!("Zstandard nivel {}", self.level),
        })
    }
    
    fn decompress<R: Read, W: Write>(&self, source: &mut R, dest: &mut W) -> Result<CompressionResult> {
        let start = std::time::Instant::now();
        let mut compressed = Vec::new();
        source.read_to_end(&mut compressed)?;
        let decompressed = zstd::decode_all(&compressed[..])
            .map_err(|e| CompressionError::Algorithm(format!("Zstd decode error: {}", e)))?;
        dest.write_all(&decompressed)?;
        Ok(CompressionResult {
            original_size: decompressed.len() as u64,
            compressed_size: compressed.len() as u64,
            compression_ratio: compressed.len() as f64 / decompressed.len() as f64,
            time_ms: start.elapsed().as_millis() as u64,
            algorithm: "Zstandard".to_string(),
        })
    }
    
    fn dictionary_size(&self) -> u64 { self.dictionary_size }
}
