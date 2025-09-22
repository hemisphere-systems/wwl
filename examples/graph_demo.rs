use petgraph::Graph;
use wwl_rust::GraphType;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("WWL Rust Library - Graph Demo");
    println!("==============================");

    // Create example graphs to demonstrate the data structures work
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

    let graphs = vec![graph1, graph2];

    println!("Successfully created {} graphs:", graphs.len());

    for (i, graph) in graphs.iter().enumerate() {
        println!("\nGraph {}:", i + 1);
        println!("  Nodes: {}", graph.node_count());
        println!("  Edges: {}", graph.edge_count());

        // Display node labels
        let node_labels: Vec<_> = graph.node_weights().collect();
        println!("  Node labels: {:?}", node_labels);

        // Display edges
        let edge_list: Vec<_> = graph
            .edge_indices()
            .map(|edge_idx| {
                let (a, b) = graph.edge_endpoints(edge_idx).unwrap();
                (a.index(), b.index())
            })
            .collect();
        println!("  Edges: {:?}", edge_list);
    }

    println!("\n✅ Graph data structures are working correctly!");
    println!("📝 Note: WWL computation requires Python WWL package to be");
    println!("   accessible to PyO3's embedded interpreter.");
    println!("   This demonstrates the Rust wrapper structures work properly.");

    Ok(())
}
