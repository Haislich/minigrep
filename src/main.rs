use std::env;
use std::process;
use minigrep::{Config,run};


fn main() {
    // Get arguments from input and store them as a vector of string.
    // after using collect Rust has no way of knowing wich type of vector 
    // we want so we need to help it, assigning manually the value.
    let args:Vec<String> = env::args().collect();

    // Te constructor for the Config struct returns a Result.
    // Using the unwrap or else we are saying unwrap (extract the value from Ok),
    // if the value is not ok then perform some operation on the error.
    let config = Config::new(&args).unwrap_or_else(|err| {
        // eprintln!() is a macro that redirects its output to the standard error
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });
    if let Err(e) = run(config){
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}


