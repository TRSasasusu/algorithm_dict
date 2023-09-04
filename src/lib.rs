use pyo3::prelude::*;

mod sort;
mod graph;

#[pymodule]
fn algorithm_dict(py: Python, m: &PyModule) -> PyResult<()> {
    register_sort_module(py, m)?;
    register_graph_module(py, m)?;

    Ok(())
}

fn register_sort_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "sort")?;
    child_module.add_function(wrap_pyfunction!(sort::bubble_sort, parent_module)?)?;
    child_module.add_function(wrap_pyfunction!(sort::selection_sort, parent_module)?)?;
    child_module.add_function(wrap_pyfunction!(sort::merge_sort, parent_module)?)?;
    child_module.add_function(wrap_pyfunction!(sort::quick_sort, parent_module)?)?;
    py.import("sys")?.getattr("modules")?.set_item("algorithm_dict.sort", child_module)?;
    parent_module.add_submodule(child_module)?;
    Ok(())
}

fn register_graph_module(py: Python, parent_module: &PyModule) -> PyResult<()> {
    let child_module = PyModule::new(py, "graph")?;
    child_module.add_function(wrap_pyfunction!(graph::dijkstra::dijkstra, parent_module)?)?;
    child_module.add_function(wrap_pyfunction!(graph::topological_sort::topological_sort, parent_module)?)?;
    //py_run!(py, child_module, "import sys; sys.modules['algorithm_dict.graph'] = child_module");
    py.import("sys")?.getattr("modules")?.set_item("algorithm_dict.graph", child_module)?; // See https://github.com/PyO3/pyo3/issues/759#issuecomment-977835119
    parent_module.add_submodule(child_module)?;
    Ok(())
}
