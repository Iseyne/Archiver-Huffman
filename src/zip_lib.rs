use std::fs;
use std::collections::HashMap;
use crate::tree_lib::*;
use crate::huffman_utils::*;
use std::fs::File;
use std::io::{Write, Seek, SeekFrom, Read};


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
        return Err("File is empty, nothing to compress.".to_string());
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
    
    sort_vectors(&mut values, &mut keys);
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

    Ok("File was zipped correctly.".to_string())
}


pub fn unzip(input_filename: String, output_filename: String) -> Result<String, String> {
    let mut input_file = fs::File::open(input_filename)
        .map_err(|e| format!("Problem reading the file: {}", e))?;

    let mut buffer_for_size:[u8; 8] = [0u8; 8];
    input_file.read(&mut buffer_for_size)
        .map_err(|_| "Problem reading the file".to_string())?;
    let output_file_size = u64::from_le_bytes(buffer_for_size);
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
        return Err("The zipped file have the empty table".to_string())
    }
    sort_vectors(&mut values, &mut keys);
    canonical_code(&mut values, &mut keys, &mut hashmap);
    let mut root = match create_tree(&mut values, &mut keys, &mut hashmap) {
        Ok(node) => node, 
        Err(e) => return Err(e)
    };
    let mut current_size: u64 = 0;
    let mut current_node = &mut root;
    let mut file = File::create(&output_filename)
        .map_err(|_| "Problem creating the file".to_string())?;

    while input_file.read(&mut buffer_for_byte)
            .map_err(|_| "Problem reading the file".to_string())? != 0 &&
            current_size < output_file_size {
        let byte = buffer_for_byte[0];
        for index in 0..8 {
            let bit = take_bit(byte as u32, 7 - index);
            if bit == 0 {
                current_node = current_node.left
                    .as_mut()
                    .ok_or_else(|| "Left child unexpectedly None"
                    .to_string())?;
            } else if bit == 1 {
                current_node = current_node.right
                    .as_mut()
                    .ok_or_else(|| "Right child unexpectedly None"
                    .to_string())?;
            } else {
                return Err(format!("Bit must be 0 or 1, but not {}", bit).to_string())
            }

            match current_node.key {
                Some(key) => {
                    file.write_all(&[key])
                        .map_err(|_| "Problem writing the file".to_string())?;
                    current_size += 1;
                    current_node = &mut root;
                    if current_size >= output_file_size {
                        break
                    }
                },
                None => {}
            }
        }
    }

    if !(current_node.key.is_none() && current_node.value == 0) {
        return Err("Incorrect type of the zipped file".to_string())
    }
    
    Ok("File was unzipped correctly.".to_string())
}
