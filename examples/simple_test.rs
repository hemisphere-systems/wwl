use petgraph::Graph;
use wwl_rust::WWLKernel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Test with minimal labeled graphs first
    let mut graph1 = Graph::new_undirected();
    let n1 = graph1.add_node(Some(1));
    let n2 = graph1.add_node(Some(2));
    graph1.add_edge(n1, n2, ());

    let graphs = vec![graph1];
    let kernel = WWLKernel::new()?;
    
    let result = kernel.compute_kernel(&graphs, None, Some(1), Some(false), None)?;
    assert_eq!(result.dim(), (1, 1));
    
    Ok(())
}