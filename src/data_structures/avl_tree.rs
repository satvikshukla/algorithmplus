use std::cmp::max;

struct AVLNode<K: PartialEq + PartialOrd + Default, V:Default> {
    left: Option<Box<AVLNode<K, V>>>,
    right: Option<Box<AVLNode<K, V>>>,
    key: K,
    value: V,
    height: u16
}

impl<K: PartialEq + PartialOrd + Default, V:Default> AVLNode<K, V> {
    fn new(k: K) -> AVLNode<K, V> {
        AVLNode {left: None, right: None, key: k, value: Default::default(), height: 0}
    }

    fn from_val(k: K, v: V) -> AVLNode<K, V> {
        AVLNode {left: None, right: None, key: k, value: v, height: 0}
    }
}

pub struct AVLTree<K: PartialEq + PartialOrd + Default, V:Default> {
    root: Option<Box<AVLNode<K, V>>>
}

impl<K: PartialEq + PartialOrd + Default, V:Default> AVLTree<K, V> {
    fn new() -> AVLTree<K, V> {
        AVLTree {root: None}
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn make_empty(&mut self) {
        self.root = None;
    }

    fn height(node: Option<Box<AVLNode<K, V>>>) -> i32 {
        match node {
            None => -1,
            Some(val) => (*val).height as i32
        }
    }
}
