/// Sort an array using selection sort
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
/// # Examples
///
/// ```rust
/// use algorithmplus::sort::selection_sort;
/// 
/// let mut ls = vec![3, 2, 1];
/// selection_sort(&mut ls);
/// 
/// assert_eq!(ls, [1, 2, 3]);
/// ```

pub fn selection_sort<T: PartialEq + PartialOrd>(arr: &mut [T]) {
    let arr_len = arr.len();

    for i in 0..arr_len {
        let mut smallest = i;
        for j in (i + 1)..arr_len {
            if arr[j] < arr[smallest] {
                smallest = j;
            }
        }
        arr.swap(smallest, i);
    }
}
