use std::env;
use std::process;

//bring in the config struct from lib.rs
use minigrep::Config;

fn main() {
    //put the arguments in the vector of strings
    let args: Vec<String> = env::args().collect();

    //build config. if Ok, put the result in config
    //if Err, call closure and pass err
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem Parsing arguments: {err}");

        //exist the program
        process::exit(1);
    });

    //println!("Searching for {}", config.query);
    //println!("In file {}", config.file_path);

    //run either returns nothing or an error
    //so we don't need to unwrap_or_else like we did
    //with config. just check for an error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        //exist the program
        process::exit(1);
    }
}
