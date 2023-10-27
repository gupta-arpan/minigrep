use std::fs;
use std::error::Error;
use std::env;

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Too few arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {query, file_path, ignore_case})

    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {

    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    }else {
        search(&config.query, &contents)
    };
 
    for line in results{
        println!("{line}");
    }
    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for lines in contents.lines() {
        if lines.contains(query) {
            result.push(lines);
        }
    }

    result
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut result = Vec::new();
    for lines in contents.lines() {
        if lines.to_lowercase().contains(&query) {
            result.push(lines);
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query  = "duct";
        let contents = "\
Rust:
safe, fast, production.
Pick three.
Duct Tape.";

        assert_eq!(vec!["safe, fast, production."],search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query  = "ruSt";
        let contents = "\
Rust:
safe, fast, production.
Pick three.
Trust me.";

        assert_eq!(vec!["Rust:","Trust me."],search_case_insensitive(query, contents));
    }


}