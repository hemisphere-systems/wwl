use petgraph::Graph;
use wwl::{DistanceConfig, KernelConfig, WWLKernel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create first graph: 2 nodes with labels [1, 2]
    let mut graph1 = Graph::new_undirected();
    let n1 = graph1.add_node(Some(1));
    let n2 = graph1.add_node(Some(2));
    graph1.add_edge(n1, n2, ());

    // Create second graph: 3 nodes with labels [1, 2, 4]
    let mut graph2 = Graph::new_undirected();
    let n3 = graph2.add_node(Some(1));
    let n4 = graph2.add_node(Some(2));
    let n5 = graph2.add_node(Some(4));
    graph2.add_edge(n3, n4, ());
    graph2.add_edge(n4, n5, ());

    let graphs = vec![graph1, graph2];
    let kernel = WWLKernel::new()?;

    // Compute WWL kernel matrix
    let kernel_matrix = kernel.compute_kernel_categorical(&graphs, &KernelConfig::default())?;
    let distance_matrix =
        kernel.compute_distance_categorical(&graphs, &DistanceConfig::default())?;

    // Test matrix dimensions
    assert_eq!(kernel_matrix.dim(), (2, 2));
    assert_eq!(distance_matrix.dim(), (2, 2));

    // Test symmetry
    assert_eq!(kernel_matrix[[0, 1]], kernel_matrix[[1, 0]]);
    assert_eq!(distance_matrix[[0, 1]], distance_matrix[[1, 0]]);

    // Test diagonal properties
    assert_eq!(kernel_matrix[[0, 0]], 1.0); // Self-similarity is 1
    assert_eq!(kernel_matrix[[1, 1]], 1.0);
    assert_eq!(distance_matrix[[0, 0]], 0.0); // Self-distance is 0
    assert_eq!(distance_matrix[[1, 1]], 0.0);

    // Extract single distance value between the two graphs
    let distance_value = distance_matrix[[0, 1]]; // f64 value
    println!("Distance between graphs: {}", distance_value);

    Ok(())
}
