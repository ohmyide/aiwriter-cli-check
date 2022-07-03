use std::error::Error;
use std::fs;
use std::env;

pub struct Config {
    pub filename: String,
    pub query: String,
    pub case_sensitive: bool
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(
    query: &str,
    contents: &'a str,
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


impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }

    pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
        let contents = fs::read_to_string(config.filename)?;
        let results = if config.case_sensitive {
            search(&config.query, &contents)
        } else {
            search_case_insensitive(&config.query, &contents)
        };
    
        for line in results {
            println!("{}", line);
        }
    
        // search("aa", contents);
        println!("With text:\n{}", contents);
        Ok(())
    }

    

}