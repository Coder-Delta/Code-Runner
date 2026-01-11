#[cfg(test)]
mod tests {
    use code_runner::{utils, Validator, Config};
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
        assert!(validator.validate("--version").is_err());
    }

    #[test]
    fn test_validator_null_bytes() {
        let validator = Validator::new(100);
        assert!(validator.validate("file\0.py").is_err());
    }

    #[test]
    fn test_validator_long_path() {
        let validator = Validator::new(100);
        let long_path = "a".repeat(5000);
        assert!(validator.validate(&long_path).is_err());
    }

    #[test]
    fn test_config_default() {
        let config = Config::default();
        assert_eq!(config.timeout, 30);
        assert_eq!(config.max_file_size_mb, 100);
        assert!(config.cleanup_artifacts);
        assert!(!config.silent_mode);
    }

    #[test]
    fn test_config_save_load() {
        let mut config = Config::default();
        config.timeout = 60;
        
        config.save().ok();
        let loaded = Config::load();
        
        assert_eq!(loaded.timeout, 60);
    }

    #[test]
    fn test_file_with_spaces() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "print('hello')").unwrap();
        
        let path = temp_file.path();
        let path_str = path.to_str().unwrap();
        
        let validator = Validator::new(100);
        assert!(validator.validate(path_str).is_ok());
    }

    #[test]
    fn test_validator_file_exists() {
        let mut temp_file = NamedTempFile::new().unwrap();
        writeln!(temp_file, "print('test')").unwrap();
        temp_file.flush().unwrap();
        
        let path_str = temp_file.path().to_str().unwrap();
        let file_info = utils::get_file_info(path_str);
        
        let validator = Validator::new(100);
        assert!(validator.validate_file(&file_info).is_ok());
    }

    #[test]
    fn test_validator_nonexistent_file() {
        let file_info = utils::get_file_info("nonexistent_file_12345.py");
        let validator = Validator::new(100);
        assert!(validator.validate_file(&file_info).is_err());
    }

    #[test]
    #[cfg(target_os = "windows")]
    fn test_validator_windows_reserved() {
        let validator = Validator::new(100);
        assert!(validator.validate("CON").is_err());
        assert!(validator.validate("PRN.txt").is_err());
        assert!(validator.validate("AUX.py").is_err());
    }
}