use std::error::Error;
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

pub fn load_file(file_path: &str) -> Result<Vec<i16>, Box<dyn Error>> {
    let mut reader = hound::WavReader::open(file_path).expect("Couldn't find file");

    let samples = reader.samples::<i16>();

    let mut vector = Vec::<i16>::new();
    for sample in samples {
        let val = sample.expect("Could not read sample");
        vector.push(val);
    };

    Ok(vector)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rms_ne_zero() {
        let mut reader = hound::WavReader::open("square_220.wav").expect("Could not read file");

        let sqr_sum = reader.samples::<i16>()
            .fold(0.0, |sqr_sum, s| {
                let sample = s.unwrap() as f64;
                sqr_sum + sample * sample
            });
        
        let rms = (sqr_sum / reader.len() as f64).sqrt();
        
        assert_ne!(0.0, rms);
    }
}