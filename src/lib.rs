// imports
use std::{fs, env, error::Error};

// allows the Config struct to be cloned
#[derive (Clone)]
pub struct Config {
    pub input_file: String,
    pub direction: String,
    pub key: String,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        // gets the input file from the command line
        let input_file = match args.next() {
            Some(arg) => arg,
            // if there is no input file given, return the error with the usage
            None => return Err("No input file given- Usage: cargo run <input file> <e/d> <key>"),
        };

        // gets the direction for encryption or decryption
        let direction = match args.next() {
            Some(arg) => arg,
            // if no direction is given, return the error with the usage
            None => return Err("Not enough information- Usage: cargo run <input file> <e/d> <key>"),
        };

        // gets the key for the cipher
        let key = match args.next() {
            Some(arg) => arg,
            // if no key is given, return the error with the usage
            None => return Err("Not enough information- Usage: cargo run <input file> <e/d> <key>"),
        };

        // return the Config if there are no errors
        Ok(Config {
            input_file, 
            direction,
            key,
        })
    }
}

pub fn run(config: Config) -> Result<String, Box<dyn Error>> {
    // reads the file contents and ? returns an error if it cannot read it
    let contents = fs::read_to_string(config.input_file)?;
    Ok(contents)
}