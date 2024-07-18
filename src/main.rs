// use std::{env, process};
use hound;
use rs_rust_audio::{self, biquad};

fn main() {
    let mut filter = biquad::AudioFilter::new();
    // let mut params = biquad::AudioFilterParameters::new();

    filter.calculate_filter_coeffs();

    let mut dest_vec = Vec::<f64>::new();

    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    
    let file_vec: Vec<i16> = rs_rust_audio::load_file("this_is_a_test.wav").expect("Could not read file");

    for sample in file_vec {
        let float_samp = sample as f64;

        let filtered_samp = filter.process_sample(float_samp);

        dest_vec.push(filtered_samp);
    }

    let mut writer = hound::WavWriter::create("filtered_speech.wav", spec).expect("Could not create writer");

    for t in 0..dest_vec.len() {
        writer.write_sample(dest_vec[t] as i16).expect("Could not write sample");
    }
    writer.finalize().expect("Could not finalize WAV file");
}

