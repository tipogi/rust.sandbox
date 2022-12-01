// Root of the library crate

use std::error::Error;
use std::fs;
use std::env;

// Make all the fn, struct... public because in Rust everything by default is private

// Return unitype which is nothing and in error case an error
// The error type would be anytype
pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
    // If we get error it will return automatically
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results
    {
        println!("{}", line)
    }
    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &str> {
        if args.len() < 3 {
            // We use panic error, when we have programing error
            //panic!("Not enough arguments, add at list two arguments");
            // But in that case, we have to use usage error type
            return Err("Not enough arguments, add at list two arguments");
        }
        // Ignore, first argument because it would be the path which the binary is located
        //let query = args[1].clone();
        //let file = &args[2].clone();
        let query = &args[1];
        let file = &args[2];

        // Add to the system env variables: export CASE_INSENSITIVE=true
        // To delete from environment: unset CASE_INSENSITIVE
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        
        println!("Searching for {} word", query);
        println!("In {} file", file);
    
        Ok(Config { 
            query: String::from(query), 
            filename: String::from(file),
            case_sensitive
        })
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(
    query: &str, 
    contents: &'a str
) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive.", "Duct tape."], search_case_insensitive(query, contents));
    }
}
  