use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;
use minigrep::search_case_insensitive;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("[ERROR]: Provide enough arguments: {err}", err = err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("[ERROR]: Failed to read the file: {err}", err = err);
        process::exit(1);
    }
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
    ignore_case: bool,
}

impl<'a> Config<'a> {
    fn build<'b>(args: &'b [String]) -> Result<Config<'b>, &'static str> {
        if args.len() < 2 {
            return Err("arg[1] - query: missing, arg[2] - file path: missing");
        }
        if args.len() < 3 {
            return Err("arg[2] - file path: missing");
        }

        let query = &args[1];
        let file_path = &args[2];
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(config.query, &contents)
    } else {
        search(config.query, &contents)
    };

    for line in results {
        println!("{line}")
    }

    Ok(())
}
