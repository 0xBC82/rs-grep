use std::{env, fs};
use std::error::Error;

pub fn run(config: &Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        // Skip initial arg.
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a query")
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("didn't get a filename")
        };

        // `is_err` checks `CASE_INSENSITIVE` is not set.
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        let config = Config { query, filename, case_sensitive };

        Ok(config)
    }
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "a";
        let contents = "foo\nbar\nbaz";
        assert_eq!(vec!["bar", "baz"], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "foo";
        let contents = "Foo\nBar\nBaz";
        assert_eq!(vec!["Foo"], search_case_insensitive(query, contents));
    }
}