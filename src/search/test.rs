mod test {
    use crate::search::{binary_search, linear_search};

    fn test_empty<F: Fn(&u8, &[u8]) -> Option<usize>>(f: F) {
        let index = f(&1, &vec![]);
        assert_eq!(index, None);
    }

    fn test_string_search_sorted<F: Fn(&String, &[String]) -> Option<usize>>(f: F) {
        let arr = vec![String::from("a"), String::from("aa"), String::from("abc")];

        let index = f(&String::from("a"), &arr);
        assert_eq!(index, Some(0));

        let index = f(&String::from("aa"), &arr);
        assert_eq!(index, Some(1));

        let index = f(&String::from("abc"), &arr);
        assert_eq!(index, Some(2));

        let index = f(&String::from("z"), &arr);
        assert_eq!(index, None);
    }

    fn test_int_search_sorted<F: Fn(&i32, &[i32]) -> Option<usize>>(f: F) {
        let arr = vec![-1, 1, 3, 6, 9];
        let index = f(&-1, &arr);
        assert_eq!(index, Some(0));

        let index = f(&1, &arr);
        assert_eq!(index, Some(1));

        let index = f(&3, &arr);
        assert_eq!(index, Some(2));

        let index = f(&6, &arr);
        assert_eq!(index, Some(3));

        let index = f(&9, &arr);
        assert_eq!(index, Some(4));

        let index = f(&10, &arr);
        assert_eq!(index, None);
    }

    fn test_float_search_sorted<F: Fn(&f32, &[f32]) -> Option<usize>>(f: F) {
        let arr = vec![-1.1, -1.0, 2.1, 3.9102, 4.6];

        let index = f(&-1.1, &arr);
        assert_eq!(index, Some(0));

        let index = f(&-1.0, &arr);
        assert_eq!(index, Some(1));

        let index = f(&2.1, &arr);
        assert_eq!(index, Some(2));

        let index = f(&3.9102, &arr);
        assert_eq!(index, Some(3));

        let index = f(&4.6, &arr);
        assert_eq!(index, Some(4));

        let index = f(&-1.2, &arr);
        assert_eq!(index, None);

        let index = f(&10.2, &arr);
        assert_eq!(index, None);
    }

    fn test_string_search_unsorted<F: Fn(&String, &[String]) -> Option<usize>>(f: F) {
        let arr = vec![String::from("ab"), String::from("az"), String::from("a")];

        let index = f(&String::from("ab"), &arr);
        assert_eq!(index, Some(0));

        let index = f(&String::from("az"), &arr);
        assert_eq!(index, Some(1));

        let index = f(&String::from("a"), &arr);
        assert_eq!(index, Some(2));

        let index = f(&String::from("z"), &arr);
        assert_eq!(index, None);
    }

    fn test_int_search_unsorted<F: Fn(&i32, &[i32]) -> Option<usize>>(f: F) {
        let arr = vec![1, 4, 3, 46, -9];
        let index = f(&1, &arr);
        assert_eq!(index, Some(0));

        let index = f(&4, &arr);
        assert_eq!(index, Some(1));

        let index = f(&3, &arr);
        assert_eq!(index, Some(2));

        let index = f(&46, &arr);
        assert_eq!(index, Some(3));

        let index = f(&-9, &arr);
        assert_eq!(index, Some(4));

        let index = f(&10, &arr);
        assert_eq!(index, None);
    }

    fn test_float_search_unsorted<F: Fn(&f32, &[f32]) -> Option<usize>>(f: F) {
        let arr = vec![1.1, 1.01, -2.1, -3.2, 4.6];

        let index = f(&1.1, &arr);
        assert_eq!(index, Some(0));

        let index = f(&1.01, &arr);
        assert_eq!(index, Some(1));

        let index = f(&-2.1, &arr);
        assert_eq!(index, Some(2));

        let index = f(&-3.2, &arr);
        assert_eq!(index, Some(3));

        let index = f(&4.6, &arr);
        assert_eq!(index, Some(4));

        let index = f(&-1.2, &arr);
        assert_eq!(index, None);
    }

    #[test]
    fn linear_search_empty() {
        test_empty(linear_search);
    }

    #[test]
    fn linear_search_int_search_sorted() {
        test_int_search_sorted(linear_search);
    }

    #[test]
    fn linear_search_float_search_sorted() {
        test_float_search_sorted(linear_search);
    }

    #[test]
    fn linear_search_string_search_sorted() {
        test_string_search_sorted(linear_search);
    }

    #[test]
    fn linear_search_int_search_unsorted() {
        test_int_search_unsorted(linear_search);
    }

    #[test]
    fn linear_search_float_search_unsorted() {
        test_float_search_unsorted(linear_search);
    }

    #[test]
    fn linear_search_string_search_unsorted() {
        test_string_search_unsorted(linear_search);
    }

    #[test]
    fn binary_search_empty() {
        test_empty(binary_search);
    }

    #[test]
    fn binary_search_int_search_sorted() {
        test_int_search_sorted(binary_search);
    }

    #[test]
    fn binary_search_float_search_sorted() {
        test_float_search_sorted(binary_search);
    }

    #[test]
    fn binary_search_string_search_sorted() {
        test_string_search_sorted(binary_search);
    }
}
