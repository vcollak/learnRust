use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    //build the config from arguments and return the a Result
    //with either Config or Error
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments.\nUsage: minigrep query file");
        }

        //if there is an environment variable called IGNORE_CASE the
        //set ignore_case is set to true.
        //env::var returns a  Result<String, VarError>
        //is_ok returns true is the Result is Ok
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        //clones the values since args were borrowed from main and need to
        //return in Config easier to just clone to create new
        //strings that can be returned even though it's not the most optimal thing
        //from a performance standpoint

        let query = args[1].clone();
        let file_path = args[2].clone();

        //returns the config struct
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })

        /*

        //Alternative way to implement where we take a reference to
        //args. We then use to_string to convert from &str to String
        let query = &args[1];
        let file_path = &args[2];

        //returns the config struct
        //this uses to_string to convert from &str to String
        Ok(Config {
            query: query.to_string(),
            file_path: file_path.to_string(),
            ignore_case,
        })

         */
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    //? will either return the Box<dyn Error> or contents of the file
    let contents = fs::read_to_string(config.file_path)?;

    //foe debugging purposes it prints the content of the file
    //println!("With text:\n{contents}");

    //two ways to search depending on config.ignore_case
    //the Vec<&str> is returned by the search functions
    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

//since we're using a reference (borrowing) from contents
//when we push values to results, we need to link the return
//value to contents using lifetimes
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    println!("Case sensitive");

    let mut results = Vec::new();

    //iterate through the contents of the file
    for line in contents.lines() {
        //is line containes our query push reference to results
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

//really same as search, but we make this case insensitive
//by lowercasing the content and query
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    println!("Case insensitive");

    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

//tests our implementation
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
