use std::{env, process};
use hound::Sample;
use rs_rust_audio::{load_file, Config};
use rs_rust_audio::biquad::{AudioFilter, AudioFilterParameters};

fn main() {
    let mut filter = AudioFilter::new();
    let mut params = AudioFilterParameters::new();

    filter.calculate_filter_coeffs();

    let mut dest_vec = Vec::<f64>::new();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let file_vec: Vec<i16> = load_file("square_220.wav").expect("Could not read file");

    for sample in file_vec {
        let float_samp = sample as f64;

        let filtered_samp = filter.process_sample(float_samp);

        dest_vec.push(filtered_samp);
    }

    let mut writer = hound::WavWriter::create("filtered_sqr.wav", spec).expect("Could not create writer");

    for t in 0..44100 {
        writer.write_sample(dest_vec[t] as i16).expect("Could not write sample");
    }
    writer.finalize().expect("Could not finalize WAV file");
}

