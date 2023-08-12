use std::collections::VecDeque;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

#[pyfunction]
pub fn bubble_sort(mut vec: PyObject) -> PyResult<PyObject> {
    let gil = Python::acquire_gil();
    let py = gil.python();
    if let Ok(vec) = vec.extract::<Vec<i64>>(py) {
        return Ok(_bubble_sort(vec).to_object(py));
    }
    else if let Ok(vec) = vec.extract::<Vec<f64>>(py) {
        return Ok(_bubble_sort(vec).to_object(py));
    }
    Err(PyTypeError::new_err("Not supported"))
//    let vec: Vec<i64> = vec.extract()?;
//    Ok(PyCell::new(py, _bubble_sort(vec)).unwrap())
//    if let Ok(i_vec) = vec.extract() {
//    //if let Ok(i_vec) = vec.cast_as::<Vec<i64>>() {
//        return _bubble_sort(i_vec);
//    }
//    else if let Ok(f_vec) = vec.cast_as::<Vec<f64>>() {
//        return _bubble_sort(f_vec);
//    }
//    return TypeError::into("hgoe");
}

//#[pyfunction]
pub fn _bubble_sort<T: Copy + PartialOrd>(mut vec: Vec<T>) -> Vec<T> {
//pub fn bubble_sort(mut vec: Vec<i64>) -> PyResult<Vec<i64>> {
    for i in 0..vec.len()-1 {
        for j in i..vec.len() {
            if vec[i] > vec[j] {
                let tmp = vec[i];
                vec[i] = vec[j];
                vec[j] = tmp;
            }
        }
    }

    vec
}

#[pyfunction]
pub fn selection_sort(mut vec: Vec<i64>) -> PyResult<Vec<i64>> {
    for i in 0..vec.len()-1 {
        let mut min_index: usize = i;
        for j in i+1..vec.len() {
            if vec[j] < vec[min_index] {
                min_index = j;
            }
        }
        let tmp = vec[i];
        vec[i] = vec[min_index];
        vec[min_index] = tmp;
    }

    Ok(vec)
}

#[pyfunction]
pub fn merge_sort(mut vec: Vec<i64>) -> PyResult<Vec<i64>> {
    let original_len = vec.len();
    let mut exp_len = 1;
    loop {
        if vec.len() < exp_len {
            while vec.len() < exp_len {
                vec.push(i64::MAX);
            }
            break;
        }
        else if vec.len() == exp_len {
            break;
        }
        exp_len *= 2;
    }

    let mut size = 1;
    loop {
        size *= 2;
        if size > vec.len() {
            break;
        }
        for i in (0..vec.len()).step_by(size) {
            let mut small_vec = Vec::new();
            let mut left_index = 0;
            let mut right_index = 0;
            loop {
                if left_index >= size/2 {
                    if right_index >= size/2 {
                        break;
                    }
                    small_vec.push(vec[i+size/2+right_index]);
                    right_index += 1;
                }
                else if right_index >= size/2 {
                    small_vec.push(vec[i+left_index]);
                    left_index += 1;
                }
                else if vec[i+left_index] < vec[i+size/2+right_index] {
                    small_vec.push(vec[i+left_index]);
                    left_index += 1;
                }
                else {
                    small_vec.push(vec[i+size/2+right_index]);
                    right_index += 1;
                }
            }
            for j in 0..size {
                vec[i+j] = small_vec[j];
            }
        }
    }

    while vec.len() > original_len {
        vec.pop();
    }

    Ok(vec)
}

#[pyfunction]
pub fn quick_sort(mut vec: Vec<i64>) -> PyResult<Vec<i64>> {
    struct BeginEnd {
        begin: usize,
        end: usize,
    }
    let mut queue = VecDeque::new();
    queue.push_front(BeginEnd { begin: 0, end: vec.len() });
    while queue.len() > 0 {
        let range = queue.pop_back().unwrap();
        let pivot_index = (range.begin+range.end) / 2;
        let pivot = vec[pivot_index];
        let mut left = range.begin;
        let mut right = range.end - 1;
        loop {
            let mut goto_next_queue = false;
            if vec[left] >= pivot {
                loop {
                    if vec[right] <= pivot {
                        let tmp = vec[left];
                        vec[left] = vec[right];
                        vec[right] = tmp;
                        break;
                    }
                    right -= 1;
                    if right <= left {
                        goto_next_queue = true;
                        break;
                    }
                }
            }
            if goto_next_queue {
                break;
            }

            left += 1;
            if right < left {
                break;
            }
        }

        if right - range.begin >= 2 {
            queue.push_front(BeginEnd { begin: range.begin, end: right });
        }
        if range.end - right >= 2 {
            queue.push_front(BeginEnd { begin: right, end: range.end });
        }
        println!("from {} to {}: {:?}", range.begin, range.end, vec);
    }

    Ok(vec)
}
