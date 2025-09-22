use petgraph::Graph;
use wwl_rust::WWLKernel;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create two identical graphs - should have distance 0
    let mut graph1 = Graph::new_undirected();
    let n1_1 = graph1.add_node(Some(1));
    let n1_2 = graph1.add_node(Some(2));
    graph1.add_edge(n1_1, n1_2, ());

    let mut graph2 = Graph::new_undirected();
    let n2_1 = graph2.add_node(Some(1));
    let n2_2 = graph2.add_node(Some(2));
    graph2.add_edge(n2_1, n2_2, ());

    // Create a different graph - should have distance > 0
    let mut graph3 = Graph::new_undirected();
    let n3_1 = graph3.add_node(Some(3));
    let n3_2 = graph3.add_node(Some(4));
    graph3.add_edge(n3_1, n3_2, ());

    let graphs = vec![graph1, graph2, graph3];
    let kernel = WWLKernel::new()?;
    
    let kernel_matrix = kernel.compute_kernel(&graphs, None, Some(3), Some(false), None)?;
    let distance_matrix = kernel.compute_wasserstein_distance(&graphs, None, Some(3), Some(false), Some(false))?;
    
    println!("Kernel matrix:\n{:?}", kernel_matrix);
    println!("Distance matrix:\n{:?}", distance_matrix);
    
    // Verify properties
    assert_eq!(kernel_matrix.dim(), (3, 3));
    assert_eq!(distance_matrix.dim(), (3, 3));
    
    // Distance matrix should be symmetric
    for i in 0..3 {
        for j in 0..3 {
            assert!((distance_matrix[[i, j]] - distance_matrix[[j, i]]).abs() < 1e-10);
        }
    }
    
    // Diagonal should be 0 (distance from graph to itself)
    for i in 0..3 {
        assert!(distance_matrix[[i, i]].abs() < 1e-10, "Distance from graph {} to itself: {}", i, distance_matrix[[i, i]]);
    }
    
    // Identical graphs (0,1) should have smaller distance than different graphs (0,2)
    assert!(distance_matrix[[0, 1]] <= distance_matrix[[0, 2]] + 1e-10, 
            "Distance between identical graphs: {}, distance to different graph: {}", 
            distance_matrix[[0, 1]], distance_matrix[[0, 2]]);
    
    println!("✅ Distance matrix verification passed!");
    Ok(())
}