use std::cmp::max;

type Link<K, V> = Option<Box<AVLNode<K, V>>>;

#[derive(Clone)]
struct AVLNode<K: PartialEq + PartialOrd + Default, V:Default> {
    left: Link<K, V>,
    right: Link<K, V>,
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
    root: Link<K, V>
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

    fn height(node: &Link<K, V>) -> i32 {
        match node {
            None => -1,
            Some(val) => (*val).height as i32
        }
    }

    // fn insert(key: K, node: AVLNode<K, V>) {

    // }

    fn rotate_with_left_child(node_link: &mut Link<K, V>) {
        let mut left: Link<K, V> = None;
        match node_link {
            None => (),
            Some(box_val) => {
                left = (*node_link.unwrap()).left;
            }
        }
    }
}
