/// Search for an element in a sorted array using binary search
///
/// # Parameters
///
/// - `target`: Reference to the element to find
/// - `arr`: Reference to the vector to search the element in
///
/// # Type parameters
///
/// - `T`: A type that can be checked for equality and ordering e.g. a `i32`, a
///     `u8`, or a `f32`.
///
/// # Examples
///
/// ```rust
/// use algorithmplus::search::binary_search;
/// 
/// let ls = vec![1, 7, 9, 11, 12];
/// let idx = binary_search(&7, &ls).unwrap_or_default();
/// 
/// assert_eq!(idx, 1);
/// ```
/// 
/// ```rust
/// use algorithmplus::search::binary_search;
/// 
/// let ls = vec![1, 7, 9, 11, 12];
/// let idx = binary_search(&8, &ls);
/// 
/// assert_eq!(idx, None);
/// ```

pub fn binary_search<T: PartialEq + PartialOrd>(target: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let arr_len = arr.len();

    if arr_len == 0 {
        return None;
    }

    let mut right = arr_len - 1;

    if &arr[left] > target || &arr[right] < target {
        return None;
    }

    while left <= right {
        let mid = left + (right - left) / 2;

        if &arr[mid] > target {
            right = mid - 1;
        } else if &arr[mid] < target {
            left = mid + 1;
        } else {
            return Some(mid);
        }
    }
    None
}
