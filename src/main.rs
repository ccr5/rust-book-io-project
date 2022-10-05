use std::env;
use std::fs;

struct Config {
    query: String,
    file_path: String,
}

impl Config {
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query: String = args[1].clone();
        let file_path: String = args[2].clone();
        Ok(Config { query, file_path })
    }
}

fn main() {
    // return the program binary and all args
    let args: Vec<String> = env::args().collect();

    /*
    macro used to depuration
    dbg!(args);
    */

    let config: Config = Config::build(&args).unwrap_or_else(|err|: &str {
        println!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    let contents: String =
        fs::read_to_string(config.file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
