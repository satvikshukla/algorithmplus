/// Search for an element in an array using linear search
///
/// # Parameters
///
/// - `target`: The element to find
/// - `arr`: A vector to search the element in
///
/// # Type parameters
///
/// - `T`: A type that can be checked for equality and ordering e.g. a `i32`, a
///     `u8`, or a `f32`.
///
/// # Examples
///
/// ```rust
/// use algorithmplus::search::linear_search;
/// 
/// let ls = vec![7, 1, -1, 10, 12];
/// let idx = linear_search(&10, &ls).unwrap_or_default();
/// 
/// assert_eq!(idx, 3);
/// ```
/// 
/// ```rust
/// use algorithmplus::search::linear_search;
/// 
/// let ls = vec![4, 1, 2, 11, -7];
/// let idx = linear_search(&8, &ls);
/// 
/// assert_eq!(idx, None);
/// ```

pub fn linear_search<T: PartialEq + PartialOrd>(target: &T, arr: &[T]) -> Option<usize> {
    let arr_len = arr.len();

    if arr_len == 0 {
        return None;
    }

    for (idx, value) in arr.iter().enumerate() {
        if value == target {
            return Some(idx);
        }
    }
    None
}
