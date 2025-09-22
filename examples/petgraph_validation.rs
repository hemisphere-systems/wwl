use petgraph::Graph;
use wwl::GraphType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create example graphs to demonstrate the data structures
    let mut graph1: GraphType = Graph::new_undirected();
    let n1_1 = graph1.add_node(Some(1));
    let n1_2 = graph1.add_node(Some(2));
    let n1_3 = graph1.add_node(Some(1));
    graph1.add_edge(n1_1, n1_2, ());
    graph1.add_edge(n1_2, n1_3, ());

    let mut graph2: GraphType = Graph::new_undirected();
    let n2_1 = graph2.add_node(Some(2));
    let n2_2 = graph2.add_node(Some(1));
    let n2_3 = graph2.add_node(Some(2));
    graph2.add_edge(n2_1, n2_2, ());
    graph2.add_edge(n2_2, n2_3, ());

    // Test basic graph properties
    assert_eq!(graph1.node_count(), 3);
    assert_eq!(graph1.edge_count(), 2);
    assert_eq!(graph2.node_count(), 3);
    assert_eq!(graph2.edge_count(), 2);

    // Test node labels
    let graph1_labels: Vec<_> = graph1.node_weights().cloned().collect();
    let graph2_labels: Vec<_> = graph2.node_weights().cloned().collect();
    assert_eq!(graph1_labels, vec![Some(1), Some(2), Some(1)]);
    assert_eq!(graph2_labels, vec![Some(2), Some(1), Some(2)]);

    println!("Graph structures validated successfully");

    Ok(())
}
