use std::cmp::{PartialEq, PartialOrd};

pub fn binary_search<T: PartialEq + PartialOrd>(target: &T, arr: &[T]) -> Option<usize> {
    let mut left = 0;
    let arr_len = arr.len();

    if arr_len == 0 {
        return None
    }

    let mut right = arr_len - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        println!("{} {} {}", left, right, mid);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = binary_search(&1, &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn search_strings() {
        let index = binary_search(&"a", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(0));

        let index = binary_search(&"d", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(3));
    }

    #[test]
    fn search_ints() {
        let index = binary_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = binary_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = binary_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = binary_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = binary_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
