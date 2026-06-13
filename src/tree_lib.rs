use std::collections::HashMap;

// Node of a binary tree.
// Key is present only for leaves; value stores frequency or code length.
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

// DFS traversal that records depths of leaf nodes.
// After traversal, the hashmap maps each byte to its Huffman code length.
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
