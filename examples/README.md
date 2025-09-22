# Examples

Examples demonstrating WWL Rust bindings usage.

## Files

- `graph_kernel.rs` - Kernel/distance computation with single value extraction
- `variable_size_graphs.rs` - Graphs of different sizes (validates main fix)
- `categorical_propagation.rs` - Discrete label-based propagation
- `petgraph_validation.rs` - Graph structure validation
- `continuous_propagation.rs` - Real-valued node feature propagation

## Usage

```bash
nix develop
cargo run --example graph_kernel
```

## Extracting Single Distance Values

```rust
let distance_matrix = kernel.compute_distance_categorical(&graphs, &config)?;
let distance: f64 = distance_matrix[[0, 1]]; // Distance between graphs 0 and 1
```

## Propagation Types

- **Categorical**: `compute_kernel_categorical()` - uses node labels
- **Continuous**: `compute_kernel_continuous()` - uses node features array
