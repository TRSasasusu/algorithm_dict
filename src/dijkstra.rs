use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyfunction]
pub fn dijkstra(mat: PyObject, start_index: usize, end_index: usize) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        if let Ok(mat) = mat.extract::<Vec<Vec<i64>>>(py) {
            return Ok(_dijkstra(&mat, start_index, end_index, 0).to_object(py));
        }
        else if let Ok(mat) = mat.extract::<Vec<Vec<f64>>>(py) {
            return Ok(_dijkstra(&mat, start_index, end_index, 0.).to_object(py));
        }
        Err(PyTypeError::new_err("Not supported type"))
    })
}

//fn _dijkstra<T: Copy + PartialOrd + std::ops::Add<Output=T>>(mat: &Vec<Vec<T>>, start_index: usize, end_index: usize, zero_weight: T) -> Vec<usize> {
fn _dijkstra<T: Copy + PartialOrd + std::ops::Add<Output=T> + std::fmt::Display>(mat: &Vec<Vec<T>>, start_index: usize, end_index: usize, zero_weight: T) -> Vec<usize> {
    assert!(mat.len() == mat[0].len());

    let mut d_v = Vec::with_capacity(mat.len());
    let mut prev_v = Vec::with_capacity(mat.len());
    let mut q = Vec::with_capacity(mat.len());
    for v in 0..mat.len() {
        if v == start_index {
            d_v.push(Some(zero_weight));
        }
        else {
            d_v.push(None);
        }
        prev_v.push(None);
        q.push(v);
    }

    while q.len() > 0 {
        let mut min_d_v = d_v[q[0]];
        let mut min_v = q[0];
        let mut min_v_index_of_q = 0;
        for (q_index, v) in q.iter().enumerate() {
            if let Some(d_v) = d_v[*v] {
                if let Some(this_min_d_v) = min_d_v {
                    if d_v < this_min_d_v {
                        min_v = *v;
                        min_d_v = Some(d_v);
                        min_v_index_of_q = q_index;
                    }
                }
                else {
                    min_v = *v;
                    min_d_v = Some(d_v);
                    min_v_index_of_q = q_index;
                }
            }
        }
        if min_v == end_index {
            break;
        }
        q.remove(min_v_index_of_q);
        let min_d_v = min_d_v.unwrap();
        println!("q: {:?}", q);
        println!("min_v: {}, min_d_v: {}", min_v, min_d_v);

        for (v, weight) in mat[min_v].iter().enumerate() {
            if *weight == zero_weight {
                continue;
            }
            if let Some(this_d_v) = d_v[v] {
                if this_d_v > min_d_v + *weight {
                    d_v[v] = Some(min_d_v + *weight);
                    prev_v[v] = Some(min_v);
                }
            }
            else {
                d_v[v] = Some(min_d_v + *weight);
                prev_v[v] = Some(min_v);
                println!("updated d_v");
            }
        }
    }

    let mut result = Vec::new();
    let mut index = end_index;
    result.push(index);
    while index != start_index {
        index = prev_v[index].unwrap();
        result.push(index);
    }
    result.reverse();
    result
}
