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

pub fn dfs(hashmap: &mut HashMap<u8, u32>, node: &Node<u8, u32>, deep: u32) {
    let mut flag = 0;
    if let Some(left) = &node.left {
        dfs(hashmap, left, deep + 1);
    } else {
        flag += 1;
    }
    if let Some(right) = &node.right {
        dfs(hashmap, right, deep + 1);
    } else {
        flag += 1;
    }
    if flag == 2
        && let Some(key) = node.key
    {
        hashmap.insert(key, deep);
    }
}
