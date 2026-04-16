use archiver_huffman::{unzip, zip};
use std::env;
use std::path::Path;

fn parser(args: &[String]) -> Result<(&str, &str, String), String> {
    if args.len() < 3 || args.len() > 4 {
        return Err("Usage: program <zip|unzip> <input> [output]".to_string());
    }

    let command = &args[1];
    if command != "zip" && command != "unzip" {
        return Err("First argument should be \"zip\" or \"unzip\".".to_string());
    }

    let input_path = &args[2];
    let output_filename = if args.len() == 4 {
        args[3].clone()
    } else {
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

        let suffix = if command == "zip" {
            "_zipped"
        } else {
            "_unzipped"
        };
        format!("{}{}{}", stem, suffix, ext)
    };

    Ok((command.as_str(), input_path.as_str(), output_filename))
}

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let (command, input, output) = parser(&args)?;

    let result = match command {
        "zip" => zip(input.to_string(), output),
        "unzip" => unzip(input.to_string(), output),
        _ => unreachable!(),
    };

    match result {
        Ok(message) => println!("{}", message),
        Err(e) => return Err(e),
    }

    Ok(())
}
