//! QIP Protocol Core Library
//! 
//! Implements Quantum Interlink Protocol - a cross-chain interoperability system
//! with QSAE state aggregation, HBSC consensus, RLNC networking, and atomic swaps.

pub mod crypto;
pub mod state;
pub mod consensus;
pub mod networking;
pub mod types;
pub mod errors;

use errors::Result;

/// Core configuration for QIP Protocol
#[derive(Debug, Clone)]
pub struct QIPConfig {
    pub validator_count: usize,
    pub consensus_threshold: f64,
    pub finality_time_ms: u64,
    pub max_shards: usize,
}

impl Default for QIPConfig {
    fn default() -> Self {
        Self {
            validator_count: 1000,
            consensus_threshold: 0.667,
            finality_time_ms: 800,
            max_shards: 100,
        }
    }
}

/// QIP Protocol Engine
pub struct QIPEngine {
    config: QIPConfig,
}

impl QIPEngine {
    pub fn new(config: QIPConfig) -> Self {
        Self { config }
    }

    pub fn initialize(&self) -> Result<()> {
        log::info!("Initializing QIP Protocol with {} validators", self.config.validator_count);
        Ok(())
    }

    pub fn get_config(&self) -> &QIPConfig {
        &self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = QIPConfig::default();
        assert_eq!(config.validator_count, 1000);
        assert_eq!(config.max_shards, 100);
    }

    #[test]
    fn test_engine_creation() {
        let engine = QIPEngine::new(QIPConfig::default());
        assert!(engine.initialize().is_ok());
    }
}
