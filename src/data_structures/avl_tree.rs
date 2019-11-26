use std::cmp::{self, max, Ordering};
use std::fmt::Display;
use std::ops::Deref;

struct AVLNode<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display> {
    left: Option<Box<AVLNode<K, V>>>,
    right: Option<Box<AVLNode<K, V>>>,
    key: K,
    value: V,
    height: i32,
}

impl<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display> AVLNode<K, V> {
    fn new(k: K, v: V) -> AVLNode<K, V> {
        AVLNode {
            left: None,
            right: None,
            key: k,
            value: v,
            height: 0,
        }
    }
}

fn height<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &Option<Box<AVLNode<K, V>>>) -> i32 {
    match node {
        None => -1,
        Some(val) => val.height,
    }
}

fn rotate_with_left_child<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &mut Box<AVLNode<K, V>>) -> Box<AVLNode<K, V>> {
    let mut left_child = node.left.take().expect("AVL tree broken");
    node.left = left_child.right;
    node.height = max(height(&node.left), height(&node.right)) + 1;
    left_child.height = max(height(&left_child.left), node.height) + 1;
    left_child.right = Some(*node);
    left_child
}

fn rotate_with_right_child<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &mut Box<AVLNode<K, V>>) -> Box<AVLNode<K, V>> {
    let mut right_child = node.right.take().expect("AVL tree broken");
    node.right = right_child.left;
    node.height = max(height(&node.left), height(&node.right)) + 1;
    right_child.height = max(height(&right_child.right), node.height) + 1;
    right_child.left = Some(*node);
    right_child
}

fn double_with_left_child<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &mut Box<AVLNode<K, V>>) -> Box<AVLNode<K, V>> {
    let left_child = node.left.take().expect("AVL tree broken");
    node.left = Some(rotate_with_right_child(&mut left_child));
    rotate_with_left_child(node)
}

fn double_with_right_child<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &mut Box<AVLNode<K, V>>) -> Box<AVLNode<K, V>> {
    let right_child = node.right.take().expect("AVL tree broken");
    node.right = Some(rotate_with_left_child(&mut right_child));
    rotate_with_right_child(node)
}

fn insert<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &mut Option<Box<AVLNode<K, V>>>, key: &K, value: &V) -> Box<AVLNode<K, V>> {
    match node {
        None => Box::new(AVLNode::new(*key, *value)),
        Some(n) => {
            let node_val = n.key;
            if key == &node_val {
                n.value = *value;
            }
            else if key < &node_val {
                n.left = Some(insert(&mut n.left, key, value));
                if height(&n.left) - height(&n.right) == 1 {
                    if key < &n.left.unwrap().key {
                        node = &mut Some(rotate_with_left_child(&mut n));
                    }
                    else {
                        node = &mut Some(double_with_left_child(&mut n));
                    }
                }
            }
            else {

            }
            n
        }
    }
}

fn count_nodes<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &Option<Box<AVLNode<K, V>>>) -> i32 {
    match node {
        None => -1,
        Some(val) => {
            let mut num_nodes = 1;
            num_nodes += count_nodes(&val.left);
            num_nodes += count_nodes(&val.right);
            num_nodes
        }
    }
}

fn search<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &Option<Box<AVLNode<K, V>>>, key: &K) -> Option<V> {
    while node.is_some() {
        let k = node.as_ref().unwrap().key;
        if key == &k {
            return Some(node.as_ref().unwrap().value);
        }
        else if key < &k {
            search(&node.as_ref().unwrap().left, key);
        }
        else {
            search(&node.as_ref().unwrap().right, key);
        }
    }
    None
}

fn pre_order<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &Option<Box<AVLNode<K, V>>>) {
    if node.is_some() {
        println!("{}->{}", node.as_ref().unwrap().key, node.as_ref().unwrap().value);
        pre_order(&node.as_ref().unwrap().left);
        pre_order(&node.as_ref().unwrap().right);
    }
}

fn in_order<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &Option<Box<AVLNode<K, V>>>) {
    if node.is_some() {
        in_order(&node.as_ref().unwrap().left);
        println!("{}->{}", node.as_ref().unwrap().key, node.as_ref().unwrap().value);
        in_order(&node.as_ref().unwrap().right);
    }
}

fn post_order<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display>(node: &Option<Box<AVLNode<K, V>>>) {
    if node.is_some() {
        post_order(&node.as_ref().unwrap().left);
        post_order(&node.as_ref().unwrap().right);
        println!("{}->{}", node.as_ref().unwrap().key, node.as_ref().unwrap().value);
    }
}

pub struct AVLTree<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display> {
    root: Option<Box<AVLNode<K, V>>>,
}

impl<K: PartialEq + PartialOrd + Copy + Display, V: Default + Copy + Display> AVLTree<K, V> {
    fn new() -> AVLTree<K, V> {
        AVLTree { root: None }
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn make_empty(&mut self) {
        self.root = None;
    }
}
