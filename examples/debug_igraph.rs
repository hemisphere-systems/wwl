use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Python::attach(|py| -> PyResult<()> {
        let igraph = py.import("igraph")?;
        let wwl = py.import("wwl")?;
        
        // Create our Rust-style graphs
        let edges1 = vec![(0usize, 1usize)];
        let kwargs1 = PyDict::new(py);
        kwargs1.set_item("directed", false)?;
        let graph1 = igraph.getattr("Graph")?.call((edges1, 2usize), Some(&kwargs1))?;
        graph1.getattr("vs")?.setattr("label", vec![1i32, 2i32])?;
        
        let edges2 = vec![(0usize, 1usize), (1usize, 2usize)];
        let kwargs2 = PyDict::new(py);
        kwargs2.set_item("directed", false)?;
        let graph2 = igraph.getattr("Graph")?.call((edges2, 3usize), Some(&kwargs2))?;
        graph2.getattr("vs")?.setattr("label", vec![1i32, 2i32, 3i32])?;
        
        // Debug the graphs
        println!("Graph1 info:");
        println!("  vcount: {}", graph1.getattr("vcount")?.call0()?);
        println!("  ecount: {}", graph1.getattr("ecount")?.call0()?);
        println!("  labels: {:?}", graph1.getattr("vs")?.getattr("label")?);
        println!("  edges: {:?}", graph1.getattr("get_edgelist")?.call0()?);
        
        println!("Graph2 info:");
        println!("  vcount: {}", graph2.getattr("vcount")?.call0()?);
        println!("  ecount: {}", graph2.getattr("ecount")?.call0()?);
        println!("  labels: {:?}", graph2.getattr("vs")?.getattr("label")?);
        println!("  edges: {:?}", graph2.getattr("get_edgelist")?.call0()?);
        
        // Try to call WWL with these graphs
        let graphs = PyList::new(py, vec![graph1, graph2])?;
        
        // Try with minimal kwargs
        let wwl_kwargs = PyDict::new(py);
        wwl_kwargs.set_item("num_iterations", 3)?;
        wwl_kwargs.set_item("sinkhorn", false)?;
        
        println!("\nTrying WWL call...");
        match wwl.getattr("wwl")?.call((graphs,), Some(&wwl_kwargs)) {
            Ok(result) => {
                println!("✅ WWL worked: shape {:?}", result.getattr("shape")?);
            }
            Err(e) => {
                println!("❌ WWL failed: {}", e);
                
                println!("Error details: {}", e);
            }
        }
        
        Ok(())
    })?;
    
    Ok(())
}