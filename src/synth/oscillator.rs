
extern crate rand;
use self::rand::random;
use std::f32;

#[derive(Debug)]
#[derive(Clone)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    Noise
}


#[derive(Debug)]
#[derive(Clone)]
pub enum WaveType {
    Sine(f32),
    Square(f32, f32),
    Sawtooth(f32),
    Triangle(f32),
    Noise,
}

impl WaveType {

    pub fn from_waveform(form: Waveform, freq: f32) -> WaveType {
        match form {
            Waveform::Triangle => {
                WaveType::Triangle(freq)
            },
            Waveform::Square => {
                // TODO Wire up square wave width
                WaveType::Square(freq, 0.5)
            },
            Waveform::Sine => {
                WaveType::Sine(freq)
            },
            Waveform::Sawtooth => {
                WaveType::Sawtooth(freq)
            },
            Waveform::Noise => {
                WaveType::Noise
            },
        }
    }
    fn oscillate(&self, t: f32) -> f32 {
        match self {
            &WaveType::Sine(f) => {
                let omega = 2.0 * f32::consts::PI;
                f32::sin(f * t * omega)
            },
            &WaveType::Square(f, d) => {
                if (f * t) % 1.0 < d {
                    0.5
                } else {
                    -0.5
                }
            },
            &WaveType::Sawtooth(f) => {
                2.0 * ((f * t) % 1.0) - 1.0
            },
            &WaveType::Triangle(f) => {
                let out = f * t;
                let saw = 2.0 * ((2.0 * out) % 1.0);
                if out % 1.0 < 0.5 {
                    saw - 1.0
                } else {
                    1.0 - saw
                }
            },
            &WaveType::Noise => {
                random::<f32>() * 2.0 - 1.0
            }
        }
    }
}

pub struct Oscillator {
    rate: f32,
    primary: WaveType,
}

impl Oscillator {
    fn get_freq(pitch: f32, rate: f32) -> f32 {
        // TODO Pitch should be between 0 and 1
        let freq_hz = (2.0 as f32).powf(pitch) * 440.0;
        freq_hz / rate
    }

    pub fn new(rate: f32) -> Oscillator {
        Oscillator {
            rate: rate,
            primary: WaveType::Sine(0.0),
        }
    }

    pub fn config(&mut self, form: Waveform, pitch: f32) {
        let freq = Oscillator::get_freq(pitch, self.rate);
        self.primary = WaveType::from_waveform(form, freq);
    }

    pub fn oscillate(&mut self, t: i64) -> f32 {
        // TODO Use omega not t.
        self.primary.oscillate(t as f32)
    }
}

