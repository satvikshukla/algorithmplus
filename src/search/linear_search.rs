use std::cmp::{PartialEq, PartialOrd};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let index = linear_search(&1, &vec![]);
        assert_eq!(index, None);
    }

    #[test]
    fn search_strings() {
        let index = linear_search(&"a", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(0));

        let index = linear_search(&"d", &vec!["a", "b", "c", "d"]);
        assert_eq!(index, Some(3));
    }

    #[test]
    fn search_ints() {
        let index = linear_search(&4, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(3));

        let index = linear_search(&3, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(2));

        let index = linear_search(&2, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(1));

        let index = linear_search(&1, &vec![1, 2, 3, 4]);
        assert_eq!(index, Some(0));
    }

    #[test]
    fn not_found() {
        let index = linear_search(&5, &vec![1, 2, 3, 4]);
        assert_eq!(index, None);
    }
}
