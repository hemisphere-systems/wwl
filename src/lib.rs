use pyo3::prelude::*;
use pyo3::types::{PyList, PyDict, PyAny};
use numpy::PyArray2;
use ndarray::Array2;
use petgraph::{Graph, Undirected};
use std::collections::HashMap;

pub type GraphType = Graph<Option<i32>, (), Undirected>;

pub struct WWLKernel {
    wwl_module: Py<PyAny>,
}

impl WWLKernel {
    pub fn new() -> PyResult<Self> {
        pyo3::prepare_freethreaded_python();
        Python::with_gil(|py| {
            let wwl_module = py.import("wwl")?.into();
            Ok(WWLKernel { wwl_module })
        })
    }

    pub fn compute_kernel(
        &self,
        graphs: &[GraphType],
        node_features: Option<&Array2<f64>>,
        num_iterations: Option<usize>,
        sinkhorn: Option<bool>,
        gamma: Option<f64>,
    ) -> PyResult<Array2<f64>> {
        Python::with_gil(|py| {
            let wwl_module = self.wwl_module.as_ref(py);
            
            // Convert petgraph graphs to Python igraph format
            let py_graphs = self.convert_graphs_to_python(py, graphs)?;

            // Prepare kwargs
            let kwargs = PyDict::new(py);
            
            if let Some(features) = node_features {
                let py_features = PyArray2::from_array(py, features);
                kwargs.set_item("node_features", py_features)?;
            }
            kwargs.set_item("num_iterations", num_iterations.unwrap_or(3))?;
            kwargs.set_item("sinkhorn", sinkhorn.unwrap_or(false))?;
            if let Some(g) = gamma {
                kwargs.set_item("gamma", g)?;
            }

            // Call WWL function
            let result = wwl_module.getattr("wwl")?.call((py_graphs,), Some(kwargs))?;
            
            // Extract result as ndarray
            let py_array: &PyArray2<f64> = result.extract()?;
            Ok(py_array.readonly().as_array().to_owned())
        })
    }

    pub fn compute_wasserstein_distance(
        &self,
        graphs: &[GraphType],
        node_features: Option<&Array2<f64>>,
        num_iterations: Option<usize>,
        sinkhorn: Option<bool>,
    ) -> PyResult<Array2<f64>> {
        Python::with_gil(|py| {
            let wwl_module = self.wwl_module.as_ref(py);
            
            // Convert petgraph graphs to Python igraph format
            let py_graphs = self.convert_graphs_to_python(py, graphs)?;

            // Prepare kwargs
            let kwargs = PyDict::new(py);
            
            if let Some(features) = node_features {
                let py_features = PyArray2::from_array(py, features);
                kwargs.set_item("node_features", py_features)?;
            }
            kwargs.set_item("num_iterations", num_iterations.unwrap_or(3))?;
            kwargs.set_item("sinkhorn", sinkhorn.unwrap_or(false))?;

            // Call Wasserstein distance function
            let result = wwl_module.getattr("pairwise_wasserstein_distance")?.call((py_graphs,), Some(kwargs))?;
            
            // Extract result as ndarray
            let py_array: &PyArray2<f64> = result.extract()?;
            Ok(py_array.readonly().as_array().to_owned())
        })
    }

    fn convert_graphs_to_python(&self, py: Python, graphs: &[GraphType]) -> PyResult<&PyList> {
        // Import igraph module
        let igraph = py.import("igraph")?;
        
        let py_graphs: PyResult<Vec<_>> = graphs.iter().map(|graph| {
            // Create edge list
            let edges: Vec<(usize, usize)> = graph.edge_indices()
                .map(|edge_idx| {
                    let (a, b) = graph.edge_endpoints(edge_idx).unwrap();
                    (a.index(), b.index())
                })
                .collect();
            
            // Create node attributes if they exist
            let node_labels: Vec<Option<i32>> = graph.node_weights().cloned().collect();
            
            // Create Python igraph object
            let args = (edges, graph.node_count());
            let kwargs = PyDict::new(py);
            kwargs.set_item("directed", false)?;
            
            let py_graph = igraph.getattr("Graph")?.call(args, Some(kwargs))?;
            
            // Add node labels if they exist
            if node_labels.iter().any(|label| label.is_some()) {
                let labels: Vec<i32> = node_labels.iter()
                    .map(|label| label.unwrap_or(0))
                    .collect();
                
                let vs = py_graph.getattr("vs")?;
                vs.set_item("label", labels)?;
            }
            
            Ok(py_graph)
        }).collect();
        
        PyList::new(py, py_graphs?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    #[test]
    fn test_wwl_kernel_creation() {
        let kernel = WWLKernel::new();
        assert!(kernel.is_ok());
    }

    #[test]
    fn test_basic_graph_conversion() {
        let kernel = WWLKernel::new().unwrap();
        let mut graph = Graph::new_undirected();
        let n1 = graph.add_node(Some(1));
        let n2 = graph.add_node(Some(2));
        graph.add_edge(n1, n2, ());
        
        let graphs = vec![graph];
        
        Python::with_gil(|py| {
            let py_graphs = kernel.convert_graphs_to_python(py, &graphs);
            assert!(py_graphs.is_ok());
        });
    }
}
