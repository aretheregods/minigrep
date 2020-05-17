use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() == 2 {
            return Err("Missing file argument");
        } else if args.len() == 1 {
            return Err("Missing query and file arguments");
        } else {
            let query = args[1].clone();
            let filename = args[2].clone();

            Ok(Config { query, filename })
        }
    }
}

pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in file_contents.lines() {
        if line.to_lowercase().contains(&query.to_lowercase()) {
            results.push(line);
        }
    }

    results
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}
