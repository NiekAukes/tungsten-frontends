# tungsten-mc

A high-performance runtime library for Minecraft worldgen, providing optimized implementations of density functions, noise generators, and world generation utilities.

## Overview

This crate is the runtime component of the Tungsten Minecraft worldgen compiler pipeline. It contains:

- **Generated Worldgen Code**: Optimized Rust implementations of Minecraft density functions compiled by [tungsten-mc-compile](https://crates.io/crates/tungsten-mc-compile)
- **Noise Generators**: High-performance Perlin noise and other noise implementations
- **Math Utilities**: Vector math, interpolation, and other computational primitives
- **Random Number Generation**: Xoroshiro128++ and other RNGs for deterministic world generation

The generated worldgen code is compiled at build time from Minecraft data packs, producing efficient density function evaluators for world generation.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
tungsten-mc = "0.1.0"
```

## Usage

### Basic World Generation

```rust
use tungsten_mc::{Vec3, sample_density_at, orchestration_seeded};

// Sample density at a specific position
let seed = 12345;
let position = Vec3 { x: 0.0, y: 64.0, z: 0.0 };
let density = sample_density_at(seed, position);

// Run full orchestration to get all density outputs
let outputs = orchestration_seeded(seed, position);
println!("Final density: {}", outputs.final_density[0]);
```

## Build Process

This crate uses [tungsten-mc-compile](https://crates.io/crates/tungsten-mc-compile) as a build dependency to generate optimized worldgen code at compile time. The build script:

1. Parses Minecraft worldgen configuration files
2. Compiles them to optimized Rust code
3. Includes the generated code in the library

## Features

- **Zero-cost abstractions**: Generated code is optimized for performance
- **Deterministic**: Same seed always produces the same world
- **Compatible**: Works with vanilla Minecraft worldgen configurations
- **Extensible**: Can be used with modded data packs

## Performance

The generated code is optimized for performance:

- Inlined density function evaluations
- Minimal branching in hot paths
- Efficient noise sampling

## License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## Related Crates

- [tungsten-mc-compile](https://crates.io/crates/tungsten-mc-compile) - Compiler for Minecraft worldgen data packs
- [tungsten-wg](https://crates.io/crates/tungsten-wg) - Code generation backend
