use std::path::Path;

#[derive(Clone, Debug)]
pub struct CommandSpec {
    pub program: String,
    pub args: Vec<String>,
}

impl CommandSpec {
    pub fn new(program: String, args: Vec<String>) -> Self {
        Self { program, args }
    }
    
    pub fn display_string(&self) -> String {
        let mut parts = vec![self.program.clone()];
        parts.extend(self.args.iter().map(|arg| {
            if arg.contains(' ') && !arg.starts_with('"') {
                format!("\"{}\"", arg)
            } else {
                arg.clone()
            }
        }));
        parts.join(" ")
    }
}

fn is_win() -> bool {
    cfg!(target_os = "windows")
}

fn bin(name: &str) -> String {
    if is_win() {
        format!("{}.exe", name)
    } else {
        format!("./{}", name)
    }
}

fn escape_for_shell(path: &str) -> String {
    if is_win() {
        if path.contains(' ') {
            format!("\"{}\"", path)
        } else {
            path.to_string()
        }
    } else {
        format!("'{}'", path.replace('\'', "'\\''"))
    }
}

pub fn get_command(ext: &str, file_path: &Path) -> Option<CommandSpec> {
    let path_str = file_path.display().to_string();
    
    match ext {
        "js" => Some(CommandSpec::new("node".to_string(), vec![path_str])),
        "ts" => Some(CommandSpec::new("npx".to_string(), vec!["tsx".to_string(), path_str])),
        "py" => Some(CommandSpec::new("python".to_string(), vec![path_str])),
        "go" => Some(CommandSpec::new("go".to_string(), vec!["run".to_string(), path_str])),
        
        "rs" => Some(CommandSpec::new(
            if is_win() { "cmd".to_string() } else { "sh".to_string() },
            if is_win() {
                vec!["/C".to_string(), format!("rustc {} -o main && {}", escape_for_shell(&path_str), bin("main"))]
            } else {
                vec!["-c".to_string(), format!("rustc {} -o main && {}", escape_for_shell(&path_str), bin("main"))]
            }
        )),
        
        "c" => Some(CommandSpec::new(
            if is_win() { "cmd".to_string() } else { "sh".to_string() },
            if is_win() {
                vec!["/C".to_string(), format!("gcc {} -o main && {}", escape_for_shell(&path_str), bin("main"))]
            } else {
                vec!["-c".to_string(), format!("gcc {} -o main && {}", escape_for_shell(&path_str), bin("main"))]
            }
        )),
        
        "cpp" | "cc" | "cxx" => Some(CommandSpec::new(
            if is_win() { "cmd".to_string() } else { "sh".to_string() },
            if is_win() {
                vec!["/C".to_string(), format!("g++ {} -o main && {}", escape_for_shell(&path_str), bin("main"))]
            } else {
                vec!["-c".to_string(), format!("g++ {} -o main && {}", escape_for_shell(&path_str), bin("main"))]
            }
        )),
        
        "java" => {
            let cls = file_path.file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("Main")
                .to_string();
            
            Some(CommandSpec::new(
                if is_win() { "cmd".to_string() } else { "sh".to_string() },
                if is_win() {
                    vec!["/C".to_string(), format!("javac {} && java {}", escape_for_shell(&path_str), cls)]
                } else {
                    vec!["-c".to_string(), format!("javac {} && java {}", escape_for_shell(&path_str), cls)]
                }
            ))
        },
        
        "php" => Some(CommandSpec::new("php".to_string(), vec![path_str])),
        "rb" => Some(CommandSpec::new("ruby".to_string(), vec![path_str])),
        "sh" => Some(CommandSpec::new("bash".to_string(), vec![path_str])),
        "ps1" => Some(CommandSpec::new(
            "powershell".to_string(),
            vec!["-ExecutionPolicy".to_string(), "Bypass".to_string(), "-File".to_string(), path_str]
        )),
        "lua" => Some(CommandSpec::new("lua".to_string(), vec![path_str])),
        "pl" => Some(CommandSpec::new("perl".to_string(), vec![path_str])),
        "r" => Some(CommandSpec::new("Rscript".to_string(), vec![path_str])),
        "swift" => Some(CommandSpec::new("swift".to_string(), vec![path_str])),
        "dart" => Some(CommandSpec::new("dart".to_string(), vec!["run".to_string(), path_str])),
        "zig" => Some(CommandSpec::new("zig".to_string(), vec!["run".to_string(), path_str])),
        "hs" => Some(CommandSpec::new("runhaskell".to_string(), vec![path_str])),
        "jl" => Some(CommandSpec::new("julia".to_string(), vec![path_str])),
        "ex" | "exs" => Some(CommandSpec::new("elixir".to_string(), vec![path_str])),
        "cr" => Some(CommandSpec::new("crystal".to_string(), vec!["run".to_string(), path_str])),
        "scala" => Some(CommandSpec::new("scala".to_string(), vec![path_str])),
        "groovy" => Some(CommandSpec::new("groovy".to_string(), vec![path_str])),
        "clj" => Some(CommandSpec::new("clojure".to_string(), vec![path_str])),
        "rkt" => Some(CommandSpec::new("racket".to_string(), vec![path_str])),
        "ml" => Some(CommandSpec::new("ocaml".to_string(), vec![path_str])),
        "erl" => Some(CommandSpec::new("escript".to_string(), vec![path_str])),
        "tcl" => Some(CommandSpec::new("tclsh".to_string(), vec![path_str])),
        
        "kt" | "kts" => Some(CommandSpec::new(
            if is_win() { "cmd".to_string() } else { "sh".to_string() },
            if is_win() {
                vec!["/C".to_string(), format!("kotlinc {} -include-runtime -d main.jar && java -jar main.jar", escape_for_shell(&path_str))]
            } else {
                vec!["-c".to_string(), format!("kotlinc {} -include-runtime -d main.jar && java -jar main.jar", escape_for_shell(&path_str))]
            }
        )),
        
        "nim" => Some(CommandSpec::new(
            if is_win() { "cmd".to_string() } else { "sh".to_string() },
            if is_win() {
                vec!["/C".to_string(), format!("nim c -r {}", escape_for_shell(&path_str))]
            } else {
                vec!["-c".to_string(), format!("nim c -r {}", escape_for_shell(&path_str))]
            }
        )),
        
        "d" => Some(CommandSpec::new(
            if is_win() { "cmd".to_string() } else { "sh".to_string() },
            if is_win() {
                vec!["/C".to_string(), format!("dmd -run {}", escape_for_shell(&path_str))]
            } else {
                vec!["-c".to_string(), format!("dmd -run {}", escape_for_shell(&path_str))]
            }
        )),
        
        "fs" | "fsx" => Some(CommandSpec::new(
            "dotnet".to_string(),
            vec!["fsi".to_string(), path_str]
        )),
        
        "cs" => Some(CommandSpec::new(
            "dotnet".to_string(),
            vec!["script".to_string(), path_str]
        )),
        
        _ => None,
    }
}