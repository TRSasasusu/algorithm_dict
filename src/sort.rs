use std::collections::VecDeque;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

fn _call_each_type_sort(fi: fn(&mut [i64]), ff: fn(&mut [f64]), fu8: fn(&mut [u8]), vec: PyObject) -> PyResult<PyObject> {
    Python::with_gil(|py| {
        if let Ok(mut vec) = vec.extract::<Vec<i64>>(py) {
            fi(&mut vec);
            return Ok(vec.to_object(py));
        }
        else if let Ok(mut vec) = vec.extract::<Vec<f64>>(py) {
            ff(&mut vec);
            return Ok(vec.to_object(py));
        }
        else if let Ok(vec) = vec.extract::<String>(py) {
            let mut vec = vec.into_bytes();
            fu8(&mut vec);
            return Ok(String::from_utf8(vec).unwrap().to_object(py));
        }
        Err(PyTypeError::new_err("Not supported"))
    })
}

#[pyfunction]
pub fn bubble_sort(vec: PyObject) -> PyResult<PyObject> {
    _call_each_type_sort(_bubble_sort::<i64>, _bubble_sort::<f64>, _bubble_sort::<u8>, vec)
}

fn _bubble_sort<T: Copy + PartialOrd>(vec: &mut [T]) {
    for i in 0..vec.len()-1 {
        for j in i..vec.len() {
            if vec[i] > vec[j] {
                let tmp = vec[i];
                vec[i] = vec[j];
                vec[j] = tmp;
            }
        }
    }
}

#[pyfunction]
pub fn selection_sort(vec: PyObject) -> PyResult<PyObject> {
    _call_each_type_sort(_selection_sort::<i64>, _selection_sort::<f64>, _selection_sort::<u8>, vec)
}

pub fn _selection_sort<T: Copy + PartialOrd>(vec: &mut [T]) {
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
}

#[pyfunction]
pub fn merge_sort(vec: PyObject) -> PyResult<PyObject> {
    _call_each_type_sort(_merge_sort::<i64>, _merge_sort::<f64>, _merge_sort::<u8>, vec)
}

pub fn _merge_sort<T: Copy + PartialOrd>(vec: &mut [T]) {
    let mut size = 1;
    loop {
        let mut end_loop = false;
        size *= 2;
        if size >= vec.len() {
            end_loop = true;
        }
        for i in (0..vec.len()).step_by(size) {
            if i + size/2 >= vec.len() {
                continue;
            }
            let mut small_vec = Vec::with_capacity(size);
            let mut right_limit = size/2;
            if i + size >= vec.len() {
                right_limit = vec.len() - i - size/2;
            }

            let mut left_index = 0;
            let mut right_index = 0;
            loop {
                if left_index >= size/2 {
                    if right_index >= right_limit {
                        break;
                    }
                    small_vec.push(vec[i+size/2+right_index]);
                    right_index += 1;
                }
                else if right_index >= right_limit {
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
                if i + j >= vec.len() - 1 {
                    break;
                }
            }
        }
        if end_loop {
            break;
        }
    }
}

#[pyfunction]
pub fn quick_sort(vec: PyObject) -> PyResult<PyObject> {
    _call_each_type_sort(_quick_sort::<i64>, _quick_sort::<f64>, _quick_sort::<u8>, vec)
}

pub fn _quick_sort<T: Copy + PartialOrd>(vec: &mut [T]) {
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
        //println!("from {} to {}: {:?}", range.begin, range.end, vec);
    }
}
