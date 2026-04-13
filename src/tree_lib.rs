use std::collections::HashMap;
use crate::zip_lib::*;


pub struct Node<T, U> {
    pub left: Option<Box<Node<T, U>>>,
    pub right: Option<Box<Node<T, U>>>,
    pub key: Option<T>,
    pub value: U,
}


impl<T, U> Node<T, U> {
    pub fn new(key: Option<T>, value: U) -> Self{
        Self {
            value,
            key,
            left: None,
            right: None
        }
    }
}


pub fn bubble_sort_nodes(nodes: &mut Vec<Node<u8, u32>>) -> () {
    let n: usize = nodes.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if nodes[j].value > nodes[j + 1].value {
                nodes.swap(j, j + 1);
            }
            else if nodes[j].value == nodes[j + 1].value && nodes[j].key > nodes[j + 1].key{
                nodes.swap(j, j + 1);
            }
        }
    }
}


pub fn dfs(hashmap: &mut HashMap<u8, u32>, node: Box<Node<u8, u32>>, deep: u32) -> () {
    let mut flag = 0;
    match node.left {
        Some(next) => dfs(hashmap, next, deep + 1),
        None => flag += 1
    }

    match node.right {
        Some(next) => dfs(hashmap, next, deep + 1),
        None => flag += 1
    }

    if flag == 2 {
        if let Some(key) = node.key {
            hashmap.insert(key, deep);
        }
    }
}


pub fn create_tree(values: &mut Vec<u8>, keys: &mut Vec<u8>, hashmap: &mut HashMap<u8, u32>) -> Box<Node<u8, u32>>{
    let mut root: Box<Node<u8, u32>> = Box::new(Node::new(None, 0));
    for index in 0..values.len() {
        let mut current_node = &mut root;
        for u in 0..values[index] {
            let bit = take_bit(hashmap[&keys[index]], values[index] - (u + 1));
            if bit == 0 {
                if current_node.left.is_none() {
                    current_node.left = Some(Box::new(Node::new(None, 0)));
                }
                current_node = current_node.left.as_mut().unwrap();
            } else {
                if current_node.right.is_none() {
                    current_node.right = Some(Box::new(Node::new(None, 0)));
                }
                current_node = current_node.right.as_mut().unwrap();
            }
        }
        current_node.key = Some(keys[index]);
    }

    root
}
