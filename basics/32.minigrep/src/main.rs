use std::env;
use std::process;

//bring in the config struct from lib.rs
//the library has to be in the lib.rs file
use minigrep::Config;

fn main() {
    //put the arguments in the vector of strings
    //env::args() returns an iterator
    //collect() transforms an iterator into a collection.
    let args: Vec<String> = env::args().collect();

    //build config. unwrap_or_else does this:
    //  if Ok, put the result in config
    //  if Err, call closure and pass err
    let config = Config::build(&args).unwrap_or_else(|err| {
        //prints to stderr instead stdout
        eprintln!("Problem Parsing arguments: {err}");

        //exist the program
        process::exit(1);
    });

    //print what we're searching for and in which file
    println!("Searching for {}", config.query);
    println!("In file {}", config.file_path);

    //run either returns nothing or an error
    //so we don't need to unwrap_or_else like we did
    //with config. just check for an error
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application Error: {e}");
        //exist the program
        process::exit(1);
    }
}
