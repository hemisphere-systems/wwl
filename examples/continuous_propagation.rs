use ndarray::Array2;
use petgraph::Graph;
use wwl::{KernelConfig, WWLKernel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create graphs without labels for continuous propagation
    let mut graph1 = Graph::new_undirected();
    let n1_1 = graph1.add_node(None);
    let n1_2 = graph1.add_node(None);
    graph1.add_edge(n1_1, n1_2, ());

    let mut graph2 = Graph::new_undirected();
    let n2_1 = graph2.add_node(None);
    let n2_2 = graph2.add_node(None);
    graph2.add_edge(n2_1, n2_2, ());

    let graphs = vec![graph1, graph2];

    // Node features: 2 graphs × 2 nodes (same size)
    let mut node_features = Array2::zeros((2, 2));
    node_features[[0, 0]] = 1.0;
    node_features[[0, 1]] = 2.0;
    node_features[[1, 0]] = 1.5;
    node_features[[1, 1]] = 2.5;

    let kernel = WWLKernel::new()?;
    let _result = kernel.compute_kernel_continuous(&graphs, &node_features, &KernelConfig::default())?;

    // Test validation
    let bad_features = Array2::zeros((2, 1));
    assert!(kernel.compute_kernel_continuous(&graphs, &bad_features, &KernelConfig::default()).is_err());

    Ok(())
}

