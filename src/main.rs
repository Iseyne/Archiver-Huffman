mod zip_lib;
mod tree_lib;
use std::env;

fn parser(args: Vec<String>) -> Result<String, String> {
    let output_filename: String;
    if args.len() == 3 {
        let last_point = match args[2].rfind('.') {
            Some(position) => position,
            None => return Err("Path to file doesn't contain a file extension".to_string())
        };

        let name: String;
        if args[1] == "zip" {
            name = "_zipped".to_string();
        } else if args[1] == "unzip" {
            name = "_unzipped".to_string();
        } else {
            return Err("First argument should be \"zip\" or \"unzip\".".to_string())
        }

        output_filename = format!("{}{}{}", &args[2][0..last_point], name, &args[2][last_point..]);
    }
    else if args.len() == 4 {
        output_filename = args[3].clone();
    }
    else{
        return Err("Incorrect number of arguments.".to_string())
    }

    Ok(output_filename)
}


fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    let output_filename = match parser(args.clone()) {
        Ok(path) => path,
        Err(e) => return Err(e)
    };

    
    
    if args[1] == "zip" {
        match zip_lib::zip(args[2].clone(), output_filename) {
            Ok(message) => println!("{}", message), 
            Err(e) => return Err(e)
        };
    }
    else if args[1] == "unzip" {
        match zip_lib::unzip(args[2].clone(), output_filename) {
            Ok(message) => println!("{}", message), 
            Err(e) => return Err(e)
        };
    }
    else {
        return Err("First argument should be \"zip\" or \"unzip\".".to_string())
    }     
    Ok(())
}
