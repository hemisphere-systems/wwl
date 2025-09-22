use petgraph::Graph;
use wwl::{DistanceConfig, KernelConfig, WWLKernel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Graph 1: 2 nodes with labels [1, 2]
    let mut graph1 = Graph::new_undirected();
    let n1 = graph1.add_node(Some(1));
    let n2 = graph1.add_node(Some(2));
    graph1.add_edge(n1, n2, ());

    // Graph 2: 3 nodes with labels [1, 2, 3]
    let mut graph2 = Graph::new_undirected();
    let n3 = graph2.add_node(Some(1));
    let n4 = graph2.add_node(Some(2));
    let n5 = graph2.add_node(Some(3));
    graph2.add_edge(n3, n4, ());
    graph2.add_edge(n4, n5, ());

    let graphs = vec![graph1, graph2];
    let kernel = WWLKernel::new()?;

    let kernel_matrix = kernel.compute_kernel_categorical(&graphs, &KernelConfig::default())?;
    let distance_matrix =
        kernel.compute_distance_categorical(&graphs, &DistanceConfig::default())?;

    // Test that WWL handles different-sized graphs correctly
    assert_eq!(kernel_matrix.dim(), (2, 2));
    assert_eq!(distance_matrix.dim(), (2, 2));

    // Test expected values (should match Python implementation)
    let kernel_similarity = kernel_matrix[[0, 1]];
    let distance_value = distance_matrix[[0, 1]];

    assert!((kernel_similarity - 0.47236655).abs() < 1e-6);
    assert!((distance_value - 0.75).abs() < 1e-6);

    println!("Different-sized graphs processed correctly");

    Ok(())
}

