// for handling arguments that are passed from a command line
use std::env;
use std::fs;

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 2 {
        println!("Usage: minigrep <regex> <filename>");
    }

    let regex = &args[0];
    let filename = &args[1];

    println!("Searching for \"{regex}\" in the file named \"{filename}\"");

    let content = fs::read_to_string(filename)
        .expect("The file must be accessbile");

    println!("{content}");
}
