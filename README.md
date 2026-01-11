cat > README.md << 'EOF'
# Code Runner

Universal code runner for 35+ programming languages. Production-ready with comprehensive testing.

## Features

- Support for 35+ languages
- Cross-platform (Windows, macOS, Linux)
- Handles all edge cases
- Configurable timeouts
- Automatic cleanup
- Full test coverage

## Installation
```bash
cargo install --path .
```

## Usage
```bash
code-runner script.py
code-runner "file with spaces.js"
code-runner file with spaces.py
```

## Supported Languages

JavaScript, TypeScript, Python, Go, Rust, C, C++, Java, Kotlin, Scala, Ruby, PHP, Lua, Perl, Swift, Dart, Haskell, Julia, Elixir, and 15+ more.

## Configuration

Create `~/.config/code-runner/config.toml`:
```toml
timeout = 30
max_file_size_mb = 100
cleanup_artifacts = true
silent_mode = false
check_installed = true
```

## Development
```bash
cargo build
cargo test
cargo run -- test.py
```

## License

MIT
EOF