// for handling arguments that are passed from a command line
use std::{ env, process };
use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    let config = Config::build(&args)
        .unwrap_or_else(|err_str| {
            println!("{err_str}");
            process::exit(1);
        });

    if let Err(err) = minigrep::run(config) {
        println!("{err}");
        process::exit(1);
    }
}
