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
