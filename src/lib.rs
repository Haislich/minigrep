use std::{fs, env};
use std::error::Error;

pub struct Config{
    pub pattern: String,
    pub filename: String,
    pub case_sensitive: bool
}
impl Config{
    pub fn new(args: &[String]) -> Result<Config,&str>{
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
pub fn run(config:Config)-> Result<(),Box<dyn Error>>{
    let contents = fs::read_to_string(config.filename)?;
    let results  = match config.case_sensitive{
        true=>search(&config.pattern, &contents),
        false=>search_case_insensitive(&config.pattern, &contents)
    };
    for line in results{
        println!("{line}")
    }
    Ok(())
}
pub fn search<'a>(pattern: &str, contents:&'a str)-> Vec<&'a str>{
    let mut results = Vec::new();
    for line in contents.lines(){
        if line.contains(pattern){
            results.push(line);
        };
    };
    results
}
pub fn search_case_insensitive<'a>(pattern: &str, contents:&'a str) -> Vec<&'a str>{
    let pattern = pattern.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines(){
        if line.to_lowercase().contains(&pattern){
            results.push(line);
        }
    }
    results
}