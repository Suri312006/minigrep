use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

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

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        //NOTE:  dont be scared to clone

        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

//NOTE: need to specify lifetime parameter here because
//we need to specify which argument lifetime is connected to the lifetime
//of the return value. The pointer we return will be a pointer to
//a string slice of contents, so their lifetimes must match
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut hits = Vec::new();

    for line in contents.lines() {
        // even though line is a pointer to a string, it gets deferenced
        if line.contains(query) {
            hits.push(line);
        }
    }
    hits
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut hits = Vec::new();
    let query = query.to_lowercase();
    for line in contents.lines() {
        // even though line is a pointer to a string, it gets deferenced
        if line.to_lowercase().contains(&query) {
            hits.push(line);
        }
    }
    hits
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape";

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
