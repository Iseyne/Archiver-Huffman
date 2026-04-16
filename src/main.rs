use Archiver_Huffman::{unzip, zip};
use std::env;
use std::path::Path;

fn parser(args: Vec<String>) -> Result<String, String> {
    let output_filename: String;

    if args.len() == 3 {
        let input_path = &args[2];
        let path = Path::new(input_path);

        let stem = path
            .file_stem()
            .and_then(|s| s.to_str())
            .ok_or_else(|| "Invalid file stem (not valid UTF-8)".to_string())?;

        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| format!(".{}", e))
            .unwrap_or_default();

        let name = match args[1].as_str() {
            "zip" => "_zipped",
            "unzip" => "_unzipped",
            _ => return Err("First argument should be \"zip\" or \"unzip\".".to_string()),
        };

        output_filename = format!("{}{}{}", stem, name, ext);
    } else if args.len() == 4 {
        output_filename = args[3].clone();
    } else {
        return Err("Incorrect number of arguments.".to_string());
    }

    Ok(output_filename)
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let output_filename = match parser(args.clone()) {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    if args[1] == "zip" {
        match zip(args[2].clone(), output_filename) {
            Ok(message) => println!("{}", message),
            Err(e) => return Err(e),
        };
    } else if args[1] == "unzip" {
        match unzip(args[2].clone(), output_filename) {
            Ok(message) => println!("{}", message),
            Err(e) => return Err(e),
        };
    } else {
        return Err("First argument should be \"zip\" or \"unzip\".".to_string());
    }
    Ok(())
}
