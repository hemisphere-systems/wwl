use petgraph::Graph;
use wwl_rust::{WWLKernel, KernelConfig};
use pyo3::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Python::attach(|py| -> PyResult<()> {
        let igraph = py.import("igraph")?;
        let wwl = py.import("wwl")?;
        
        // Create working direct graphs
        let graph1_direct = igraph.getattr("Graph")?.call((
            vec![(0usize, 1usize)], 
            2usize
        ), None)?;
        let vs1 = graph1_direct.getattr("vs")?;
        vs1.setattr("label", vec![1i32, 2i32])?;
        
        let graph2_direct = igraph.getattr("Graph")?.call((
            vec![(0usize, 1usize), (1usize, 2usize)], 
            3usize
        ), None)?;
        let vs2 = graph2_direct.getattr("vs")?;
        vs2.setattr("label", vec![1i32, 2i32, 3i32])?;
        
        println!("=== WORKING DIRECT GRAPHS ===");
        println!("Graph 1 type: {:?}", graph1_direct.get_type());
        println!("Graph 1 str: {:?}", graph1_direct.str());
        println!("Graph 2 type: {:?}", graph2_direct.get_type());
        println!("Graph 2 str: {:?}", graph2_direct.str());
        
        // Test working graphs
        let working_graphs = vec![graph1_direct, graph2_direct];
        match wwl.getattr("wwl")?.call((working_graphs,), None) {
            Ok(result) => println!("✅ Direct graphs work: {:?}", result.getattr("shape")?),
            Err(e) => println!("❌ Direct graphs failed: {}", e),
        }
        
        Ok(())
    })?;
    
    // Now test our conversion
    println!("\n=== OUR CONVERSION ===");
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
        Ok(result) => println!("✅ Our conversion works: {:?}", result.dim()),
        Err(e) => println!("❌ Our conversion failed: {}", e),
    }
    
    Ok(())
}