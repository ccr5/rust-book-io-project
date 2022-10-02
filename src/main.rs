use std::env;
use std::fs;

fn main() {
    // return the program binary and all args
    let args: Vec<String> = env::args().collect();

    // macro used to depuration
    // dbg!(args);

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);

    let contents: String =
        fs::read_to_string(file_path).expect("Should have been able to read the file");

    println!("With text:\n{contents}");
}
