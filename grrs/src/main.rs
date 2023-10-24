#![allow(unused)]

use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Parser)]
struct Cli {
    // The pattern to search for
    pattern: String,
    // The path to the file to read
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    // Method 1: read the whole file into memory
//     let content = std::fs::read_to_string(&args.path)
//         .expect("could not read file");

//     for line in content.lines() {
//         if line.contains(&args.pattern) {
//             println!("{}", line);
//     }
//   }

    // Method 2: read the file line by line
    let f = File::open(&args.path)
        .expect("could not read file");

    let reader = BufReader::new(f);
    match reader {
        Ok(_) => println!("File opened successfully"),
        Err(_) => println!("Error opening file"),}

    for line in reader.lines() {
        let line = line.expect("could not read line");
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }
    match line {
        Ok(_) => println!("Line read successfully"),
        Err(_) => println!("Error reading line"),
    }
}

// let pattern = std::env::args().nth(1).expect("missing pattern argument");
// let path = std::env::args().nth(2).expect("missing path argument");

// let args = Cli {
//     pattern: pattern,
//     path: std::path::PathBuf::from(path),
// };
