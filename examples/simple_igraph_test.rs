use pyo3::prelude::*;
use pyo3::types::PyDict;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Python::attach(|py| -> PyResult<()> {
        let igraph = py.import("igraph")?;
        
        // Create a simple graph and check label setting
        let edges = vec![(0usize, 1usize)];
        let kwargs = PyDict::new(py);
        kwargs.set_item("directed", false)?;
        let graph = igraph.getattr("Graph")?.call((edges, 2usize), Some(&kwargs))?;
        
        println!("Created graph with {} nodes", graph.getattr("vcount")?.call0()?);
        
        // Try different ways to set labels
        println!("Trying to set labels...");
        
        // Method 1: Direct assignment
        let vs = graph.getattr("vs")?;
        vs.setattr("label", vec![1i32, 2i32])?;
        
        // Check if it worked
        match vs.getattr("label") {
            Ok(labels) => {
                println!("✅ Labels set: {:?}", labels);
                
                // Try WWL with this graph
                let wwl = py.import("wwl")?;
                match wwl.getattr("wwl")?.call((vec![graph],), None) {
                    Ok(result) => println!("✅ Single graph WWL works"),
                    Err(e) => println!("❌ Single graph WWL failed: {}", e),
                }
            },
            Err(e) => {
                println!("❌ Label access failed: {}", e);
                
                // Try alternative method
                println!("Trying alternative label setting...");
                graph.setattr("vertex_attrs", PyDict::new(py))?;
                let vertex_attrs = graph.getattr("vertex_attrs")?;
                vertex_attrs.set_item("label", vec![1i32, 2i32])?;
            }
        }
        
        Ok(())
    })?;
    
    Ok(())
}