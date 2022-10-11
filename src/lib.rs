use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    // (ts) String[] = (rs) [String]
    // 'static = extend lifetime of a variable
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

    /*
    ? is the operator to propagate errors
    When ? is placed after a result and the value is Ok(), it will continue on this expression
    but if Err(), the value will be return whole function to the caller.
    */

    let contents: String = fs::read_to_string(config.file_path)?;

    for line in search(&config.query, &contents) {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    'a define a explicit lifetime.
    Rust require you to connect arguments to return values in the signature.
    */
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}
