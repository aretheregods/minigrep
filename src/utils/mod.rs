use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub insensitive: bool,
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

            let mut insensitive = false;

            if args.len() == 4 
            && (
                args[3].to_lowercase() == "insensitive"
                || args[3].to_lowercase() == "--i"
            ) {
                insensitive = true;
            }

            Ok(Config { query, filename, insensitive })
        }
    }
}

pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    let mut results = vec![];

    for line in file_contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
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

    let results = if config.insensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}
