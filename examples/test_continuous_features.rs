use petgraph::Graph;
use wwl_rust::{WWLKernel, KernelConfig};
use ndarray::Array2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create graphs of different sizes to test node features handling
    
    // Graph 1: 2 nodes
    let mut graph1 = Graph::new_undirected();
    let n1_1 = graph1.add_node(Some(1));
    let n1_2 = graph1.add_node(Some(2));
    graph1.add_edge(n1_1, n1_2, ());

    // Graph 2: 3 nodes  
    let mut graph2 = Graph::new_undirected();
    let n2_1 = graph2.add_node(Some(1));
    let n2_2 = graph2.add_node(Some(2));
    let n2_3 = graph2.add_node(Some(3));
    graph2.add_edge(n2_1, n2_2, ());
    graph2.add_edge(n2_2, n2_3, ());

    // Graph 3: 4 nodes
    let mut graph3 = Graph::new_undirected();
    let n3_1 = graph3.add_node(Some(1));
    let n3_2 = graph3.add_node(Some(2));
    let n3_3 = graph3.add_node(Some(3));
    let n3_4 = graph3.add_node(Some(4));
    graph3.add_edge(n3_1, n3_2, ());
    graph3.add_edge(n3_2, n3_3, ());
    graph3.add_edge(n3_3, n3_4, ());

    let graphs = vec![graph1, graph2, graph3];
    
    println!("Graph sizes:");
    for (i, graph) in graphs.iter().enumerate() {
        println!("  Graph {}: {} nodes", i + 1, graph.node_count());
    }
    
    // Test 1: Correctly sized node features (padded with zeros)
    // Max nodes is 4, so each graph needs 4 feature slots
    let mut node_features = Array2::zeros((3, 4)); // 3 graphs, 4 max nodes
    
    // Fill in features for graph 1 (2 nodes)
    node_features[[0, 0]] = 1.0; // node 1 feature
    node_features[[0, 1]] = 2.0; // node 2 feature
    // nodes 3,4 remain 0.0 (padding)
    
    // Fill in features for graph 2 (3 nodes)  
    node_features[[1, 0]] = 1.5;
    node_features[[1, 1]] = 2.5;
    node_features[[1, 2]] = 3.5;
    // node 4 remains 0.0 (padding)
    
    // Fill in features for graph 3 (4 nodes)
    node_features[[2, 0]] = 1.0;
    node_features[[2, 1]] = 2.0;
    node_features[[2, 2]] = 3.0;
    node_features[[2, 3]] = 4.0;
    
    println!("\nNode features matrix (3 graphs × 4 max nodes):");
    println!("{:?}", node_features);
    
    let kernel = WWLKernel::new()?;
    
    // This should work
    let result = kernel.compute_kernel_continuous(&graphs, &node_features, &KernelConfig::default())?;
    println!("\nKernel computation successful!");
    println!("Kernel matrix shape: {:?}", result.dim());
    
    // Test 2: Try with insufficient node feature columns (should fail)
    let bad_features = Array2::zeros((3, 2)); // Only 2 node slots but max graph has 4 nodes
    match kernel.compute_kernel_continuous(&graphs, &bad_features, &KernelConfig::default()) {
        Ok(_) => println!("ERROR: Should have failed with insufficient node features!"),
        Err(e) => println!("Expected error with insufficient node features: {}", e),
    }
    
    // Test 3: Try with wrong number of graphs (should fail)
    let bad_features2 = Array2::zeros((2, 4)); // Only 2 graphs but 3 provided
    match kernel.compute_kernel_continuous(&graphs, &bad_features2, &KernelConfig::default()) {
        Ok(_) => println!("ERROR: Should have failed with wrong graph count!"),
        Err(e) => println!("Expected error with wrong graph count: {}", e),
    }
    
    Ok(())
}