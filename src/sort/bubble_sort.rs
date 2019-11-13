/// Sort an array using bubble sort
///
/// # Parameters
///
/// - `arr`: Mutable reference to the vector to sort in-place
///
/// # Type parameters
///
/// - `T`: A type that can be checked for equality and ordering e.g. a `i32`, a
///     `u8`, or a `f32`.
///
/// # Examples
///
/// ```rust
/// use algorithmplus::sort::bubble_sort;
/// 
/// let mut ls = vec![3, 2, 1];
/// bubble_sort(&mut ls);
/// 
/// assert_eq!(ls, [1, 2, 3]);
/// ```

pub fn bubble_sort<T: PartialEq + PartialOrd>(arr: &mut [T]) {
    let arr_len = arr.len();

    for i in 0..arr_len {
        for j in 0..arr_len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}
