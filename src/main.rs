use rust_book_io_project::{run, Config};
use std::env;
use std::process;

fn main() {
    // return the program binary and all args
    let args: Vec<String> = env::args().collect();

    // macro used to depuration
    dbg!(&args);

    // unwrap_or_else() ensure the execution of the code or will be execute a fallback function
    // Ex: build implementation return a result. if error, it will be execute on fallback
    let config: Config = Config::build(&args).unwrap_or_else(|err: &str| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    if let Err(e) = run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}
