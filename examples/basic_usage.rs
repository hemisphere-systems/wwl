use petgraph::Graph;
use wwl_rust::{DistanceConfig, KernelConfig, WWLKernel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut graph1 = Graph::new_undirected();
    let n1 = graph1.add_node(Some(1));
    let n2 = graph1.add_node(Some(2));
    graph1.add_edge(n1, n2, ());

    let mut graph2 = Graph::new_undirected();
    let n3 = graph2.add_node(Some(1));
    let n4 = graph2.add_node(Some(2));
    let n5 = graph2.add_node(Some(4));
    graph2.add_edge(n3, n4, ());
    graph2.add_edge(n4, n5, ());

    let graphs = vec![graph1, graph2];
    let kernel = WWLKernel::new()?;

    let kernel_matrix = kernel.compute_kernel_categorical(&graphs, &KernelConfig::default())?;
    let distance_matrix =
        kernel.compute_distance_categorical(&graphs, &DistanceConfig::default())?;

    assert_eq!(kernel_matrix.dim(), (2, 2));
    assert_eq!(distance_matrix.dim(), (2, 2));

    Ok(())
}
