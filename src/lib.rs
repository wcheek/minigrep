use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
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
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn multiple_results() {
        let query = "body";
        let contents = fs::read_to_string("poem.txt").unwrap();

        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!"
            ],
            search(query, &contents)
        );
    }

    #[test]
    fn no_results() {
        let query = "metamorphosis";
        let contents = fs::read_to_string("poem.txt").unwrap();

        assert_eq!(vec![] as Vec<&str>, search(query, &contents));
    }

    #[test]
    fn one_result_case_insensitive() {
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

    #[test]
    fn multiple_results_case_insensitive() {
        let query = "BOdy";
        let contents = fs::read_to_string("poem.txt").unwrap();

        assert_eq!(
            vec![
                "I'm nobody! Who are you?",
                "Are you nobody, too?",
                "How dreary to be somebody!"
            ],
            search_case_insensitive(query, &contents)
        );
    }

    #[test]
    fn no_results_case_insensitive() {
        let query = "metAMorphosis";
        let contents = fs::read_to_string("poem.txt").unwrap();

        assert_eq!(
            vec![] as Vec<&str>,
            search_case_insensitive(query, &contents)
        );
    }
}
