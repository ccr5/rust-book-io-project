use std::env;

fn main() {
    // return the program binary and all args
    let args: Vec<String> = env::args().collect();

    // macro used to depuration
    // dbg!(args);

    let query: &String = &args[1];
    let file_path: &String = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", file_path);
}
