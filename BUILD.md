# Building Astrolog-rs

This document provides detailed instructions for building and running the Astrolog-rs project.

## Prerequisites

- Rust 1.87.0 or later
- Cargo (Rust's package manager)
- Git

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/astrolog-rs.git
   cd astrolog-rs
   ```

2. Install Rust (if not already installed):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

3. Update Rust to the required version:
   ```bash
   rustup update
   rustup default stable
   ```

## Building

### Development Build

1. Build the project:
   ```bash
   cargo build
   ```

2. Run tests:
   ```bash
   cargo test
   ```

3. Run with logging:
   ```bash
   RUST_LOG=debug cargo run
   ```

### Release Build

For optimal performance, use a release build:

```bash
cargo build --release
```

The release binary will be located at `target/release/astrolog-rs`.

## Running the API Server

1. Start the server:
   ```bash
   cargo run
   ```

2. The server will start on `http://localhost:3000`

3. Test the health endpoint:
   ```bash
   curl http://localhost:3000/health
   ```

## Development Tools

### Code Formatting

Format your code using rustfmt:
```bash
cargo fmt
```

### Linting

Check your code with clippy:
```bash
cargo clippy
```

### Documentation

Generate documentation:
```bash
cargo doc --open
```

## Common Issues

### Compilation Errors

1. **Module Resolution Errors**
   - Ensure all module files exist in the correct locations
   - Check module declarations in `mod.rs` files
   - Verify import paths are correct

2. **Trait Bounds Errors**
   - Check that all required traits are implemented
   - Verify generic type constraints
   - Ensure proper use of `#[derive]` attributes

3. **Dependency Version Conflicts**
   - Check `Cargo.toml` for compatible versions
   - Run `cargo update` to update dependencies
   - Clear cargo cache if needed: `cargo clean`

### Runtime Issues

1. **Port Already in Use**
   - Check if another instance is running
   - Use a different port by setting the `PORT` environment variable

2. **Permission Issues**
   - Ensure you have write permissions in the project directory
   - Check system firewall settings

## Environment Variables

- `RUST_LOG`: Set logging level (debug, info, warn, error)
- `PORT`: Set custom port number (default: 3000)
- `HOST`: Set custom host address (default: 127.0.0.1)

## IDE Setup

### VS Code

1. Install the "rust-analyzer" extension
2. Install the "CodeLLDB" extension for debugging
3. Recommended settings:
   ```json
   {
     "rust-analyzer.checkOnSave.command": "clippy",
     "editor.formatOnSave": true
   }
   ```

### IntelliJ IDEA

1. Install the "Rust" plugin
2. Enable "Run on Save" for rustfmt
3. Configure the Rust toolchain in Settings

## Contributing

When contributing to the project:

1. Create a new branch for your changes
2. Run tests before submitting:
   ```bash
   cargo test
   cargo clippy
   cargo fmt
   ```
3. Update documentation if needed
4. Submit a pull request

## Troubleshooting

If you encounter issues:

1. Check the [GitHub Issues](https://github.com/yourusername/astrolog-rs/issues)
2. Search the [Rust Forums](https://users.rust-lang.org/)
3. Join our [Discord Server](https://discord.gg/your-server) for help 