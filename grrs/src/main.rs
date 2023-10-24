fn main() {
    println!("Hello, world!");
}


struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

let pattern = std::env::args().nth(1).expect("missing pattern argument");
let path = std::env::args().nth(2).expect("missing path argument");
let args = Cli {
    pattern: pattern,
    path: std::path::PathBuf::from(path),
};
