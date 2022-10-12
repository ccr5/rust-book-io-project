use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
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
        let ignore_case: bool = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
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

    let results: Vec<&str> = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    /*
    'a define a explicit lifetime.
    Rust require you to connect arguments to return values in the signature.

    Example:

        If you see, the code has a for loop that is adding references (&str) from contents
        in a mutable vector called results to be return later.
        I know this cause i'm reading, but how the search
        function knows if this references was borrowed from query or content variable?
        So, we have to 'link' the variable and result to tell him it.

        Other explanation is that results is Vec<&str> and it will be return later.
        but &str is a reference for what?
    */
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query: String = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

/*
cfg = Configuration conditional checks are possible through two different operators
https://doc.rust-lang.org/rust-by-example/attribute/cfg.html

cfg(test) is enabled when building “unit tests” and “unit benchmarks” found
inside your library or binary crate. If you have a #[test] or #[bench] function inside
your library crate (src/lib.rs and its submodules) or your binary crate (src/main.rs or
its submodules), Cargo will build that entire crate with rustc --test, which enables
cfg(test) and produces a “test harness” binary containing all of its unit tests/benchmarks.
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query: &str = "duct";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query: &str = "rUsT";
        let contents: &str = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
