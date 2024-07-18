use std::{env, error::Error, fs};
use hound;

pub mod biquad;

pub struct Config {
    pub file_path: String,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        // args.next();

        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        Ok(Config {
            file_path,
        })
    }
}

pub fn load_file(config: Config) -> Result<Vec<f64>, Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let mut reader = hound::WavReader::open("square_220.wav").expect("Couldn't find file");
}