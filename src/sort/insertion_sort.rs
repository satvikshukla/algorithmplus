/// Sort an array using insertion sort
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
/// # Undefined Behavior
/// 
/// Does not work with `String` vectors.
/// 
/// # Examples
///
/// ```rust
/// use algorithmplus::sort::insertion_sort;
/// 
/// let mut ls = vec![3, 2, 1];
/// insertion_sort(&mut ls);
/// 
/// assert_eq!(ls, [1, 2, 3]);
/// ```

pub fn insertion_sort<T: PartialEq + PartialOrd + Copy>(arr: &mut [T]) {
    for i in 1..arr.len() {
        let mut j = i;
        while j > 0 && arr[j - 1] > arr[j] {
            arr.swap(j - 1, j);
            j -= 1;
        }
    }
}
