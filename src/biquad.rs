use std::{f64::consts::PI};


#[derive(Copy, Clone, PartialEq)]
pub enum FilterAlgorithm {
    Lpf1P,
    Lpf1,
    Hpf1,
    Lpf2,
    Hpf2,
    Bpf2,
    Bsf2,
}

#[derive(Copy, Clone, PartialEq)]
pub struct AudioFilterParameters {
    algorithm: FilterAlgorithm,
    fc: f64,
    q: f64,
    boost_cut_db: f64,
}

impl AudioFilterParameters {
    pub fn new() -> AudioFilterParameters {

    }
}

pub struct Biquad {
    coefficients_array: Vec<f64>,
}

impl Biquad {
    pub fn new() -> Biquad {

    }

    pub fn set_coefficients(&mut self, coefficients_array: Vec<f64>) {
        self.coefficients_array = coefficients_array;
    }
}

pub struct AudioFilter {
    parameters: AudioFilterParameters,
    biquad: Biquad,
    coeff_array: Vec<f64>,
    num_coeffs: i32,
    sample_rate: f64,
}

impl AudioFilter {
    pub fn new() -> AudioFilter {
        AudioFilter {
            parameters: AudioFilterParameters::new(),
            biquad: Biquad::new(),
            coeff_array: vec![0.0; 7usize],
            num_coeffs: 7,
            sample_rate: 44100.0,
        }
    }

    pub fn get_params(&self) -> AudioFilterParameters {
        self.parameters // change to remove copy
    }

    pub fn set_params(&mut self, params: AudioFilterParameters) {
        self.parameters = params;
    }

    pub fn reset(&self) {

    }

    pub fn process_audio_sample(&mut self, xn: f64) {//-> f64 {


    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
    }

    fn calculate_filter_coeffs(&mut self) {
        self.coeff_array.fill(0.0);

        self.coeff_array[0] = 1.0; // a0
        self.coeff_array[5] = 1.0; // c0
        self.coeff_array[6] = 0.0; // d0

        let filter_algorithm = self.parameters.algorithm;
        let fc = self.parameters.fc;
        let q = self.parameters.q;
        let boost_cut_db = self.parameters.boost_cut_db;

        if filter_algorithm == FilterAlgorithm::Lpf1 {
            let theta_c = 2.0 * PI * fc / self.sample_rate;
            let gamma = f64::cos(theta_c) / (1.0 + f64::sin(theta_c));

            self.coeff_array[0] = (1.0 - gamma) / 2.0; // a0
            self.coeff_array[1] = (1.0 - gamma) / 2.0; // a1
            self.coeff_array[2] = 0.0;                 // a2
            self.coeff_array[3] = -gamma;              // b1
            self.coeff_array[4] = 0.0;                 // b2
            
            self.biquad.set_coefficients(self.coeff_array.clone()); // used clone, can improve
            
        } else if filter_algorithm == FilterAlgorithm::Lpf2 {

        }
    }
}