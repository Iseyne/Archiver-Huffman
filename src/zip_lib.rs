use std::fs;
use std::collections::HashMap;
use crate::tree_lib::*;
use std::fs::File;
use std::io::{Write, Seek, SeekFrom, Read};


fn create_nodes(hashmap: &HashMap<u8, u32>) -> Vec<Node<u8, u32>>{
    let mut nodes: Vec<Node<u8, u32>> = Vec::new();
    for (key, value) in hashmap {
        let new_node = Node::new(Some(*key), *value);
        nodes.push(new_node);
    }
    bubble_sort_nodes(&mut nodes);
    return nodes
}

fn find_index(nodes: &mut Vec<Node<u8, u32>>, item: &mut Node<u8, u32>) -> usize {
    let size: usize = nodes.len();
    let mut min_index = 0;
    let mut max_index = size;
    while min_index < max_index {
        let middle = (max_index + min_index) / 2;
        if nodes[middle].value < item.value {
            min_index = middle + 1;
        }
        else{
            max_index = middle;
        }
    }
    min_index 
}

pub fn take_bit(num: u32, position: u8) -> u8 {
    if (num & (1 << position)) == (1 << position) {1} else {0}
}


fn bubble_sort_vectors<T: Ord, U: Ord>(vector_1: &mut Vec<T>, vector_2: &mut Vec<U>) -> () {
    let n: usize = vector_1.len();
    for i in 0..n {
        for j in 0..(n - i - 1) {
            if vector_1[j] > vector_1[j + 1] {
                vector_1.swap(j, j + 1);
                vector_2.swap(j, j + 1);
            }
            else if vector_1[j] == vector_1[j + 1] && vector_2[j] > vector_2[j + 1] {
                vector_1.swap(j, j + 1);
                vector_2.swap(j, j + 1);
            }
        }
    }
}


fn huffmans_algorithm(nodes: &mut Vec<Node<u8, u32>>, num_of_elements: usize) -> () {
    if num_of_elements == 1 {
        let left: Node<u8, u32> = nodes.remove(0);
        let right: Node<u8, u32> = Node::new(None, 0);
        let mut new_node: Node<u8, u32> = Node::new(None, left.value + right.value);
        new_node.left = Some(Box::new(left));
        new_node.right = Some(Box::new(right));
        nodes.insert(0, new_node);
    }
    else {
        for _ in 0..(num_of_elements - 1) {
            let left: Node<u8, u32> = nodes.remove(0);
            let right: Node<u8, u32> = nodes.remove(0);
            let mut new_node: Node<u8, u32> = Node::new(None, left.value + right.value);
            new_node.left = Some(Box::new(left));
            new_node.right = Some(Box::new(right));
            let index = find_index(nodes, &mut new_node);
            nodes.insert(index, new_node);
        }
    }
}


fn canonical_code(values: &mut Vec<u8>, keys: &mut Vec<u8>, hashmap: &mut HashMap<u8, u32>) -> () {
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


pub fn zip(input_filename: String, output_filename: String) -> Result<String, String> {
    let mut input_file = fs::File::open(&input_filename)
        .map_err(|e| format!("Problem reading the file: {}", e))?;

    let original_size = input_file.metadata()
        .map_err(|e| format!("Failed to get metadata: {}", e))?
        .len() as u64;

    let mut buffer = [0u8; 1];
    let mut hashmap: HashMap<u8, u32>= HashMap::new();

    
    while input_file.read(&mut buffer).map_err(|_| "Problem reading the file".to_string())? != 0 { 
        let byte = buffer[0];

        match hashmap.get(&byte).copied() {
            Some(result) => {hashmap.insert(byte, result + 1)},
            None => {hashmap.insert(byte, 1)}
        };
    }
    let mut nodes = create_nodes(&hashmap);
    let num_of_elements = nodes.len();

    if num_of_elements == 0 {
        return Ok("File is empty, nothing to compress.".to_string());
    }

    huffmans_algorithm(&mut nodes, num_of_elements);

    let root = Box::new(nodes.remove(0));
    dfs(&mut hashmap, root, 0);
    let mut keys: Vec<u8> = Vec::new();
    let mut values: Vec<u8> = Vec::new();
    for (key, value) in &hashmap {
        keys.push(*key);
        values.push(*value as u8);
    }
    
    bubble_sort_vectors(&mut values, &mut keys);
    canonical_code(&mut values, &mut keys, &mut hashmap);

    let mut file = File::create(&output_filename)
        .map_err(|_| "Problem creating the file".to_string())?;

     file.write_all(&original_size.to_le_bytes())
        .map_err(|_| "Problem writing original size".to_string())?;

    for index in 0..=255 {
        match keys.iter().position(|&item| item == index) {
            Some(result) => file.write_all(&[values[result]]).map_err(|_| "Problem writing the file".to_string())?,
            None => file.write_all(&[0]).map_err(|_| "Problem writing the file".to_string())?
        }
    }

    input_file.seek(SeekFrom::Start(0))
        .map_err(|e| format!("Problem seeking file: {}", e))?;
    let mut buffer_for_output = 0u8;
    let mut buffer_length = 0;

    while input_file.read(&mut buffer).map_err(|_| "Problem reading the file".to_string())? != 0 { 
        let byte = buffer[0];
        let value_length = match keys.iter().position(|&key| key == byte) {
            Some(index) => values[index],
            None => return Err("Can't find value by key".to_string())
        };
        for index in 0..value_length {
            let bit = take_bit(hashmap[&byte], value_length - (index + 1));

            buffer_for_output <<= 1;
            buffer_for_output += bit;

            buffer_length += 1;
            if buffer_length == 8 {
                file.write_all(&[buffer_for_output]).map_err(|_| "Problem writing the file".to_string())?;
                buffer_length = 0;
                buffer_for_output = 0u8;
            }
        }
    }

    if buffer_length > 0 {
        buffer_for_output <<= 8 - buffer_length;
        file.write_all(&[buffer_for_output]).map_err(|_| "Problem writing the file".to_string())?;
    }
    println!("{}", original_size);

    Ok("File was zipped correctly.".to_string())
}


pub fn unzip(input_filename: String, output_filename: String) -> Result<String, String> {
    let mut input_file = fs::File::open(input_filename)
        .map_err(|e| format!("Problem reading the file: {}", e))?;

    let mut buffer_for_size:[u8; 8] = [0u8; 8];
    input_file.read(&mut buffer_for_size)
        .map_err(|_| "Problem reading the file".to_string())?;
    let _output_file_size = u64::from_le_bytes(buffer_for_size);
    let input_file_size = input_file.metadata()
        .map_err(|e| format!("Failed to get metadata: {}", e))?
        .len() as u64;

    if input_file_size < 256 {
        return Err("Incorrect type of the zipped file".to_string())
    }

    let mut buffer_for_byte = [0u8; 1];
    let mut hashmap: HashMap<u8, u32> = HashMap::new();
    let mut keys: Vec<u8> = Vec::new();
    let mut values: Vec<u8> = Vec::new();
    for index in 0..=255 {
        input_file.read(&mut buffer_for_byte)
            .map_err(|_| "Problem reading the file".to_string())?;

        if buffer_for_byte[0] != 0 {
            keys.push(index);
            values.push(buffer_for_byte[0]);
            hashmap.insert(index, buffer_for_byte[0] as u32);
        }

    }

    if keys.len() == 0 {
        return Err("The unzipped file is empty".to_string())
    }
    bubble_sort_vectors(&mut values, &mut keys);
    canonical_code(&mut values, &mut keys, &mut hashmap);
    let _root = create_tree(&mut values, &mut keys, &mut hashmap);

    
    let _output_file = fs::File::create(output_filename)
        .map_err(|e| format!("Problem reading the file: {}", e))?;

    Ok("File was unzipped correctly.".to_string())
}