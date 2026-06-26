# tungsten-mc-compile

A Minecraft worldgen compiler that parses Minecraft data packs and compiles them to optimized Rust and CUDA code.

## Overview

This crate provides tools to:

- Parse Minecraft worldgen configuration files (density functions, noise settings, biomes, etc.)
- Transform them into an intermediate SPMT (Sparse Matrix Program Tree) representation
- Compile the representation to optimized Rust or CUDA code using the [tungsten-wg](https://github.com/NiekAukes/tungsten-wg) backend

## Features

- **Data Pack Parsing**: Load vanilla and modded Minecraft worldgen configurations
- **Code Generation**: Generate optimized Rust or CUDA implementations
- **Chunk-based Processing**: Configurable chunk size for generation
- **CLI Tool**: Command-line interface for easy compilation
- **Library API**: Use as a library in your own projects

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tungsten-mc-compile = "0.1.0"
```

## Usage

### As a Library

```rust
use tungsten_mc_compile::{MinecraftCompilerConfig, run_generation};

let config = MinecraftCompilerConfig::new()
    .with_chunk_size(16);

let output = run_generation(&config);

// Access the generated code
println!("Rust code: {}", output.rust_code);
```

### As a CLI Tool

```bash
# Install the CLI
cargo install tungsten-mc-compile

# Compile vanilla worldgen
tungsten-mc-compile --output density/

# Compile with a mod/datapack
tungsten-mc-compile --mod-folder path/to/datapack --output density/

# Generate CUDA code
tungsten-mc-compile --cuda --output density/
```

## CLI Options

- `-m, --mod-folder <PATH>`: Path to a mod or data pack folder
- `-o, --output <PATH>`: Output directory (default: `density`)
- `--cuda`: Generate CUDA code instead of Rust
- `--chunk-size <SIZE>`: Chunk size for generation (default: 16)
- `--emit-intermediates`: Output intermediate representations
- `-v, --verbose`: Enable verbose logging

## Configuration

The `MinecraftCompilerConfig` struct provides a builder-style API:

```rust
let config = MinecraftCompilerConfig::new()
    .with_mod_path("path/to/datapack")
    .with_chunk_size(16)
    .with_backend_config(CompilerConfig::default().with_rcl(true));
```

## How It Works

1. **Loading**: Reads vanilla worldgen configs (bundled) and optional mod/datapack configs
2. **Parsing**: Parses JSON files into internal data structures
3. **Transformation**: Converts Minecraft's density function graph into SPMT representation
4. **Compilation**: Uses tungsten-wg to generate optimized target code

## Requirements

- Rust 2021 edition or later
- Vanilla worldgen data files are bundled with the crate

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Related Projects

- [tungsten-wg](https://github.com/NiekAukes/tungsten-wg): The backend compiler library
- [tungsten-frontends](https://github.com/NiekAukes/tungsten-frontends): Collection of frontend parsers
