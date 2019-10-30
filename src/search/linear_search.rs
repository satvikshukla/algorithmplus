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
