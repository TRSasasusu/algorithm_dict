use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyfunction]
pub fn topological_sort(mat: PyObject) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        if let Ok(mut mat) = mat.extract::<Vec<Vec<i64>>>(py) {
            return Ok(_topological_sort(&mut mat, 0).to_object(py));
        }
        else if let Ok(mut mat) = mat.extract::<Vec<Vec<f64>>>(py) {
            return Ok(_topological_sort(&mut mat, 0.).to_object(py));
        }
        Err(PyTypeError::new_err("Not supported type"))
    })
}

fn _topological_sort<T: Copy + PartialOrd>(mat: &mut Vec<Vec<T>>, zero_weight: T) -> Vec<usize> {
    assert!(mat.len() == mat[0].len());

    // find nodes without input edges
    let mut s = Vec::new();
    for v in 0..mat.len() {
        let mut has_input = false;
        for i in 0..mat.len() {
            if mat[i][v] != zero_weight {
                has_input = true;
                break;
            }
        }
        if !has_input {
            s.push(v);
        }
    }
    s.reverse();

    let mut l = Vec::new();
    while s.len() > 0 {
        let n = s.pop().unwrap();
        l.push(n);
        for m in 0..mat.len() {
            if mat[n][m] != zero_weight {
                mat[n][m] = zero_weight;

                let mut has_input = false;
                for i in 0..mat.len() {
                    if mat[i][m] != zero_weight {
                        has_input = true;
                        break;
                    }
                }
                if !has_input {
                    s.push(m);
                }
            }
        }
    }

    l
}
