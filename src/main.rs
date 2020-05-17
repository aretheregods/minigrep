use std::env;
use std::fs;
use minigrep::utils;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, filename) = utils::parse_config(&args);

    println!("Searching for {}", query);
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Error opening file");

    println!("With text:\n\n{}", contents);
}
