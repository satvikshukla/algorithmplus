/// Sort an array using merge sort
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
/// use algorithmplus::sort::merge_sort;
/// 
/// let mut ls = vec![3, 2, 1];
/// merge_sort(&mut ls);
/// 
/// assert_eq!(ls, [1, 2, 3]);
/// ```

pub fn merge_sort<T: PartialEq + PartialOrd + Copy>(arr: &mut [T]) {
    let mid = arr.len() / 2;

    if mid > 0 {
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);

        let mut ret = arr.to_vec();
        merge(&arr[..mid], &arr[mid..], &mut ret[..]);
        arr.copy_from_slice(&ret);
    }
}

fn merge<T: PartialEq + PartialOrd + Copy>(arr_one: &[T], arr_two: &[T], result_arr: &mut [T]) {
    let mut left = 0;
    let mut right = 0;
    let mut index = 0;
    let arr_one_len = arr_one.len();
    let arr_two_len = arr_two.len();

    while left < arr_one_len && right < arr_two_len {
        if arr_one[left] <= arr_two[right] {
            result_arr[index] = arr_one[left];
            index += 1;
            left += 1;
        } else {
            result_arr[index] = arr_two[right];
            index += 1;
            right += 1;
        }
    }

    if left < arr_one_len {
        for idx in left..arr_one_len {
            result_arr[index] = arr_one[idx];
            index += 1;
        }
    }

    if right < arr_two_len {
        for idx in right..arr_two_len {
            result_arr[index] = arr_two[idx];
            index += 1;
        }
    }
}
