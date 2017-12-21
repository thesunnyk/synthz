
extern crate rand;
use self::rand::random;
use std::f32;
use std::iter::Cycle;
use std::slice::Iter;

use synth::module;

#[derive(Debug)]
#[derive(Clone)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
    Triangle,
    Noise
}

impl Waveform {

    fn oscillate(&self, t: f32, f: f32, d: f32) -> f32 {
        match self {
            &Waveform::Sine => {
                let omega = 2.0 * f32::consts::PI;
                f32::sin(f * t * omega)
            },
            &Waveform::Square => {
                if (f * t) % 1.0 < d {
                    0.5
                } else {
                    -0.5
                }
            },
            &Waveform::Sawtooth => {
                2.0 * ((f * t) % 1.0) - 1.0
            },
            &Waveform::Triangle => {
                let out = f * t;
                let saw = 2.0 * ((2.0 * out) % 1.0);
                if out % 1.0 < 0.5 {
                    saw - 1.0
                } else {
                    1.0 - saw
                }
            },
            &Waveform::Noise => {
                random::<f32>() * 2.0 - 1.0
            }
        }
    }
}

pub struct Oscillator {
    w: f32,
    rate: f32,
    primary: Waveform,
    freq_in: module::DataIn,
    duty_cycle_in: module::DataIn,
}

impl Oscillator {
    fn get_freq(pitch: f32, rate: f32) -> f32 {
        // TODO Pitch should be between 0 and 1
        let freq_hz = (2.0 as f32).powf(pitch) * 440.0;
        freq_hz / rate
    }

    pub fn new(rate: f32) -> Oscillator {
        let ret = Oscillator {
            w: 0.0,
            rate,
            primary: Waveform::Sine,
            freq_in: module::DataIn::new(0.0),
            duty_cycle_in: module::DataIn::new(0.5),
        };
        ret
    }

    pub fn oscillate(&mut self, freq_in: f32, duty_cycle_in: f32) -> f32 {
        let freq = Oscillator::get_freq(freq_in, self.rate);
        let res = self.primary.oscillate(self.w, freq, duty_cycle_in);
        // TODO Use omega not t.
        self.w = self.w + 1.0;
        res
    }
}

impl module::Module for Oscillator {
    fn feed(&mut self, offset: usize, v: Vec<f32>) {
        assert!(offset == 0);
        match offset {
            0 => self.freq_in.set(v),
            1 => self.duty_cycle_in.set(v),
            _ => panic!("Invalid input")
        }
    }

    fn extract(&mut self, offset: usize, len: usize) -> Vec<f32> {
        let mut ret = Vec::<f32>::with_capacity(len);
        let v = self.freq_in.get();
        let mut it = v.iter().cycle();

        for i in 0..len {
            // TODO duty_cycle_in.iter();
            let freq: f32 = *it.next().expect("Expected more data");
            ret.push(self.oscillate(freq, 0.0));
        }
        ret
    }
}


