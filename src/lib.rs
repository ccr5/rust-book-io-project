use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    /*
    Box<dyn Error> means the function will return a type that
    implements the Error trait, but we don’t have to specify
    what particular type the return value will be

    The dyn keyword is short for “dynamic.”
    */

    let contents: String = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
