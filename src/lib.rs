use std::env;
use std::error::Error;
use std::fs;

pub struct ArgsVal {
    pub query: String,
    pub filep: String,
    pub ignore_case: bool,
}

impl ArgsVal {
    pub fn new(args: &[String]) -> Result<ArgsVal, &'static str> {
        if args.len() < 3 {
            return Err("Need 2 arguments");
        }
        let query = args[1].clone();
        let filep = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(ArgsVal {
            query,
            filep,
            ignore_case,
        })
    }
}

pub fn run(args: ArgsVal) -> Result<(), Box<dyn Error>> {
    let f_contents = fs::read_to_string(args.filep)?;

    let result = if args.ignore_case {
        search_case_insensitive(&args.query, &f_contents)
    } else {
        search(&args.query, &f_contents)
    };

    for line in result {
        println!("{line}");
    }
    Ok(())
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
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
mod tests {
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
