use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub filename: String,
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

        Ok(Config { query, filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    Ok(())
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
            String::from("great"),
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
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}