use std::cmp::min;

const RUN: usize = 32;

/// Sort an array using tim sort
///
/// # Parameters
///
/// - `arr`: A vector to sort in-place
///
/// # Type parameters
///
/// - `T`: A type that can be checked for equality and ordering e.g. a `i32`, a
///     `u8`, or a `f32`.
///
/// # Undefined Behavior
/// 
/// Does not work with `String` vectors.
/// 
/// # Examples
///
/// ```rust
/// use algorithmplus::sort::tim_sort;
/// 
/// let mut ls = vec![3, 2, 1];
/// tim_sort(&mut ls);
/// 
/// assert_eq!(ls, [1, 2, 3]);
/// ```

pub fn tim_sort<T: PartialEq + PartialOrd + Copy>(arr: &mut [T]) {
    let arr_len = arr.len();

    for i in (0..arr_len).step_by(RUN) {
        insertion_sort(arr, i as isize, min(i + 31, arr_len - 1) as isize);
    }

    let mut size = RUN;

    while size < arr_len {
        let mut left = 0;

        while left < arr_len {
            let mid = left + size - 1;
            let right = min(left + 2 * size - 1, arr_len - 1);
            merge(arr, left as isize, mid as isize, right as isize);
            left += 2 * size;
        }
        size *= 2;
    }
}

fn insertion_sort<T: PartialEq + PartialOrd + Copy>(arr: &mut [T], left: isize, right: isize) {
    for i in left + 1..right + 1 {
        let temp = arr[i as usize];
        let mut j = i - 1;

        while j >= left && arr[j as usize] > temp  {
            arr[(j + 1) as usize] = arr[j as usize];
            j -= 1;
        }
        arr[(j + 1) as usize] = temp;
    }
}

fn merge<T: PartialEq + PartialOrd + Copy>(arr: &mut [T], left: isize, mid: isize, right: isize) {
    let len_one = mid - left + 1;
    let len_two = right - mid;
    let mut left_arr: Vec<T> = Vec::with_capacity(len_one as usize);
    let mut right_arr: Vec<T> = Vec::with_capacity(len_two as usize);

    for i in 0..len_one {
        left_arr.push(arr[(left + i) as usize]);
    }

    for i in 0..len_two {
        right_arr.push(arr[(mid + i + 1) as usize]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < len_one && j < len_two {
        if left_arr[i as usize] <= right_arr[j as usize] {
            arr[k as usize] = left_arr[i as usize];
            i += 1;
        }
        else {
            arr[k as usize] = right_arr[j as usize];
            j += 1;
        }
        k += 1;
    }

    while i < len_one {
        arr[k as usize] = left_arr[i as usize];
        k += 1;
        i += 1;
    }

    while j < len_two {
        arr[k as usize] = right_arr[j as usize];
        k += 1;
        j += 1;
    }
}
