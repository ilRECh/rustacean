// for handling arguments that are passed from a command line
use std::env;
use std::fs;

fn main() {
    // get the arguments
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::new(&args);

    println!("Searching for \"{}\" in the file named \"{}\"", config.regex, config.filename);

    let content = fs::read_to_string(config.filename)
        .expect("The file must be accessbile");

    println!("{content}");
}

struct Config {
    regex: String,
    filename: String
}

impl Config {
    pub fn new(args: &[String]) -> Self {
        if args.len() < 2 {
            println!("Not enugh arguments.\nUsage: minigrep <regex> <filename>");
            std::process::exit(1);
        }

        Config {
            regex: args[0].clone(),
            filename: args[1].clone()
        }
    }
}
