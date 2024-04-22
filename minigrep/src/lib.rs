use std::{ fs, error::Error };

pub struct Config {
    pub regex: String,
    pub filename: String
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

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;

    println!("{content}");

    Ok(())
}
