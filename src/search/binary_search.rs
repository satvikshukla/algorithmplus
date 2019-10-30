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
