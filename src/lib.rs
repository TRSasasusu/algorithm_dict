use pyo3::prelude::*;

mod sort;
mod graph;

#[pymodule]
fn algorithm_dict(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sort::bubble_sort, m)?)?;
    m.add_function(wrap_pyfunction!(sort::selection_sort, m)?)?;
    m.add_function(wrap_pyfunction!(sort::merge_sort, m)?)?;
    m.add_function(wrap_pyfunction!(sort::quick_sort, m)?)?;

    m.add_function(wrap_pyfunction!(graph::dijkstra::dijkstra, m)?)?;
    m.add_function(wrap_pyfunction!(graph::topological_sort::topological_sort, m)?)?;

    Ok(())
}
