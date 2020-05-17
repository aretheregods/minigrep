use std::env;
use std::process;
use minigrep::utils::{Config, run};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("There was a problem parsing arguments: {}", err);
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);

        process::exit(1);
    };
}
