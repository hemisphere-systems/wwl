<h1 align="center">`wwl`</h1>
<div align="center">
  <strong>
    Rust bindings for Wasserstein Weisfeiler-Lehman Graph Kernels.
    Compute distances and similarities between graphs using optimal transport.
  </strong>
</div>
<br />

## Examples

### Computing Distance Between Two Graphs

```rust
use petgraph::Graph;
use wwl::{WWLKernel, DistanceConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph1 = Graph::new_undirected();
    let n1 = graph1.add_node(Some(1));
    let n2 = graph1.add_node(Some(2));
    graph1.add_edge(n1, n2, ());

    let mut graph2 = Graph::new_undirected();
    let n3 = graph2.add_node(Some(1));
    let n4 = graph2.add_node(Some(2));
    let n5 = graph2.add_node(Some(3));
    graph2.add_edge(n3, n4, ());
    graph2.add_edge(n4, n5, ());

    let graphs = vec![graph1, graph2];
    let kernel = WWLKernel::new()?;
    let distances = kernel.compute_distance_categorical(&graphs, &DistanceConfig::default())?;

    let distance: f64 = distances[[0, 1]];
    println!("Distance: {}", distance);

    Ok(())
}
```

### Graph Similarity

```rust
use wwl::KernelConfig;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graphs = create_graphs();
    let kernel = WWLKernel::new()?;
    
    let kernel_matrix = kernel.compute_kernel_categorical(&graphs, &KernelConfig::default())?;
    let similarity: f64 = kernel_matrix[[0, 1]];
    
    println!("Similarity: {}", similarity);
    Ok(())
}
```

### Continuous Node Features

For graphs with real-valued node attributes instead of discrete labels.

```rust
use ndarray::Array2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graphs = create_unlabeled_graphs();
    let kernel = WWLKernel::new()?;
    
    // Feature matrix: rows = graphs, cols = max nodes across all graphs
    let mut features = Array2::zeros((2, 3));
    features[[0, 0]] = 1.5; // Graph 0, Node 0
    features[[0, 1]] = 2.0; // Graph 0, Node 1
    features[[1, 0]] = 1.0; // Graph 1, Node 0
    features[[1, 1]] = 2.5; // Graph 1, Node 1
    features[[1, 2]] = 3.0; // Graph 1, Node 2
    
    let kernel_matrix = kernel.compute_kernel_continuous(&graphs, &features, &KernelConfig::default())?;
    
    Ok(())
}
```

The WWL algorithm automatically handles graphs of different sizes by using optimal transport to align their node representations. This makes it particularly useful for comparing molecular structures, social networks, or any graph data where size varies.

Add to your `Cargo.toml`:

```toml
[dependencies]
wwl = "0.1.0"
petgraph = "0.8"
ndarray = "0.15"
```

Requires Python WWL library to be installed and accessible.
