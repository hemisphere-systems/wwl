use pyo3::prelude::*;
use pyo3::types::{PyDict, PyList};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Python::attach(|py| -> PyResult<()> {
        let igraph = py.import("igraph")?;
        let wwl = py.import("wwl")?;
        
        // Create the exact same graphs that work in pure Python
        // Graph 1: 2 nodes, 1 edge
        let graph1 = igraph.getattr("Graph")?.call((
            vec![(0usize, 1usize)], 
            2usize
        ), None)?;
        let vs1 = graph1.getattr("vs")?;
        vs1.setattr("label", vec![1i32, 2i32])?;
        
        // Graph 2: 3 nodes, 2 edges  
        let graph2 = igraph.getattr("Graph")?.call((
            vec![(0usize, 1usize), (1usize, 2usize)], 
            3usize
        ), None)?;
        let vs2 = graph2.getattr("vs")?;
        vs2.setattr("label", vec![1i32, 2i32, 3i32])?;
        
        println!("Created graphs successfully");
        
        // Test individually first
        println!("Testing graph 1 alone:");
        match wwl.getattr("wwl")?.call((vec![graph1.clone()],), None) {
            Ok(result) => println!("✅ Graph 1 alone works: {:?}", result.getattr("shape")?),
            Err(e) => println!("❌ Graph 1 alone failed: {}", e),
        }
        
        println!("Testing graph 2 alone:");
        match wwl.getattr("wwl")?.call((vec![graph2.clone()],), None) {
            Ok(result) => println!("✅ Graph 2 alone works: {:?}", result.getattr("shape")?),
            Err(e) => println!("❌ Graph 2 alone failed: {}", e),
        }
        
        // Now test both together
        println!("Testing both graphs together:");
        let graphs_vec = vec![graph1, graph2];
        let graphs_list = PyList::new(py, graphs_vec)?;
        
        match wwl.getattr("wwl")?.call((graphs_list,), None) {
            Ok(result) => {
                println!("✅ Both graphs work: {:?}", result.getattr("shape")?);
                println!("Result: {:?}", result);
            },
            Err(e) => {
                println!("❌ Both graphs failed: {}", e);
                println!("This proves the issue is in our WWLKernel implementation, not the graphs themselves");
            }
        }
        
        Ok(())
    })?;
    
    Ok(())
}