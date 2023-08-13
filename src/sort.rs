use std::collections::VecDeque;
use pyo3::exceptions::PyTypeError;
use pyo3::prelude::*;

//fn _call_general_sort<F: Fn<T: Copy + PartialOrd>>(f: F, vec: PyObject) -> PyResult<PyObject> {
//fn _call_general_sort<F>(f: F, vec: PyObject) where F: Fn(fn()) -> PyResult<PyObject> {
//_call_each_type_sort<Fi: Fn(Vec<i64>), Ff: Fn(Vec<f64>), Fu8: Fn(<Vec<u8>>)>(fi: Fi, ff: Ff, fu8: Fu8) -> PyResult<PyObject> {
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
            //return Ok(String::from_utf8(fu8(vec.into_bytes())).unwrap().to_object(py));
        }
        Err(PyTypeError::new_err("Not supported"))
    })
}

#[pyfunction]
pub fn bubble_sort(vec: PyObject) -> PyResult<PyObject> {
    _call_each_type_sort(_bubble_sort::<i64>, _bubble_sort::<f64>, _bubble_sort::<u8>, vec)
}

//fn _bubble_sort<T: Copy + PartialOrd>(mut vec: Vec<T>) -> Vec<T> {
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
//pub fn selection_sort(vec: PyObject) -> PyResult<PyObject> {
//    Python::with_gil(|py| {
//        _call_general_sort(_bubble_sort)
//    })
//}
//
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
