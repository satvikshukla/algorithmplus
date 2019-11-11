/// Sort an array using quick sort
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
/// use algorithmplus::sort::quick_sort;
/// 
/// let mut ls = vec![3, 2, 1];
/// quick_sort(&mut ls);
/// 
/// assert_eq!(ls, [1, 2, 3]);
/// ```

pub fn quick_sort<T: PartialEq + PartialOrd + Copy> (arr: &mut [T]) {
    _quick_sort(arr, 0, arr.len() as isize - 1);
}

fn _quick_sort<T: PartialEq + PartialOrd + Copy> (arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let pi = partition(arr, low, high);

        _quick_sort(arr, low, pi - 1);
        _quick_sort(arr, pi + 1, high);
    }
}

fn partition<T: PartialEq + PartialOrd + Copy> (arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = arr[high as usize];
    let mut i = low - 1;

    for j in low..high {
        if arr[j as usize] < pivot {
            i += 1;
            arr.swap(i as usize, j as usize);
        }
    }

    arr.swap((i + 1) as usize, high as usize);
    i + 1
}
