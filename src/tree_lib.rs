use std::collections::HashMap;


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
        hashmap.insert(match node.key {Some(result) => result, None => 0}, deep);
    }
}