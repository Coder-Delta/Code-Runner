use crate::{CodeRunnerError, CommandSpec, Config, Result};
use std::io::{self, Write};
use std::process::{Command, Stdio};
use std::time::Duration;
use wait_timeout::ChildExt;
use std::fs;
use std::path::Path;

pub struct Executor {
    config: Config,
}

impl Executor {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    
    pub fn execute(&self, cmd_spec: &CommandSpec) -> Result<ExecutionResult> {
        if cmd_spec.program.is_empty() {
            return Err(CodeRunnerError::ExecutionFailed(
                "Program name is empty".to_string()
            ));
        }
        
        if !self.config.silent_mode {
            println!("Running: {}\n", cmd_spec.display_string());
            io::stdout().flush()?;
        }
        
        let mut child = Command::new(&cmd_spec.program)
            .args(&cmd_spec.args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| CodeRunnerError::ExecutionFailed(
                format!("Failed to start '{}': {}", cmd_spec.program, e)
            ))?;
        
        let timeout = Duration::from_secs(self.config.timeout);
        
        match child.wait_timeout(timeout)? {
            Some(status) => {
                let output = child.wait_with_output()?;
                
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = status.code().unwrap_or(-1);
                
                if !self.config.silent_mode {
                    if !stderr.trim().is_empty() {
                        eprintln!("stderr:\n{}", stderr);
                    }
                    if !stdout.is_empty() {
                        print!("{}", stdout);
                        io::stdout().flush()?;
                    }
                }
                
                if !status.success() {
                    return Err(CodeRunnerError::ExecutionFailed(
                        format!("Command exited with code: {}", exit_code)
                    ));
                }
                
                Ok(ExecutionResult {
                    stdout,
                    stderr,
                    exit_code,
                    success: true,
                })
            }
            None => {
                child.kill()?;
                child.wait()?;
                Err(CodeRunnerError::Timeout(self.config.timeout))
            }
        }
    }
    
    pub fn cleanup(&self, extension: &str) -> Result<()> {
        if !self.config.cleanup_artifacts {
            return Ok(());
        }
        
        match extension {
            "rs" | "c" | "cpp" | "cc" | "cxx" => {
                let main_exe = if cfg!(target_os = "windows") {
                    "main.exe"
                } else {
                    "main"
                };
                
                if Path::new(main_exe).exists() {
                    fs::remove_file(main_exe).ok();
                }
            }
            "nim" => {
                if Path::new("main").exists() {
                    fs::remove_file("main").ok();
                }
            }
            _ => {}
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionResult {
    pub stdout: String,
    pub stderr: String,
    pub exit_code: i32,
    pub success: bool,
}