use std::env;
use std::error::Error;
use std::fs;
use std::process;

use minigrep::search;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("[ERROR]: Provide enough arguments: {err}", err = err);
        process::exit(1);
    });

    println!("[INFO]: Query: {query}", query = config.query);
    println!("[INFO]: File : {file_path}", file_path = config.file_path);

    if let Err(err) = run(config) {
        println!("[ERROR]: Failed to read the file: {err}", err = err);
        process::exit(1);
    }
}

struct Config<'a> {
    query: &'a str,
    file_path: &'a str,
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

        Ok(Config { query, file_path })
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}")
    }

    println!("[INFO]: File contents:\n{contents}");
    Ok(())
}
