use std::{f64::consts::PI, mem};

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
        AudioFilterParameters {
            algorithm: FilterAlgorithm::Lpf2,
            fc: 750.0,
            q: 3.0,
            boost_cut_db: 0.0,
        }
    }
}

pub struct Biquad {
    coeff_array: Vec<f64>,
    state_array: Vec<f64>,
}

impl Biquad {
    pub fn new() -> Biquad {
        Biquad {
            coeff_array: vec![0.0; 7],
            state_array: vec![0.0; 4]
        }
    }

    // pub fn get_params() {}

    // pub fn set_params() {}

    pub fn reset() {}

    pub fn process_sample(&mut self, xn: f64) -> f64 {
        // canonical form only
        let wn = xn - (self.coeff_array[3] * self.state_array[0]) - (self.coeff_array[4] * self.state_array[1]);

        let yn = self.coeff_array[0] * wn + self.coeff_array[1] * self.state_array[0] + self.coeff_array[2] * self.state_array[1];

        self.state_array[1] = self.state_array[0];
        self.state_array[0] = wn;

        yn
    }

    // pub fn set_coeffs(&mut self, coeff_array: Vec<f64>) {
    //     self.coeff_array = coeff_array; // replace w/ Rc
    // }

    // pub fn get_coeffs(&self) -> Vec<f64> {
    //     self.coeff_array.clone() // replace w/ Rc
    // }

    // pub fn get_state_array(&self) -> Vec<f64> {
    //     self.coeff_array.clone() // replace w/ Rc
    // }
}

pub struct AudioFilter {
    parameters: AudioFilterParameters,
    biquad: Biquad,
    coeff_array: Vec<f64>,
    // num_coeffs: i32,
    sample_rate: f64,
}

impl AudioFilter {
    pub fn new() -> AudioFilter {
        let num_coeffs: i32 = 7;

        AudioFilter {
            parameters: AudioFilterParameters::new(),
            biquad: Biquad::new(),
            coeff_array: vec![0.0; num_coeffs as usize],
            // num_coeffs: num_coeffs,
            sample_rate: 44100.0,
        }
    }

    pub fn get_params(&self) -> AudioFilterParameters {
        self.parameters // does this move?
    }

    pub fn set_params(&mut self, params: AudioFilterParameters) {
        self.parameters = params;

        if self.parameters.q <= 0.0 {
            self.parameters.q = 0.707;
        }

        self.calculate_filter_coeffs();
    }

    pub fn reset(&self) {}

    pub fn process_sample(&mut self, xn: f64) -> f64 {
        self.biquad.coeff_array[6] * xn + self.biquad.coeff_array[5] * self.biquad.process_sample(xn) // coeff_array[d0] and c0
    }

    pub fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
    }

    pub fn calculate_filter_coeffs(&mut self) {
        self.coeff_array.fill(0.0);

        self.coeff_array[0] = 1.0; // a0
        self.coeff_array[5] = 1.0; // c0
        self.coeff_array[6] = 0.0; // d0

        let filter_algorithm = self.parameters.algorithm;
        let fc = self.parameters.fc;
        let q = self.parameters.q;
        // let boost_cut_db = self.parameters.boost_cut_db;

        if filter_algorithm == FilterAlgorithm::Lpf1 {
            let theta_c = 2.0 * PI * fc / self.sample_rate;
            let gamma = f64::cos(theta_c) / (1.0 + f64::sin(theta_c));

            self.coeff_array[0] = (1.0 - gamma) / 2.0; // a0
            self.coeff_array[1] = (1.0 - gamma) / 2.0; // a1
            self.coeff_array[2] = 0.0;                 // a2
            self.coeff_array[3] = -gamma;              // b1
            self.coeff_array[4] = 0.0;                 // b2

            mem::swap(&mut self.biquad.coeff_array, &mut self.coeff_array);
            
        } else if filter_algorithm == FilterAlgorithm::Lpf2 {
            let theta_c = 2.0 * PI * fc / self.sample_rate;
            let d = 1.0 / q;
            let beta_numerator = 1.0 - ((d / 2.0) * (f64::sin(theta_c)));
            let beta_denominator = 1.0 + ((d / 2.0) * f64::sin(theta_c));

            let beta = 0.5 * (beta_numerator / beta_denominator);
            let gamma = (0.5 + beta) * (f64::cos(theta_c));
            let alpha = (0.5 + beta - gamma) / 2.0;

            self.coeff_array[0] = alpha;        // a0
            self.coeff_array[1] = 2.0 * alpha;  // a1
            self.coeff_array[2] = alpha;        // a2
            self.coeff_array[3] = -2.0 * gamma; // b1
            self.coeff_array[4] = 2.0 * beta;   // b2

            mem::swap(&mut self.biquad.coeff_array, &mut self.coeff_array);
        }
    }
}