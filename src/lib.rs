use std::{fs, env};
use std::error::Error;

pub struct Config{
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool
}
impl Config{
    pub fn new(args: &[String]) -> Result<Config,&str>{
        // Constructor for Config struct, it returns a Result containing
        // an initialized Config or a an error explaining why it couldn't be initialized
        match args.len(){
            3 => (),
            _ => return Err("Usage: minigrep pattern file")
        };
        let pattern = args[1].clone();
        let filename = args[2].clone();
        let case_sensitive = env::var("CASE_SENSITIVE").is_err();
        Ok(Config {pattern,filename,case_sensitive})
    } 
}
// The dyn Error is used because we're propagating the error from the read_to_string
// the problem is that we don't know at compile time what error might be.
// For example it could be that the file cannot be read, the file doesn't exists or the
// characters are not all utf-8 vlaid characters
pub fn run(config:Config)-> Result<(),Box<dyn Error>>{
    // Function that does that prints if there are matches.

    // The ? operator returns the error if encountered, else continues 
    let contents = fs::read_to_string(config.filename)?;
    let results  = match config.case_sensitive{
        true  => search(&config.pattern, &contents),
        false => search_case_insensitive(&config.pattern, &contents)
    };
    for line in results{
        println!("{line}")
    }
    Ok(())
}
fn search<'a>(pattern: &str, contents:&'a str)-> Vec<&'a str>{
    let mut results = Vec::new();
    // More functional approach
    contents.lines().for_each(|line| {
        if line.contains(pattern){
            results.push(line)}
        });
    results
}
fn search_case_insensitive<'a>(pattern: &str, contents:&'a str) -> Vec<&'a str>{
    let pattern = pattern.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    // More 'standard' approach
    for line in contents.lines(){
        if line.to_lowercase().contains(&pattern){
            results.push(line);
        }
    }
    results
}