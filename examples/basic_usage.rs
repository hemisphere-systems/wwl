use wwl_rust::{WWLKernel, GraphType};
use ndarray::Array2;
use petgraph::Graph;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the WWL kernel
    let kernel = WWLKernel::new()?;
    
    // Create some example graphs using petgraph
    let mut graph1: GraphType = Graph::new_undirected();
    let n1 = graph1.add_node(Some(1)); // Node with label 1
    let n2 = graph1.add_node(Some(2)); // Node with label 2
    let n3 = graph1.add_node(Some(1)); // Node with label 1
    graph1.add_edge(n1, n2, ());
    graph1.add_edge(n2, n3, ());
    
    let mut graph2: GraphType = Graph::new_undirected();
    let m1 = graph2.add_node(Some(2)); // Node with label 2
    let m2 = graph2.add_node(Some(3)); // Node with label 3
    let m3 = graph2.add_node(Some(2)); // Node with label 2
    graph2.add_edge(m1, m2, ());
    graph2.add_edge(m2, m3, ());
    graph2.add_edge(m1, m3, ()); // Triangle
    
    let graphs = vec![graph1, graph2];
    
    // Example node features (optional - can be None for categorical labels)
    let node_features: Option<Array2<f64>> = None;
    
    // Compute WWL kernel matrix
    let kernel_matrix = kernel.compute_kernel(
        &graphs,
        node_features.as_ref(),
        Some(3),     // num_iterations
        Some(false), // sinkhorn
        Some(1.0),   // gamma
    )?;
    
    println!("Kernel matrix shape: {:?}", kernel_matrix.dim());
    println!("Kernel matrix:\n{}", kernel_matrix);
    
    // Compute Wasserstein distances
    let distance_matrix = kernel.compute_wasserstein_distance(
        &graphs,
        node_features.as_ref(),
        Some(3),     // num_iterations
        Some(false), // sinkhorn
    )?;
    
    println!("Distance matrix shape: {:?}", distance_matrix.dim());
    println!("Distance matrix:\n{}", distance_matrix);
    
    Ok(())
}