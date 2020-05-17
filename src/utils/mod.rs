use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 2 {
            return Err("Missing file argument")
        } else if args.len() == 1 {
            return Err("Missing query and file arguments")
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();

            Ok(Config { query, filename})
        }   
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    println!("With text:\n\n{}", contents);

    Ok(())
}