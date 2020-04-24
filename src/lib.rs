use std::env;
use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(
                "Wrong amount of arguments, expected 2 (usage: minigrep <query> <filename>)",
            );
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        // get case_sensitive from env vars and cli args, with cli args taking precedence
        let mut case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        if let Some(case_sensitive_cli_var) = args.get(3) {
            case_sensitive = match case_sensitive_cli_var.parse() {
                Ok(val) => val,
                Err(_) => {
                    return Err("Failed to parse case sensitive cli arg");
                }
            };
        }

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;
    let search_function = match config.case_sensitive {
        true => search,
        false => search_case_insensitive,
    };

    for line in search_function(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines: Vec<&str> = vec![];
    for line in contents.lines() {
        if line.contains(query) {
            matching_lines.push(line);
        }
    }

    matching_lines
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut matching_lines = Vec::new();
    let query = &query.to_lowercase();

    for line in contents.lines() {
        if line.to_lowercase().contains(query) {
            matching_lines.push(line);
        }
    }

    matching_lines
}

mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn config_check_error() {
        Config::new(&vec![String::from("test")]).unwrap();
    }

    #[test]
    fn config_check_success() {
        Config::new(&vec![
            String::from("test"),
            String::from("very"),
            String::from("nice"),
        ])
        .unwrap();

        Config::new(&vec![
            String::from("test"),
            String::from("very"),
            String::from("nice"),
            String::from("true"),
        ])
        .unwrap();
    }

    #[test]
    fn run_check() {
        run(Config::new(&vec![
            String::from("test"),
            String::from("query"),
            String::from("poem.txt"),
        ])
        .unwrap())
        .unwrap();
    }

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
