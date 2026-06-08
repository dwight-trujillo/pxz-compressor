use std::path::{Path, PathBuf};
use std::time::Instant;
use crate::domain::compressors::r#trait::Compressor;
use crate::domain::errors::Result;

#[derive(Clone)]
pub struct CompressFileConfig {
    pub algorithm: Box<dyn Compressor>,
    pub output_suffix: String,
    pub delete_original: bool,
    pub timeout_seconds: u64,
}

impl Default for CompressFileConfig {
    fn default() -> Self {
        Self {
            algorithm: Box::new(crate::infrastructure::compressors::ZstdCompressor::new(10)),
            output_suffix: ".pxz".to_string(),
            delete_original: false,
            timeout_seconds: 300,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompressionReport {
    pub input_path: PathBuf,
    pub output_path: PathBuf,
    pub original_size: u64,
    pub compressed_size: u64,
    pub ratio: f64,
    pub time_ms: u64,
    pub throughput_mb_s: f64,
    pub algorithm: String,
}

pub struct CompressFileUseCase {
    config: CompressFileConfig,
}

impl CompressFileUseCase {
    pub fn new(config: CompressFileConfig) -> Self {
        Self { config }
    }
    
    pub async fn execute(&self, input_path: impl AsRef<Path>) -> Result<CompressionReport> {
        let input_path = input_path.as_ref();
        let metadata = tokio::fs::metadata(input_path).await?;
        let original_size = metadata.len();
        let output_path = input_path.with_extension(&self.config.output_suffix);
        let start = Instant::now();
        let data = tokio::fs::read(input_path).await?;
        let mut compressed = Vec::new();
        let mut reader = std::io::Cursor::new(&data);
        let comp_result = self.config.algorithm.compress(&mut reader, &mut compressed)?;
        tokio::fs::write(&output_path, &compressed).await?;
        let elapsed = start.elapsed().as_millis() as u64;
        let throughput = (original_size as f64 / 1024.0 / 1024.0) / (elapsed as f64 / 1000.0);
        Ok(CompressionReport {
            input_path: input_path.to_path_buf(),
            output_path,
            original_size,
            compressed_size: compressed.len() as u64,
            ratio: compressed.len() as f64 / original_size as f64,
            time_ms: elapsed,
            throughput_mb_s: throughput,
            algorithm: self.config.algorithm.name().to_string(),
        })
    }
}
