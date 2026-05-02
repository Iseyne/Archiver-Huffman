use crate::tree_lib::*;
use std::collections::HashMap;

pub fn create_nodes(hashmap: &HashMap<u8, u32>) -> Vec<Node<u8, u32>> {
    let mut nodes: Vec<Node<u8, u32>> = Vec::new();
    for (key, value) in hashmap {
        let new_node = Node::new(Some(*key), *value);
        nodes.push(new_node);
    }
    nodes.sort_unstable_by(|a, b| a.value.cmp(&b.value).then_with(|| a.key.cmp(&b.key)));
    nodes
}

pub fn take_bit(num: u32, position: u8) -> u8 {
    if (num & (1 << position)) == (1 << position) {
        1
    } else {
        0
    }
}

pub fn sort_vectors<T: Ord, U: Ord>(v1: &mut Vec<T>, v2: &mut Vec<U>) {
    let mut pairs: Vec<_> = v1.drain(..).zip(v2.drain(..)).collect();
    pairs.sort_unstable_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));
    for (a, b) in pairs {
        v1.push(a);
        v2.push(b);
    }
}

pub fn huffmans_algorithm(nodes: &mut Vec<Node<u8, u32>>, num_of_elements: usize) {
    if num_of_elements == 1 {
        let left: Node<u8, u32> = nodes.remove(0);
        let right: Node<u8, u32> = Node::new(None, 0);
        let mut new_node: Node<u8, u32> = Node::new(None, left.value + right.value);
        new_node.left = Some(Box::new(left));
        new_node.right = Some(Box::new(right));
        nodes.insert(0, new_node);
    } else {
        for _ in 0..(num_of_elements - 1) {
            let left: Node<u8, u32> = nodes.remove(0);
            let right: Node<u8, u32> = nodes.remove(0);
            let mut new_node: Node<u8, u32> = Node::new(None, left.value + right.value);
            new_node.left = Some(Box::new(left));
            new_node.right = Some(Box::new(right));
            let index = match nodes.binary_search_by_key(&new_node.value, |n| n.value) {
                Ok(pos) | Err(pos) => pos,
            };
            nodes.insert(index, new_node);
        }
    }
}

pub fn canonical_code(values: &[u8], keys: &[u8], hashmap: &mut HashMap<u8, u32>) {
    let mut prev_length = values[0];
    let mut prev_code = 0;
    hashmap.insert(keys[0], 0);
    for index in 1..keys.len() {
        let code = (prev_code + 1) << (values[index] - prev_length);
        prev_length = values[index];
        prev_code = code;
        hashmap.insert(keys[index], code);
    }
}

pub fn create_tree(
    values: &[u8],
    keys: &[u8],
    hashmap: &mut HashMap<u8, u32>,
) -> Result<Box<Node<u8, u32>>, String> {
    let mut root: Box<Node<u8, u32>> = Box::new(Node::new(None, 0));
    for index in 0..values.len() {
        let mut current_node = &mut root;
        for u in 0..values[index] {
            let bit = take_bit(hashmap[&keys[index]], values[index] - (u + 1));
            if bit == 0 {
                if current_node.left.is_none() {
                    current_node.left = Some(Box::new(Node::new(None, 1)));
                }
                current_node = current_node
                    .left
                    .as_mut()
                    .ok_or_else(|| "Internal error: left child unexpectedly None".to_string())?;
            } else if bit == 1 {
                if current_node.right.is_none() {
                    current_node.right = Some(Box::new(Node::new(None, 1)));
                }
                current_node = current_node
                    .right
                    .as_mut()
                    .ok_or_else(|| "Internal error: right child unexpectedly None".to_string())?;
            } else {
                return Err(format!("Bit must be 0 or 1, but not {}", bit));
            }
        }
        current_node.key = Some(keys[index]);
        current_node.value = values[index] as u32;
    }

    Ok(root)
}
