use std::path::{Path, PathBuf};
use std::fs;

pub struct FileInfo {
    pub ext: String,
    pub abs_path: PathBuf,
}

pub fn get_file_info(file: &str) -> FileInfo {
    let path = Path::new(file);
    
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();
    
    let abs_path = match fs::canonicalize(path) {
        Ok(canonical) => normalize_path(canonical),
        Err(_) => PathBuf::from(file),
    };
    
    FileInfo { ext, abs_path }
}

#[cfg(target_os = "windows")]
fn normalize_path(path: PathBuf) -> PathBuf {
    let path_str = path.display().to_string();
    if path_str.starts_with(r"\\?\") {
        PathBuf::from(&path_str[4..])
    } else {
        path
    }
}

#[cfg(not(target_os = "windows"))]
fn normalize_path(path: PathBuf) -> PathBuf {
    path
}

pub fn check_program_installed(program: &str) -> crate::Result<()> {
    if program == "cmd" || program == "sh" {
        return Ok(());
    }
    
    if program.is_empty() {
        return Err(crate::CodeRunnerError::InvalidPath(
            "Program name cannot be empty".to_string()
        ));
    }
    
    let check_result = if cfg!(target_os = "windows") {
        std::process::Command::new("where")
            .arg(program)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()
    } else {
        std::process::Command::new("which")
            .arg(program)
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .output()
    };
    
    match check_result {
        Ok(output) if output.status.success() => Ok(()),
        _ => Err(crate::CodeRunnerError::ProgramNotInstalled(program.to_string())),
    }
}