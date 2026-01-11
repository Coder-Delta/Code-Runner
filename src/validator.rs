use crate::{CodeRunnerError, Result, FileInfo};
use std::path::Path;
use std::fs;

pub struct Validator {
    max_file_size_mb: u64,
}

impl Validator {
    pub fn new(max_file_size_mb: u64) -> Self {
        Self { max_file_size_mb }
    }
    
    pub fn validate(&self, file_path: &str) -> Result<()> {
        let trimmed = file_path.trim();
        
        if trimmed.is_empty() {
            return Err(CodeRunnerError::InvalidPath("File path cannot be empty".to_string()));
        }
        
        if trimmed == "." || trimmed == ".." {
            return Err(CodeRunnerError::InvalidPath("Cannot run directories".to_string()));
        }
        
        if trimmed.starts_with('-') {
            return Err(CodeRunnerError::InvalidPath(
                format!("Path '{}' looks like a command flag", trimmed)
            ));
        }
        
        if trimmed.contains('\0') {
            return Err(CodeRunnerError::InvalidPath("Path contains null bytes".to_string()));
        }
        
        if trimmed.len() > 4096 {
            return Err(CodeRunnerError::InvalidPath("Path is too long (max 4096 characters)".to_string()));
        }
        
        self.validate_windows_reserved(trimmed)?;
        
        Ok(())
    }
    
    pub fn validate_file(&self, file_info: &FileInfo) -> Result<()> {
        if !file_info.abs_path.exists() {
            return Err(CodeRunnerError::FileNotFound(
                file_info.abs_path.display().to_string()
            ));
        }
        
        if file_info.abs_path.is_dir() {
            return Err(CodeRunnerError::IsDirectory(
                file_info.abs_path.display().to_string()
            ));
        }
        
        if file_info.ext.is_empty() {
            return Err(CodeRunnerError::UnsupportedFileType(String::new()));
        }
        
        let metadata = fs::metadata(&file_info.abs_path)?;
        let file_size_mb = metadata.len() / (1024 * 1024);
        
        if file_size_mb > self.max_file_size_mb {
            return Err(CodeRunnerError::FileTooLarge(file_size_mb, self.max_file_size_mb));
        }
        
        Ok(())
    }
    
    fn validate_windows_reserved(&self, path: &str) -> Result<()> {
        if cfg!(target_os = "windows") {
            let filename = Path::new(path)
                .file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_lowercase();
            
            let reserved = [
                "con", "prn", "aux", "nul", 
                "com1", "com2", "com3", "com4", "com5", "com6", "com7", "com8", "com9",
                "lpt1", "lpt2", "lpt3", "lpt4", "lpt5", "lpt6", "lpt7", "lpt8", "lpt9"
            ];
            
            for name in &reserved {
                if filename == *name || filename.starts_with(&format!("{}.", name)) {
                    return Err(CodeRunnerError::InvalidPath(
                        format!("'{}' is a reserved filename on Windows", name)
                    ));
                }
            }
        }
        Ok(())
    }
}