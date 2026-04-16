use Archiver_Huffman::{unzip, zip};
use std::fs;
use std::path::Path;
const OK_ZIP: &str = "File was zipped correctly.";
const OK_UNZIP: &str = "File was unzipped correctly.";

#[test]
fn random_true_test() {
    let input = "test_input.bin";
    let input_zipped = "test_input_zipped.bin";
    let input_unzipped = "test_input_unzipped.bin";

    fs::write(input, b"787878787878787878ocamldocker").unwrap();
    let zipped = match zip(input.to_string(), input_zipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_ZIP, zipped, "File must be zipped");
    let input_zipped_file = fs::File::open(input_zipped).unwrap();
    let size_input_zipped = input_zipped_file.metadata().unwrap().len() as u64;

    assert!(
        size_input_zipped >= 265,
        "The minimal size of zipped file must be 265 bytes, but not {}",
        size_input_zipped
    );
    let unzipped = match unzip(input_zipped.to_string(), input_unzipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_UNZIP, unzipped, "File must be unzipped");

    let from_input = fs::read(input).unwrap();
    let from_unzipped = fs::read(input_unzipped).unwrap();

    assert_eq!(
        from_input, from_unzipped,
        "Unzipped file should contain the same data with input file"
    );

    fs::remove_file(input).ok();
    fs::remove_file(input_zipped).ok();
    fs::remove_file(input_unzipped).ok();
}

#[test]
fn test_large_data() {
    let input = "test_large_data_input.bin";
    let input_zipped = "test_large_data_input_zipped.bin";
    let input_unzipped = "test_large_data_input_unzipped.bin";

    let data = vec![0xAD; 1024 * 1024];
    fs::write(input, &data).unwrap();
    let zipped = match zip(input.to_string(), input_zipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_ZIP, zipped, "File must be zipped");
    let input_zipped_file = fs::File::open(input_zipped).unwrap();
    let size_input_zipped = input_zipped_file.metadata().unwrap().len() as u64;

    assert!(
        size_input_zipped >= 265,
        "The minimal size of zipped file must be 265 bytes, but not {}",
        size_input_zipped
    );
    let unzipped = match unzip(input_zipped.to_string(), input_unzipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_UNZIP, unzipped, "File must be unzipped");

    let from_input = fs::read(input).unwrap();
    let from_unzipped = fs::read(input_unzipped).unwrap();

    assert_eq!(
        from_input, from_unzipped,
        "Unzipped file should contain the same data with input file"
    );

    fs::remove_file(input).ok();
    fs::remove_file(input_zipped).ok();
    fs::remove_file(input_unzipped).ok();
}

#[test]
fn test_empty_file() {
    let input = "test_empty_input.bin";
    let input_zipped = "test_empty_input_zipped.bin";

    fs::write(input, "").unwrap();
    let zipped = match zip(input.to_string(), input_zipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(
        "File is empty, nothing to compress.", zipped,
        "File must not be zipped"
    );
    assert!(!Path::new(input_zipped).exists());

    fs::remove_file(input).ok();
    fs::remove_file(input_zipped).ok();
}

#[test]

fn test_file_one_byte() {
    let input = "test_one_byte_input.bin";
    let input_zipped = "test_one_byte_input_zipped.bin";
    let input_unzipped = "test_one_byte_input_unzipped.bin";

    fs::write(input, b"z").unwrap();
    let zipped = match zip(input.to_string(), input_zipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_ZIP, zipped, "File must be zipped");
    let input_zipped_file = fs::File::open(input_zipped).unwrap();
    let size_input_zipped = input_zipped_file.metadata().unwrap().len() as u64;

    assert!(
        size_input_zipped == 265,
        "The size of zipped file must be 265 bytes, but not {}",
        size_input_zipped
    );
    let unzipped = match unzip(input_zipped.to_string(), input_unzipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_UNZIP, unzipped, "File must be unzipped");

    let from_input = fs::read(input).unwrap();
    let from_unzipped = fs::read(input_unzipped).unwrap();

    assert_eq!(
        from_input, from_unzipped,
        "Unzipped file should contain the same data with input file"
    );

    fs::remove_file(input).ok();
    fs::remove_file(input_zipped).ok();
    fs::remove_file(input_unzipped).ok();
}

#[test]
fn test_file_same_bytes() {
    let input = "test_same_bytes_input.bin";
    let input_zipped = "test_same_bytes_input_zipped.bin";
    let input_unzipped = "test_same_bytes_input_unzipped.bin";

    let data = vec![9; 512];
    fs::write(input, &data).unwrap();
    let zipped = match zip(input.to_string(), input_zipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_ZIP, zipped, "File must be zipped");
    let input_zipped_file = fs::File::open(input_zipped).unwrap();
    let size_input_zipped = input_zipped_file.metadata().unwrap().len() as u64;

    assert!(
        size_input_zipped == 328,
        "The size of zipped file must be 328 bytes, but not {}",
        size_input_zipped
    );
    let unzipped = match unzip(input_zipped.to_string(), input_unzipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_UNZIP, unzipped, "File must be unzipped");

    let from_input = fs::read(input).unwrap();
    let from_unzipped = fs::read(input_unzipped).unwrap();

    assert_eq!(
        from_input, from_unzipped,
        "Unzipped file should contain the same data with input file"
    );

    fs::remove_file(input).ok();
    fs::remove_file(input_zipped).ok();
    fs::remove_file(input_unzipped).ok();
}

#[test]
fn test_file_all_bytes_n_times() {
    let input = "test_all_bytes_n_times_input.bin";
    let input_zipped = "test_all_bytes_n_times_input_zipped.bin";
    let input_unzipped = "test_all_bytes_n_times_input_unzipped.bin";

    let mut data: Vec<u8> = Vec::new();
    for i in 0..=255 {
        for _ in 0..=i {
            data.push(i);
        }
    }
    fs::write(input, &data).unwrap();
    let zipped = match zip(input.to_string(), input_zipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_ZIP, zipped, "File must be zipped");
    let input_zipped_file = fs::File::open(input_zipped).unwrap();
    let size_input_zipped = input_zipped_file.metadata().unwrap().len() as u64;

    assert!(
        size_input_zipped >= 265 && size_input_zipped < 500000,
        "The size of zipped file must be from 265 to 50000 bytes, but not {}",
        size_input_zipped
    );
    let unzipped = match unzip(input_zipped.to_string(), input_unzipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_UNZIP, unzipped, "File must be unzipped");

    let from_input = fs::read(input).unwrap();
    let from_unzipped = fs::read(input_unzipped).unwrap();

    assert_eq!(
        from_input, from_unzipped,
        "Unzipped file should contain the same data with input file"
    );

    fs::remove_file(input).ok();
    fs::remove_file(input_zipped).ok();
    fs::remove_file(input_unzipped).ok();
}

#[test]
fn test_file_all_bytes() {
    let input = "test_all_bytes_input.bin";
    let input_zipped = "test_all_bytes_input_zipped.bin";
    let input_unzipped = "test_all_bytes_input_unzipped.bin";

    let mut data: Vec<u8> = Vec::new();
    for i in 0..=255 {
        data.push(i);
    }
    fs::write(input, &data).unwrap();
    let zipped = match zip(input.to_string(), input_zipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_ZIP, zipped, "File must be zipped");
    let input_zipped_file = fs::File::open(input_zipped).unwrap();
    let size_input_zipped = input_zipped_file.metadata().unwrap().len() as u64;

    let input_file = fs::File::open(input).unwrap();
    let size_input = input_file.metadata().unwrap().len() as u64;

    assert!(
        size_input_zipped <= size_input + 300,
        "The size of zipped file must be approximately same with size of input file, but not {}",
        size_input_zipped
    );
    let unzipped = match unzip(input_zipped.to_string(), input_unzipped.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert_eq!(OK_UNZIP, unzipped, "File must be unzipped");

    let from_input = fs::read(input).unwrap();
    let from_unzipped = fs::read(input_unzipped).unwrap();

    assert_eq!(
        from_input, from_unzipped,
        "Unzipped file should contain the same data with input file"
    );

    fs::remove_file(input).ok();
    fs::remove_file(input_zipped).ok();
    fs::remove_file(input_unzipped).ok();
}

#[test]
fn test_unzip_short_file_defect() {
    let input_zipped = "test_short_defect_zipped.bin";
    let output = "test_short_defect_out.bin";

    fs::write(input_zipped, &[0u8; 100]).unwrap();

    let unzipped = match unzip(input_zipped.to_string(), output.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert!(
        unzipped == "Incorrect type of the zipped file",
        "Should reject short file, got: {}",
        unzipped
    );
    assert!(
        !Path::new(output).exists(),
        "Output file should not be created"
    );

    fs::remove_file(input_zipped).ok();
    fs::remove_file(output).ok();
}


#[test]
fn test_unzip_file_damaged_table() {
    let input = "test_damaged_table_input.bin";
    let input_zipped = "test_damaged_table_zipped.bin";
    let output = "test_damaged_table_out.bin";

    fs::write(input, b"Hello world!").unwrap();
    let _ = zip(input.to_string(), input_zipped.to_string()).unwrap();

    let mut data = fs::read(input_zipped).unwrap();
    assert!(data.len() > 264, "Archive too small for test");
    for i in 8..264 {
        data[i] = 0;
    }
    fs::write(input_zipped, &data).unwrap();

    let unzipped = match unzip(input_zipped.to_string(), output.to_string()) {
        Ok(message) => message,
        Err(error) => error,
    };

    assert!(
        unzipped.contains("Incorrect type")
            || unzipped.contains("unexpectedly None")
            || unzipped.contains("empty"),
        "Should reject archive with damaged table, got: {}",
        unzipped
    );

    fs::remove_file(input).ok();
    fs::remove_file(input_zipped).ok();
    fs::remove_file(output).ok();
}
