use code_runner::{
    commands, utils, Config, Executor, Result, Validator,
};
use std::env;
use std::process::exit;

fn main() {
    env_logger::init();
    
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage(&args);
        exit(1);
    }
    
    let file = args[1..].join(" ");
    
    match run(&file) {
        Ok(_) => exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            exit(1);
        }
    }
}

fn run(file: &str) -> Result<()> {
    let config = Config::load();
    
    let validator = Validator::new(config.max_file_size_mb);
    validator.validate(file)?;
    
    let file_info = utils::get_file_info(file);
    validator.validate_file(&file_info)?;
    
    let cmd_spec = commands::get_command(&file_info.ext, &file_info.abs_path)
        .ok_or_else(|| code_runner::CodeRunnerError::UnsupportedFileType(file_info.ext.clone()))?;
    
    if config.check_installed {
        utils::check_program_installed(&cmd_spec.program)?;
    }
    
    let executor = Executor::new(config.clone());
    executor.execute(&cmd_spec)?;
    executor.cleanup(&file_info.ext)?;
    
    Ok(())
}

fn print_usage(args: &[String]) {
    let program = args.get(0).map(|s| s.as_str()).unwrap_or("code-runner");
    eprintln!("Usage: {} <file>", program);
    eprintln!("\nSupported Languages: 35+");
    eprintln!("  JavaScript, TypeScript, Python, Go, Rust");
    eprintln!("  C, C++, Java, Kotlin, Scala");
    eprintln!("  Ruby, PHP, Lua, Perl");
    eprintln!("  And many more...");
    eprintln!("\nExamples:");
    eprintln!("  {} script.py", program);
    eprintln!("  {} \"file with spaces.js\"", program);
}