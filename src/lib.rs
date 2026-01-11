pub mod commands;
pub mod config;
pub mod executor;
pub mod utils;
pub mod validator;

pub use commands::CommandSpec;
pub use config::Config;
pub use executor::Executor;
pub use utils::FileInfo;
pub use validator::Validator;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodeRunnerError {
    #[error("File not found: {0}")]
    FileNotFound(String),
    
    #[error("Invalid file path: {0}")]
    InvalidPath(String),
    
    #[error("Unsupported file type: .{0}")]
    UnsupportedFileType(String),
    
    #[error("File is a directory: {0}")]
    IsDirectory(String),
    
    #[error("File is too large: {0} MB (max: {1} MB)")]
    FileTooLarge(u64, u64),
    
    #[error("Program not installed: {0}")]
    ProgramNotInstalled(String),
    
    #[error("Execution failed: {0}")]
    ExecutionFailed(String),
    
    #[error("Timeout: Command exceeded {0} seconds")]
    Timeout(u64),
    
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, CodeRunnerError>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_file_info_extraction() {
        let info = utils::get_file_info("test.py");
        assert_eq!(info.ext, "py");
    }

    #[test]
    fn test_uppercase_extension() {
        let info = utils::get_file_info("TEST.PY");
        assert_eq!(info.ext, "py");
    }

    #[test]
    fn test_multiple_dots() {
        let info = utils::get_file_info("file.test.py");
        assert_eq!(info.ext, "py");
    }

    #[test]
    fn test_no_extension() {
        let info = utils::get_file_info("noext");
        assert_eq!(info.ext, "");
    }

    #[test]
    fn test_validator_empty_path() {
        let validator = Validator::new(100);
        assert!(validator.validate("").is_err());
        assert!(validator.validate("   ").is_err());
    }

    #[test]
    fn test_validator_directory_paths() {
        let validator = Validator::new(100);
        assert!(validator.validate(".").is_err());
        assert!(validator.validate("..").is_err());
    }

    #[test]
    fn test_validator_flag_like_paths() {
        let validator = Validator::new(100);
        assert!(validator.validate("-help").is_err());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.timeout, 30);
        assert_eq!(config.max_file_size_mb, 100);
        assert!(config.cleanup_artifacts);
    }
}