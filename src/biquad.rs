
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

pub struct Biquad {

}

pub struct AudioFilter {
    parameters: AudioFilterParameters,
    biquad: Biquad,
    coeff_array: Vec<f64>,
    sample_rate: f64,
}

impl AudioFilter {
    pub fn get_params(&self) -> AudioFilterParameters {
        self.parameters // change to remove copy
    }

    pub fn set_params(&mut self, params: AudioFilterParameters) {
        self.parameters = params;
    }

    pub fn reset(&self) {

    }

    pub fn process_audio_sample(&self, xn: f64) {//-> f64 {

    }

    fn set_sample_rate(&mut self, sample_rate: f64) {
        self.sample_rate = sample_rate;
    }
}