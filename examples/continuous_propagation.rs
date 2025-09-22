use ndarray::Array2;
use petgraph::Graph;
use wwl::{KernelConfig, WWLKernel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create graphs with different numbers of nodes
    let mut graph1 = Graph::new_undirected();
    let n1_1 = graph1.add_node(Some(1));
    let n1_2 = graph1.add_node(Some(2));
    graph1.add_edge(n1_1, n1_2, ());

    let mut graph2 = Graph::new_undirected();
    let n2_1 = graph2.add_node(Some(1));
    let n2_2 = graph2.add_node(Some(2));
    let n2_3 = graph2.add_node(Some(3));
    graph2.add_edge(n2_1, n2_2, ());
    graph2.add_edge(n2_2, n2_3, ());

    let graphs = vec![graph1, graph2];

    // Node features: 2 graphs × 3 max nodes (padded with zeros)
    let mut node_features = Array2::zeros((2, 3));

    // Graph 1 features (2 nodes + 1 padding)
    node_features[[0, 0]] = 1.0;
    node_features[[0, 1]] = 2.0;
    // node_features[[0, 2]] = 0.0 (padding)

    // Graph 2 features (3 nodes)
    node_features[[1, 0]] = 1.5;
    node_features[[1, 1]] = 2.5;
    node_features[[1, 2]] = 3.5;

    let kernel = WWLKernel::new()?;
    let result =
        kernel.compute_kernel_continuous(&graphs, &node_features, &KernelConfig::default())?;

    println!("Continuous propagation kernel computed successfully");
    assert_eq!(result.dim(), (2, 2));

    // Test validation: insufficient node features should fail
    let bad_features = Array2::zeros((2, 1)); // Only 1 node slot but graphs have 2-3 nodes
    assert!(kernel
        .compute_kernel_continuous(&graphs, &bad_features, &KernelConfig::default())
        .is_err());

    Ok(())
}

