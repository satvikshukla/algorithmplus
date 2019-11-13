#[derive(Clone)]
struct TrieNode {
    val: char,
    is_word: bool,
    children: Vec<TrieNode>
}

impl TrieNode {
    fn new() -> TrieNode {
        TrieNode {val: '#', is_word: false, children: Vec::with_capacity(26)}
    }

    fn from_char(c: char) -> TrieNode {
        let ls = vec![TrieNode::new(); 26];
        TrieNode {val: c, is_word: false, children: ls}
    }
}

/// Implementation of Trie data structure for lowercase letters
///
/// # Examples
///
/// ```rust
/// use algorithmplus::data_structures::Trie;
/// 
/// let mut trie = Trie::new();
/// trie.insert(String::from("abc"));
/// assert_eq!(trie.search(String::from("abc")), true);
/// assert_eq!(trie.starts_with(String::from("ab")), true);
/// ```

pub struct Trie {
    root: TrieNode
}

impl Trie {
    pub fn new() -> Trie {
        Trie {root: TrieNode::from_char(' ')}
    }

    pub fn insert(&mut self, word: String) {
        let mut node = &mut self.root;
        for c in word.chars() {
            if (c as usize) < 97 || (c as usize) > 122 {
                panic!("Found character in input string ({}) that is not between a-z", c);
            }
            if node.children[c as usize - 97].val == '#' {
                node.children[c as usize - 97] = TrieNode::from_char(c);
            }
            node = &mut node.children[c as usize - 97];
        }
        node.is_word = true;
    }

    pub fn search(&self, word: String) -> bool {
        let mut node = &self.root;
        for c in word.chars() {
            if (c as usize) < 97 || (c as usize) > 122 {
                return false;
            }
            if node.children[c as usize - 97].val == '#' {
                return false;
            }
            node = &node.children[c as usize - 97];
        }
        node.is_word
    }

    pub fn starts_with(&self, prefix: String) -> bool {
        let mut node = &self.root;
        for c in prefix.chars() {
            if (c as usize) < 97 || (c as usize) > 122 {
                return false;
            }
            if node.children[c as usize - 97].val == '#' {
                return false;
            }
            node = &node.children[c as usize - 97];
        }
        true
    }
}
