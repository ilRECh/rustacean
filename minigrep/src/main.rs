// for handling arguments that are passed from a command line
use std::{ env, fs, process, error::Error };

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err_str| {
            println!("{err_str}");
            process::exit(1);
        });

    println!("Searching for \"{}\" in the file named \"{}\"", config.regex, config.filename);

    if let Err(err) = run(config) {
        println!("{err}");
        process::exit(1);
    }
}

fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("{content}");

    Ok(())
}

struct Config {
    regex: String,
    filename: String
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments.\nUsage: minigrep <regex> <filename>");
        }

        Ok(Config {
            regex: args[0].clone(),
            filename: args[1].clone()
        })
    }
}
