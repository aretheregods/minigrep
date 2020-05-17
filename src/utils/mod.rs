use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub insensitive: bool,
}

impl Config {
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing query argument"),
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Missing file argument"),
        };

        let env_var_undefined = env::var("insensitive").is_err();

        let insensitive = match args.next() {
            Some(arg) => {
                if !env_var_undefined 
                        || arg.to_lowercase() == "insensitive"
                        || arg.to_lowercase() == "--i" 
                {
                    true
                } else {
                    false
                }
            },
            None => {
                if !env_var_undefined {
                    true
                } else {
                    false
                }
            },
        };

        Ok(Config { query, filename, insensitive })
    }
}

pub fn search<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, file_contents: &'a str) -> Vec<&'a str> {
    file_contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
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
