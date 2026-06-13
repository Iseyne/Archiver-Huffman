use crate::huffman_utils::*;
use crate::tree_lib::*;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

// First pass: read file and count occurrences of each byte.
fn count_frequencies(file: &mut File) -> Result<(HashMap<u8, u32>, u64), String> {
    let original_size = file
        .metadata()
        .map_err(|e| format!("Failed to get metadata: {}", e))?
        .len();

    let mut buffer = [0u8; 1];
    let mut hashmap: HashMap<u8, u32> = HashMap::new();

    while file.read_exact(&mut buffer).is_ok() {
        let byte = buffer[0];
        match hashmap.get(&byte).copied() {
            Some(count) => hashmap.insert(byte, count + 1),
            None => hashmap.insert(byte, 1),
        };
    }

    Ok((hashmap, original_size))
}

// Build Huffman tree from frequencies, compute code lengths and canonical codes.
fn build_canonical_codes(frequencies: &HashMap<u8, u32>) -> Result<(Vec<u8>, Vec<u8>, HashMap<u8, u32>), String> {
    let mut nodes = create_nodes(frequencies);
    let num_of_elements = nodes.len();

    if num_of_elements == 0 {
        return Err("File is empty, nothing to compress.".to_string());
    }

    huffmans_algorithm(&mut nodes, num_of_elements);

    let root = Box::new(nodes.remove(0));
    let mut code_map = frequencies.clone();
    dfs(&mut code_map, &root, 0);

    let mut keys: Vec<u8> = Vec::new();
    let mut values: Vec<u8> = Vec::new();
    for (key, value) in &code_map {
        keys.push(*key);
        values.push(*value as u8);
    }

    sort_vectors(&mut values, &mut keys);
    canonical_code(&values, &keys, &mut code_map);

    Ok((keys, values, code_map))
}

// Write archive header: original size, symbol count, code-length table.
fn write_archive_header(file: &mut File, original_size: u64, keys: &[u8], values: &[u8]) -> Result<(), String> {
    file.write_all(&original_size.to_le_bytes())
        .map_err(|_| "Problem writing original size".to_string())?;

    file.write_all(&[(keys.len() - 1) as u8])
        .map_err(|_| "Problem writing number of elements".to_string())?;

    if keys.len() < 128 {
        for i in 0..keys.len() {
            file.write_all(&[keys[i], values[i]])
                .map_err(|_| "Problem writing the file".to_string())?
        }
    } else {
        for byte in 0..=255u8 {
            match keys.iter().position(|&k| k == byte) {
                Some(idx) => file.write_all(&[values[idx]]),
                None => file.write_all(&[0]),
            }
            .map_err(|_| "Problem writing the file".to_string())?;
        }
    }

    Ok(())
}

// Second pass: read input again, write Huffman-coded bits into output.
fn write_compressed_data(input: &mut File, output: &mut File, keys: &[u8], values: &[u8], codes: &HashMap<u8, u32>) -> Result<(), String> {
    input
        .seek(SeekFrom::Start(0))
        .map_err(|e| format!("Problem seeking file: {}", e))?;

    let mut buffer = [0u8; 1];
    let mut out_byte = 0u8;
    let mut bit_count = 0;

    while input.read_exact(&mut buffer).is_ok() {
        let byte = buffer[0];
        let code_len = match keys.iter().position(|&k| k == byte) {
            Some(i) => values[i],
            None => return Err("Can't find value by key".to_string()),
        };
        let code = codes[&byte];

        for i in 0..code_len {
            let bit = take_bit(code, code_len - (i + 1));
            out_byte <<= 1;
            out_byte += bit;
            bit_count += 1;

            if bit_count == 8 {
                output
                    .write_all(&[out_byte])
                    .map_err(|_| "Problem writing the file".to_string())?;
                out_byte = 0;
                bit_count = 0;
            }
        }
    }

    if bit_count > 0 {
        out_byte <<= 8 - bit_count;
        output
            .write_all(&[out_byte])
            .map_err(|_| "Problem writing the file".to_string())?;
    }

    Ok(())
}

// Compress a file using Huffman coding with canonical codes.
pub fn zip(input_filename: String, output_filename: String) -> Result<String, String> {
    let mut input_file =
        fs::File::open(&input_filename).map_err(|e| format!("Problem reading the file: {}", e))?;

    let (frequencies, original_size) = count_frequencies(&mut input_file)?;
    let (keys, values, codes) = build_canonical_codes(&frequencies)?;

    let mut output_file =
        File::create(&output_filename).map_err(|_| "Problem creating the file".to_string())?;

    write_archive_header(&mut output_file, original_size, &keys, &values)?;
    write_compressed_data(&mut input_file, &mut output_file, &keys, &values, &codes)?;

    Ok("File was zipped correctly.".to_string())
}

// Read and parse archive header: size, symbol count, code-length table.
fn read_archive_header(file: &mut File) -> Result<(u64, Vec<u8>, Vec<u8>, HashMap<u8, u32>), String> {
    let mut size_buf = [0u8; 8];
    file.read_exact(&mut size_buf)
        .map_err(|_| "Problem reading the file".to_string())?;
    let output_file_size = u64::from_le_bytes(size_buf);

    let mut byte_buf = [0u8; 1];
    file.read_exact(&mut byte_buf)
        .map_err(|_| "Problem reading the file".to_string())?;
    let stored_count = byte_buf[0];

    let mut hashmap: HashMap<u8, u32> = HashMap::new();
    let mut keys: Vec<u8> = Vec::new();
    let mut values: Vec<u8> = Vec::new();

    if stored_count < 127 {
        for _ in 0..=stored_count {
            file.read_exact(&mut byte_buf)
                .map_err(|_| "Problem reading the file".to_string())?;
            let key = byte_buf[0];

            file.read_exact(&mut byte_buf)
                .map_err(|_| "Problem reading the file".to_string())?;
            let value = byte_buf[0];

            keys.push(key);
            values.push(value);
            hashmap.insert(key, value as u32);
        }
    } else {
        for index in 0..=255 {
            file.read_exact(&mut byte_buf)
                .map_err(|_| "Problem reading the file".to_string())?;

            if byte_buf[0] != 0 {
                keys.push(index);
                values.push(byte_buf[0]);
                hashmap.insert(index, byte_buf[0] as u32);
            }
        }
    }

    if keys.is_empty() {
        return Err("The zipped file has the empty table".to_string());
    }

    Ok((output_file_size, keys, values, hashmap))
}

// Decompress a file: reconstruct tree, decode bits, verify integrity.
pub fn unzip(input_filename: String, output_filename: String) -> Result<String, String> {
    let mut input_file =
        fs::File::open(input_filename).map_err(|e| format!("Problem reading the file: {}", e))?;

    let (output_file_size, mut keys, mut values, mut hashmap) =
        read_archive_header(&mut input_file)?;

    sort_vectors(&mut values, &mut keys);
    canonical_code(&values, &keys, &mut hashmap);
    let mut root = create_tree(&values, &keys, &mut hashmap)?;

    let mut current_size: u64 = 0;
    let mut current_node = &mut root;
    let mut file =
        File::create(&output_filename).map_err(|_| "Problem creating the file".to_string())?;

    let mut byte_buf = [0u8; 1];
    loop {
        let check = input_file.read_exact(&mut byte_buf).is_ok();
        if !(check && current_size < output_file_size) {
            break;
        }

        let byte = byte_buf[0];
        for i in 0..8 {
            let bit = take_bit(byte as u32, 7 - i);
            current_node = if bit == 0 {
                current_node
                    .left
                    .as_mut()
                    .ok_or_else(|| "Left child unexpectedly None".to_string())?
            } else {
                current_node
                    .right
                    .as_mut()
                    .ok_or_else(|| "Right child unexpectedly None".to_string())?
            };

            if let Some(key) = current_node.key {
                file.write_all(&[key])
                    .map_err(|_| "Problem writing the file".to_string())?;
                current_size += 1;
                current_node = &mut root;
                if current_size >= output_file_size {
                    break;
                }
            }
        }
    }

    if current_size != output_file_size {
        drop(file);
        let _ = fs::remove_file(&output_filename);
        return Err("Decompressed size does not match expected size".to_string());
    }

    if !(current_node.key.is_none() && current_node.value == 0) {
        return Err("Incorrect type of the zipped file".to_string());
    }

    Ok("File was unzipped correctly.".to_string())
}
