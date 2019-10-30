mod test {
    use crate::sort::{bubble_sort, selection_sort};

    fn test_empty_arr<F: Fn(&mut [u8])>(f: F) {
        let mut arr = vec![];
        f(&mut arr);
        let expected: Vec<u8> = vec![];
        assert_eq!(arr, expected);
    }

    fn test_one_element_arr<F: Fn(&mut [u8])>(f: F) {
        let mut arr = vec![2];
        f(&mut arr);
        assert_eq!(arr, vec![2]);
    }

    fn test_pre_sorted_arr<F: Fn(&mut [i32])>(f: F) {
        let mut arr = vec![-4, 9, 11];
        f(&mut arr);
        assert_eq!(arr, vec![-4, 9, 11]);
    }

    fn test_int_arr<F: Fn(&mut [i32])>(f: F) {
        let mut arr = vec![5, -9, 4, 1, 3, 3];
        f(&mut arr);
        assert_eq!(arr, vec![-9, 1, 3, 3, 4, 5]);
    }

    fn test_float_arr<F: Fn(&mut [f32])>(f: F) {
        let mut arr = vec![-1.1, -7.5, -0.1, 1.1, 1.11];
        f(&mut arr);
        assert_eq!(arr, vec![-7.5, -1.1, -0.1, 1.1, 1.11]);
    }

    fn test_string_arr<F: Fn(&mut [String])>(f: F) {
        let mut arr = vec![String::from("abc"), String::from("aa"), String::from("a"), String::from("z"), String::from("abb")];
        f(&mut arr);
        assert_eq!(arr, vec!["a", "aa", "abb", "abc", "z"]);
    }

    #[test]
    fn selection_sort_empty() {
        test_empty_arr(selection_sort);
    }

    #[test]
    fn selection_sort_one_element() {
        test_one_element_arr(selection_sort);
    }

    #[test]
    fn selection_sort_pre_sorted() {
        test_pre_sorted_arr(selection_sort);
    }

    #[test]
    fn selection_sort_int_sort() {
        test_int_arr(selection_sort);
    }

    #[test]
    fn selection_sort_float_sort() {
        test_float_arr(selection_sort);
    }

    #[test]
    fn selection_sort_string_sort() {
        test_string_arr(selection_sort);
    }

    #[test]
    fn bubble_sort_empty() {
        test_empty_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_one_element() {
        test_one_element_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_pre_sorted() {
        test_pre_sorted_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_int_sort() {
        test_int_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_float_sort() {
        test_float_arr(bubble_sort);
    }

    #[test]
    fn bubble_sort_string_sort() {
        test_string_arr(bubble_sort);
    }
}
