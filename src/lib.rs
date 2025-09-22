//! WWL (Wasserstein Weisfeiler-Lehman) Rust Bindings
//!
//! This crate provides Rust bindings for the WWL Graph Kernels Python library.
//!
//! The library correctly handles graphs of different sizes for categorical
//! propagation (labeled graphs) and supports continuous propagation with
//! proper node features.
//!
//! ## Node Features
//!
//! Node features are numerical vectors associated with each node in a graph.
//! They enable continuous propagation schemes where the algorithm operates on
//! real-valued node attributes instead of discrete labels.

use petgraph::{Graph, Undirected};

use ndarray::Array2;
use numpy::{PyArray2, PyArrayMethods};
use pyo3::prelude::*;
use pyo3::types::{PyAny, PyDict, PyList};
use pythonize::pythonize;

/// Graph type alias for undirected graphs with optional integer node weights
pub type GraphType = Graph<Option<i32>, (), Undirected>;

/// WWL Kernel implementation
pub struct WWLKernel {
    wwl: Py<PyAny>,
}

/// Configuration for WWL kernel computation
#[derive(Clone)]
pub struct KernelConfig {
    pub num_iterations: usize,
    pub sinkhorn: bool,
    pub gamma: Option<f64>,
}

impl Default for KernelConfig {
    fn default() -> Self {
        Self {
            num_iterations: 3,
            sinkhorn: false,
            gamma: None,
        }
    }
}

/// Configuration for Wasserstein distance computation
#[derive(Clone)]
pub struct DistanceConfig {
    pub num_iterations: usize,
    pub sinkhorn: bool,
    pub enforce_continuous: bool,
}

impl Default for DistanceConfig {
    fn default() -> Self {
        Self {
            num_iterations: 3,
            sinkhorn: false,
            enforce_continuous: false,
        }
    }
}

impl WWLKernel {
    /// Creates a new WWLKernel instance
    pub fn new() -> Result<Self, String> {
        Python::attach(|py| match py.import("wwl") {
            Ok(wwl) => Ok(WWLKernel {
                wwl: wwl.unbind().into(),
            }),
            Err(e) => Err(format!("Failed to import WWL module: {}", e)),
        })
    }

    /// Computes the WWL kernel matrix for labeled graphs (categorical propagation)
    pub fn compute_kernel_categorical(
        &self,
        graphs: &[GraphType],
        config: &KernelConfig,
    ) -> Result<Array2<f64>, String> {
        self.compute_kernel_impl(graphs, None, config)
    }

    /// Computes the WWL kernel matrix with node features (continuous propagation)
    ///
    /// # Node Features Format
    ///
    /// Node features should be provided as a 2D array where:
    /// - Each row represents a graph
    /// - Each column represents a node feature dimension
    /// - For graphs with different numbers of nodes, pad with zeros
    ///
    /// Example: For 2 graphs with 3 nodes each and 1 feature dimension:
    /// ```text
    /// [[graph1_node1_feat, graph1_node2_feat, graph1_node3_feat],
    ///  [graph2_node1_feat, graph2_node2_feat, graph2_node3_feat]]
    /// ```
    pub fn compute_kernel_continuous(
        &self,
        graphs: &[GraphType],
        node_features: &Array2<f64>,
        config: &KernelConfig,
    ) -> Result<Array2<f64>, String> {
        self.validate_node_features(graphs, node_features)?;
        self.compute_kernel_impl(graphs, Some(node_features), config)
    }

    /// Computes pairwise Wasserstein distances for labeled graphs
    pub fn compute_distance_categorical(
        &self,
        graphs: &[GraphType],
        config: &DistanceConfig,
    ) -> Result<Array2<f64>, String> {
        self.compute_distance_impl(graphs, None, config)
    }

    /// Computes pairwise Wasserstein distances with node features
    pub fn compute_distance_continuous(
        &self,
        graphs: &[GraphType],
        node_features: &Array2<f64>,
        config: &DistanceConfig,
    ) -> Result<Array2<f64>, String> {
        self.validate_node_features(graphs, node_features)?;
        self.compute_distance_impl(graphs, Some(node_features), config)
    }

    fn validate_node_features(
        &self,
        graphs: &[GraphType],
        node_features: &Array2<f64>,
    ) -> Result<(), String> {
        let (num_graphs, max_nodes) = node_features.dim();

        if num_graphs != graphs.len() {
            return Err(format!(
                "Node features has {} graphs but {} graphs provided",
                num_graphs,
                graphs.len()
            ));
        }

        let actual_max_nodes = graphs.iter().map(|g| g.node_count()).max().unwrap_or(0);
        if max_nodes < actual_max_nodes {
            return Err(format!(
                "Node features has {} node slots but largest graph has {} nodes",
                max_nodes, actual_max_nodes
            ));
        }

        Ok(())
    }

    fn compute_kernel_impl(
        &self,
        graphs: &[GraphType],
        node_features: Option<&Array2<f64>>,
        config: &KernelConfig,
    ) -> Result<Array2<f64>, String> {
        Python::attach(|py| {
            let wwl_module = self.wwl.bind(py);
            let py_graphs = self
                .convert_graphs_to_python(py, graphs)
                .map_err(|e| format!("Graph conversion failed: {}", e))?;

            let kwargs = PyDict::new(py);

            if let Some(features) = node_features {
                let py_features = PyArray2::from_array(py, features);
                kwargs.set_item("node_features", py_features).unwrap();
            }
            kwargs
                .set_item("num_iterations", config.num_iterations)
                .unwrap();
            kwargs.set_item("sinkhorn", config.sinkhorn).unwrap();
            if let Some(g) = config.gamma {
                kwargs.set_item("gamma", g).unwrap();
            }

            // Call WWL with explicit node_features=None for categorical mode
            if node_features.is_none() {
                kwargs.set_item("node_features", py.None()).unwrap();
            }

            let result = wwl_module
                .getattr("wwl")
                .and_then(|f| f.call((&py_graphs,), Some(&kwargs)))
                .map_err(|e| format!("WWL computation failed: {}", e))?;

            result
                .downcast::<PyArray2<f64>>()
                .map(|py_array| py_array.readonly().as_array().to_owned())
                .map_err(|e| format!("Result extraction failed: {}", e))
        })
    }

    fn compute_distance_impl(
        &self,
        graphs: &[GraphType],
        node_features: Option<&Array2<f64>>,
        config: &DistanceConfig,
    ) -> Result<Array2<f64>, String> {
        Python::attach(|py| {
            let wwl_module = self.wwl.bind(py);
            let py_graphs = self
                .convert_graphs_to_python(py, graphs)
                .map_err(|e| format!("Graph conversion failed: {}", e))?;

            let kwargs = PyDict::new(py);

            if let Some(features) = node_features {
                let py_features = PyArray2::from_array(py, features);
                kwargs.set_item("node_features", py_features).unwrap();
            }
            kwargs
                .set_item("num_iterations", config.num_iterations)
                .unwrap();
            kwargs.set_item("sinkhorn", config.sinkhorn).unwrap();
            kwargs
                .set_item("enforce_continuous", config.enforce_continuous)
                .unwrap();

            let result = wwl_module
                .getattr("pairwise_wasserstein_distance")
                .and_then(|f| f.call((&py_graphs,), Some(&kwargs)))
                .map_err(|e| format!("Wasserstein distance computation failed: {}", e))?;

            result
                .downcast::<PyArray2<f64>>()
                .map(|py_array| py_array.readonly().as_array().to_owned())
                .map_err(|e| format!("Result extraction failed: {}", e))
        })
    }

    /// Legacy method - use compute_kernel_categorical or compute_kernel_continuous instead
    #[deprecated(note = "Use compute_kernel_categorical or compute_kernel_continuous instead")]
    pub fn compute_kernel(
        &self,
        graphs: &[GraphType],
        node_features: Option<&Array2<f64>>,
        num_iterations: Option<usize>,
        sinkhorn: Option<bool>,
        gamma: Option<f64>,
    ) -> Result<Array2<f64>, String> {
        let config = KernelConfig {
            num_iterations: num_iterations.unwrap_or(3),
            sinkhorn: sinkhorn.unwrap_or(false),
            gamma,
        };
        self.compute_kernel_impl(graphs, node_features, &config)
    }

    /// Legacy method - use compute_distance_categorical or compute_distance_continuous instead
    #[deprecated(note = "Use compute_distance_categorical or compute_distance_continuous instead")]
    pub fn compute_wasserstein_distance(
        &self,
        graphs: &[GraphType],
        node_features: Option<&Array2<f64>>,
        num_iterations: Option<usize>,
        sinkhorn: Option<bool>,
        enforce_continuous: Option<bool>,
    ) -> Result<Array2<f64>, String> {
        let config = DistanceConfig {
            num_iterations: num_iterations.unwrap_or(3),
            sinkhorn: sinkhorn.unwrap_or(false),
            enforce_continuous: enforce_continuous.unwrap_or(false),
        };
        self.compute_distance_impl(graphs, node_features, &config)
    }

    fn convert_graphs_to_python(&self, py: Python, graphs: &[GraphType]) -> PyResult<Py<PyList>> {
        let igraph = py.import("igraph")?;

        // WWL should handle different graph sizes automatically

        let py_graphs: PyResult<Vec<_>> = graphs
            .iter()
            .map(|graph| {
                // Convert edges using pythonize for proper serialization
                let edges = graph
                    .edge_indices()
                    .map(|edge_idx| {
                        let (a, b) = graph.edge_endpoints(edge_idx).unwrap();
                        let a = a.index();
                        let b = b.index();

                        vec![a, b]
                    })
                    .collect::<Vec<Vec<usize>>>();

                let edges = PyArray2::from_vec2(py, &edges)?;

                let node_labels: Vec<Option<i32>> = graph.node_weights().cloned().collect();

                // Create igraph - must specify directed=False for undirected graphs
                let kwargs = PyDict::new(py);
                kwargs.set_item("edges", edges)?;
                kwargs.set_item("n", graph.node_count())?;
                kwargs.set_item("directed", false)?;

                let py_graph = igraph.getattr("Graph")?.call((), Some(&kwargs))?;

                // Always set labels if any nodes have labels
                if node_labels.iter().any(|label| label.is_some()) {
                    let labels: Vec<i32> =
                        node_labels.iter().map(|label| label.unwrap_or(0)).collect();

                    // Use pythonize to ensure proper conversion
                    let py_labels = pythonize(py, &labels)?;

                    let vs = py_graph.getattr("vs")?;
                    vs.set_item("label", py_labels)?;
                }


                Ok(py_graph.clone())
            })
            .collect();

        let py_graphs_vec = py_graphs?;
        let py_list = PyList::new(py, py_graphs_vec)?;

        Ok(py_list.unbind())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    #[test]
    fn test_config_defaults() {
        let kernel_config = KernelConfig::default();
        assert_eq!(kernel_config.num_iterations, 3);
        assert!(!kernel_config.sinkhorn);
        assert!(kernel_config.gamma.is_none());

        let distance_config = DistanceConfig::default();
        assert_eq!(distance_config.num_iterations, 3);
        assert!(!distance_config.sinkhorn);
        assert!(!distance_config.enforce_continuous);
    }

    #[test]
    fn test_node_features_validation() {
        match WWLKernel::new() {
            Ok(kernel) => {
                let mut graph = Graph::new_undirected();
                graph.add_node(None);
                graph.add_node(None);
                let graphs = vec![graph];

                // Wrong number of graphs
                let bad_features = Array2::zeros((2, 2)); // 2 graphs, but only 1 provided
                assert!(kernel
                    .validate_node_features(&graphs, &bad_features)
                    .is_err());

                // Too few node slots
                let bad_features = Array2::zeros((1, 1)); // 1 node slot, but graph has 2 nodes
                assert!(kernel
                    .validate_node_features(&graphs, &bad_features)
                    .is_err());

                // Correct format
                let good_features = Array2::zeros((1, 2)); // 1 graph, 2 nodes
                assert!(kernel
                    .validate_node_features(&graphs, &good_features)
                    .is_ok());
            }
            Err(_) => {
                println!("Skipping validation test - WWL module not available");
            }
        }
    }

    #[test]
    fn test_graph_type_creation() {
        let mut graph: GraphType = Graph::new_undirected();
        let n1 = graph.add_node(Some(1));
        let n2 = graph.add_node(Some(2));
        graph.add_edge(n1, n2, ());

        assert_eq!(graph.node_count(), 2);
        assert_eq!(graph.edge_count(), 1);

        let weights: Vec<_> = graph.node_weights().collect();
        assert_eq!(weights.len(), 2);
        assert_eq!(*weights[0], Some(1));
        assert_eq!(*weights[1], Some(2));
    }
}
