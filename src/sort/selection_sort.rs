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
