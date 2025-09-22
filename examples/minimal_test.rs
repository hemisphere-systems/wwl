use petgraph::Graph;
use wwl_rust::{WWLKernel, KernelConfig};
use pyo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test if we can call Python WWL directly from Rust
    Python::attach(|py| -> PyResult<()> {
        // Import igraph and wwl
        let igraph = py.import("igraph")?;
        let wwl = py.import("wwl")?;
        
        // Create graphs manually in Python to verify they work
        let graph1 = igraph.getattr("Graph")?.call((vec![(0usize, 1usize)], 2usize), None)?;
        graph1.getattr("vs")?.setattr("label", vec![1i32, 2i32])?;
        
        let graph2 = igraph.getattr("Graph")?.call((vec![(0usize, 1usize), (1usize, 2usize)], 3usize), None)?;
        graph2.getattr("vs")?.setattr("label", vec![1i32, 2i32, 3i32])?;
        
        let graphs = pyo3::types::PyList::new(py, vec![graph1, graph2])?;
        
        println!("Created graphs in Python successfully");
        
        // Try WWL computation
        let result = wwl.getattr("wwl")?.call((graphs,), None)?;
        println!("✅ Direct Python call works!");
        println!("Result shape: {:?}", result.getattr("shape")?);
        
        Ok(())
    })?;
    
    // Now try our Rust wrapper
    println!("\nTesting Rust wrapper:");
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
    
    match kernel.compute_kernel_categorical(&graphs, &KernelConfig::default()) {
        Ok(result) => {
            println!("✅ Rust wrapper works: {:?}", result.dim());
        }
        Err(e) => {
            println!("❌ Rust wrapper failed: {}", e);
        }
    }
    
    Ok(())
}