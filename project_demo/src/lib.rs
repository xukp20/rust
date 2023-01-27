use std::{fs, error::Error, env};

pub fn run(config: Config) -> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    println!("Results:");
    
    for line in results {
        println!("{}", line);
    }

    Ok(())
}

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static  str> {
        if args.len() < 3 {
            return Err("missing parameters");
        }

        let query = args[1].clone();
        let filename = args[2].clone();
        
        // get case_sensitive from the env
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err(); // if no this env val
        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn search<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query:&str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    let query = query.to_lowercase();
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
        Rust: productive.
        Hello world.
        Duct.";

        assert_eq!(1, search(query, contents).len())
    }

    #[test]
    fn case_insensitive() {
        let query = "rust";
        let contents = "\
        Rust: productive.
        Hello world.
        Duct.
        rust!";

        assert_eq!(2, search_case_insensitive(query, contents).len())
    }
}