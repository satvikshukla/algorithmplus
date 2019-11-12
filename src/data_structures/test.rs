mod test {
    use crate::data_structures::Trie;

    #[test]
    fn test_empty_trie() {
        let trie = Trie::new();
        assert_eq!(trie.starts_with(String::from(" ")), false);
        assert_eq!(trie.starts_with(String::from("a")), false);
    }

    #[test]
    #[should_panic]
    fn test_invalid_input_one() {
        let mut trie = Trie::new();
        trie.insert(String::from("1"));
    }

    #[test]
    #[should_panic]
    fn test_invalid_input_two() {
        let mut trie = Trie::new();
        trie.insert(String::from("abcd,e"));
    }

    #[test]
    fn test_filled_trie() {
        let mut trie = Trie::new();
        trie.insert(String::from("a"));
        assert_eq!(trie.search(String::from("a")), true);
        assert_eq!(trie.starts_with(String::from("a")), true);
        assert_eq!(trie.search(String::from("c")), false);
        assert_eq!(trie.starts_with(String::from("c")), false);
        trie.insert(String::from("b"));
        assert_eq!(trie.search(String::from("b")), true);
        assert_eq!(trie.starts_with(String::from("b")), true);
        trie.insert(String::from("ab"));
        assert_eq!(trie.search(String::from("ab")), true);
        assert_eq!(trie.starts_with(String::from("ab")), true);
        assert_eq!(trie.search(String::from("ac")), false);
        assert_eq!(trie.starts_with(String::from("ac")), false);
        assert_eq!(trie.search(String::from("abc")), false);
        assert_eq!(trie.starts_with(String::from("abc")), false);
    }
}
