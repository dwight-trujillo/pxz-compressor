use std::io::{Read, Write};
use crate::domain::compressors::r#trait::{Compressor, CompressionResult};
use crate::domain::errors::{CompressionError, Result};

pub struct Lzma2Compressor {
    level: u8,
    dictionary_size: u64,
}

impl Lzma2Compressor {
    pub fn new(level: u8) -> Self {
        let l = level.clamp(1, 9);
        Self {
            level: l,
            dictionary_size: match l {
                1 => 256 * 1024,
                5 => 16 * 1024 * 1024,
                9 => 128 * 1024 * 1024,
                _ => 8 * 1024 * 1024,
            },
        }
    }
}

impl Compressor for Lzma2Compressor {
    fn name(&self) -> &str { "LZMA2" }
    fn level(&self) -> u8 { self.level }
    
    fn compress<R: Read, W: Write>(&self, source: &mut R, dest: &mut W) -> Result<CompressionResult> {
        let start = std::time::Instant::now();
        let mut buffer = Vec::new();
        source.read_to_end(&mut buffer)?;
        let original_size = buffer.len();
        let options = lzma_rs::LzmaOptions::new_preset(self.level as u32)
            .map_err(|e| CompressionError::Algorithm(format!("LZMA error: {}", e)))?;
        let compressed_size = lzma_rs::lzma_compress(&mut std::io::Cursor::new(&buffer), dest, &options)
            .map_err(|e| CompressionError::Algorithm(format!("LZMA error: {}", e)))?;
        Ok(CompressionResult {
            original_size: original_size as u64,
            compressed_size: compressed_size as u64,
            compression_ratio: compressed_size as f64 / original_size as f64,
            time_ms: start.elapsed().as_millis() as u64,
            algorithm: format!("LZMA2 nivel {}", self.level),
        })
    }
    
    fn decompress<R: Read, W: Write>(&self, source: &mut R, dest: &mut W) -> Result<CompressionResult> {
        let start = std::time::Instant::now();
        lzma_rs::lzma_decompress(source, dest)
            .map_err(|e| CompressionError::Algorithm(format!("LZMA decode error: {}", e)))?;
        Ok(CompressionResult {
            original_size: 0,
            compressed_size: 0,
            compression_ratio: 0.0,
            time_ms: start.elapsed().as_millis() as u64,
            algorithm: "LZMA2".to_string(),
        })
    }
    
    fn dictionary_size(&self) -> u64 { self.dictionary_size }
}
