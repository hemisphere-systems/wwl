use petgraph::Graph;
use wwl_rust::{WWLKernel, KernelConfig};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test with same-sized graphs first
    let mut graph1 = Graph::new_undirected();
    let n1 = graph1.add_node(Some(1));
    let n2 = graph1.add_node(Some(2));
    graph1.add_edge(n1, n2, ());

    let mut graph2 = Graph::new_undirected();
    let n3 = graph2.add_node(Some(3));
    let n4 = graph2.add_node(Some(4));
    graph2.add_edge(n3, n4, ());

    let graphs = vec![graph1, graph2];
    let kernel = WWLKernel::new()?;
    
    println!("Testing same-sized graphs (2 nodes each):");
    match kernel.compute_kernel_categorical(&graphs, &KernelConfig::default()) {
        Ok(result) => {
            println!("✅ Same-sized graphs work: {:?}", result.dim());
        }
        Err(e) => {
            println!("❌ Same-sized graphs failed: {}", e);
            return Err(e.into());
        }
    }

    // Now test different-sized graphs
    let mut graph3 = Graph::new_undirected();
    let n5 = graph3.add_node(Some(1));
    let n6 = graph3.add_node(Some(2));
    let n7 = graph3.add_node(Some(3));
    graph3.add_edge(n5, n6, ());
    graph3.add_edge(n6, n7, ());

    let mixed_graphs = vec![graphs[0].clone(), graph3];
    
    println!("Testing different-sized graphs (2 vs 3 nodes):");
    match kernel.compute_kernel_categorical(&mixed_graphs, &KernelConfig::default()) {
        Ok(result) => {
            println!("✅ Different-sized graphs work: {:?}", result.dim());
        }
        Err(e) => {
            println!("❌ Different-sized graphs failed: {}", e);
            return Err(e.into());
        }
    }

    Ok(())
}