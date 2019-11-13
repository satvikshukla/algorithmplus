use std::collections::BinaryHeap;

/// Sort an array using heap sort
///
/// # Parameters
///
/// - `arr`: Mutable reference to the vector to sort in-place
///
/// # Type parameters
///
/// - `T`: A type that can be checked for equality and ordering e.g. a `i32`, a
///     `u8`, or a `String`.
///
/// # Undefined Behavior
/// 
/// Does not work with `f32` and `f64` vectors.
/// 
/// # Examples
///
/// ```rust
/// use algorithmplus::sort::heap_sort;
/// 
/// let mut ls = vec![3, 2, 1];
/// heap_sort(&mut ls);
/// 
/// assert_eq!(ls, [1, 2, 3]);
/// ```

pub fn heap_sort<T: Ord + Clone>(arr: &mut [T]) {
    let arr_len = arr.len();
    let mut heap = BinaryHeap::from(arr.to_vec());

    for i in 0..arr_len {
        arr[arr_len - i - 1] = heap.pop().unwrap();
    }
}
