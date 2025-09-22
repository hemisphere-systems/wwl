use petgraph::Graph;
use wwl::{DistanceConfig, GraphType, KernelConfig, WWLKernel};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let graphs = create_labeled_graphs();
    let kernel = WWLKernel::new()?;

    let kernel_matrix = kernel.compute_kernel_categorical(&graphs, &KernelConfig::default())?;
    let distance_matrix =
        kernel.compute_distance_categorical(&graphs, &DistanceConfig::default())?;

    assert_eq!(kernel_matrix.dim(), (2, 2));
    assert_eq!(distance_matrix.dim(), (2, 2));

    Ok(())
}

fn create_labeled_graphs() -> Vec<GraphType> {
    let mut graph1 = Graph::new_undirected();
    let n1_1 = graph1.add_node(Some(1));
    let n1_2 = graph1.add_node(Some(2));
    let n1_3 = graph1.add_node(Some(1));
    graph1.add_edge(n1_1, n1_2, ());
    graph1.add_edge(n1_2, n1_3, ());

    let mut graph2 = Graph::new_undirected();
    let n2_1 = graph2.add_node(Some(2));
    let n2_2 = graph2.add_node(Some(1));
    let n2_3 = graph2.add_node(Some(2));
    graph2.add_edge(n2_1, n2_2, ());
    graph2.add_edge(n2_2, n2_3, ());

    vec![graph1, graph2]
}

