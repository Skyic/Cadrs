# Cadrs CAD 

A comprehensive Rust-based computer-aided design (CAD) library providing powerful 2D geometric modeling capabilities.

## Overview

Cadrs CAD is a pure Rust implementation of a CAD kernel that offers extensive functionality for geometric modeling, rendering, data management, and file format support. The SDK is designed with a modular architecture, allowing users to enable only the features they need.

## Features

### Geometry Module
- Primitive geometries: Points, Lines, Arcs, Circles, Ellipses
- Advanced curves: B-splines, NURBS, Polylines
- Geometric operations: Boolean operations, Intersections, Geometric analysis
- Spatial indexing for efficient queries

### Data Structures
- Document management with layers and blocks
- Entity system with unique IDs
- Event system for plugin architecture
- Selection management

### Rendering
- Software renderer
- GPU-accelerated rendering via wgpu
- SVG export support
- Viewport management

### I/O Support
- DWG format (via lopdf)
- DXF format
- IGES import/export
- STEP import/export
- SVG import/export

### Additional Features
- Command system with history management
- Geometric constraint solving
- Dimensioning and annotations
- Grid and snap point functionality
- Performance optimizations with parallel processing

## Getting Started

### Prerequisites

- Rust 1.70 or later
- Cargo package manager

### Installation

Add Cadrs to your `Cargo.toml`:

```toml
[dependencies]
cadrs = { path = "path/to/cadrs", features = ["geometry", "math", "data_structure", "io", "render"] }
```

### Quick Example

```rust
use cadrs::prelude::*;

fn main() -> Result<(), CadrsError> {
    // Create a new document
    let mut doc = Document::new();
    
    // Create geometric entities
    let point = Point::new(0.0, 0.0);
    let line = Line::new(point, Point::new(10.0, 10.0));
    
    // Add entities to document
    doc.add_entity(&line)?;
    
    Ok(())
}
```

## Module Organization

The SDK is organized into the following modules:

| Module | Description | Feature Flag |
|--------|-------------|--------------|
| `api` | High-level CAD API | `api` |
| `command` | Command execution system | default |
| `constraint` | Geometric constraint solving | `constraint` |
| `data_structure` | Core document and entity management | default |
| `dimension` | Dimension annotations | default |
| `edit` | Entity editing and transformations | default |
| `geometric_tolerance` | GD&T symbols | default |
| `geometry` | Geometric primitives and operations | default |
| `grid` | Grid system | default |
| `hatch` | Hatching patterns | default |
| `history` | Command history management | default |
| `io` | File format support (DXF, DWG, IGES, STEP, SVG) | default |
| `layer` | Layer management | default |
| `math` | Mathematical utilities (vectors, matrices, transformations) | default |
| `performance` | Performance optimization utilities | `performance` |
| `render` | Rendering engine (software, GPU, SVG) | default |
| `selection` | Entity selection system | default |
| `snap` | Snap point functionality | default |
| `spatial` | Spatial indexing | default |
| `text` | Text and mtext annotations | default |

## Feature Flags

Configure the SDK with the following features:

```toml
[dependencies.cadrs]
path = "path/to/sdk"
default-features = false
features = [
    "geometry",
    "math",
    "data_structure",
    "io",
    "render",
    "api",
    "boolean",
    "constraint",
    "gpu"
]
```

| Feature | Description | Default |
|---------|-------------|---------|
| `geometry` | Enable geometry primitives and operations | Yes |
| `math` | Enable mathematical utilities | Yes |
| `data_structure` | Enable document and entity management | Yes |
| `io` | Enable file I/O (requires quick-xml) | Yes |
| `render` | Enable rendering engine (requires wgpu) | Yes |
| `api` | Enable high-level API | No |
| `performance` | Enable performance utilities | No |
| `boolean` | Enable boolean operations | No |
| `constraint` | Enable geometric constraints | No |
| `gpu` | Enable GPU rendering features | No |

## Platform Support

- **Linux**: Full support including GPU rendering
- **macOS**: Full support including Metal rendering
- **Windows**: Full support including DirectX rendering
- **WASM**: Experimental support for web applications

## Building Documentation

Build API documentation with:

```bash
cargo doc --all-features --open
```

Or use the workspace configuration:

```bash
cargo doc -p cadrs --all-features --open
```

## Testing

Run the test suite with:

```bash
cargo test --all-features
```

## Performance

Cadrs CAD SDK includes several performance optimizations:

- **Parallel Processing**: Utilizes rayon for parallel computations
- **Spatial Indexing**: Uses rstar for efficient spatial queries
- **Memory Management**: Optimized memory allocation with parking_lot
- **Hash Maps**: Fast hashing with ahash

Enable performance features for benchmarks:

```bash
cargo bench --features performance
```

## Contributing

Contributions are welcome! Please read our contributing guidelines before submitting pull requests.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Thanks to all developers who contributed to this project
- Built with Rust - a language empowering everyone to build reliable and efficient software
