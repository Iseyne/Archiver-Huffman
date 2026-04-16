use std::collections::HashMap;

pub struct Node<T, U> {
    pub left: Option<Box<Node<T, U>>>,
    pub right: Option<Box<Node<T, U>>>,
    pub key: Option<T>,
    pub value: U,
}

impl<T, U> Node<T, U> {
    pub fn new(key: Option<T>, value: U) -> Self {
        Self {
            value,
            key,
            left: None,
            right: None,
        }
    }
}

pub fn dfs(hashmap: &mut HashMap<u8, u32>, node: Box<Node<u8, u32>>, deep: u32) -> () {
    let mut flag = 0;
    match node.left {
        Some(next) => dfs(hashmap, next, deep + 1),
        None => flag += 1,
    }

    match node.right {
        Some(next) => dfs(hashmap, next, deep + 1),
        None => flag += 1,
    }

    if flag == 2 {
        if let Some(key) = node.key {
            hashmap.insert(key, deep);
        }
    }
}
