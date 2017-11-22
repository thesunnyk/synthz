
extern crate rand;
use self::rand::random;
use std::f32;

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

struct DataIn {
    v: Option<Vec<f32>>,
    default: f32
}

impl DataIn {
    fn new(default: f32) -> DataIn {
        DataIn { v: None, default: default }
    }

    // TODO actually return the iterator
    fn next(&mut self) -> f32 {
        self.v.as_mut().map(|v| *v.iter().next().expect("expected more data")).unwrap_or(self.default)
    }

}

impl module::Input for DataIn {
    fn feed(&mut self, v: Vec<f32>) {
        self.v = Some(v);
    }
}

pub struct Oscillator {
    w: f32,
    rate: f32,
    primary: Waveform,
    freq_in: DataIn,
    duty_cycle_in: DataIn,
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
            freq_in: DataIn::new(0.0),
            duty_cycle_in: DataIn::new(0.5),
        };
        ret
    }

    pub fn oscillate(&mut self) -> f32 {
        let freq = Oscillator::get_freq(self.freq_in.next(), self.rate);
        let res = self.primary.oscillate(self.w, freq, self.duty_cycle_in.next());
        // TODO Use omega not t.
        self.w = self.w + 1.0;
        res
    }
}

impl module::Output for Oscillator {
    fn extract(&mut self, len: usize) -> Vec<f32> {
        let mut ret = Vec::<f32>::with_capacity(len);
        for i in 0..len {
            ret.push(self.oscillate());
        }
        ret
    }
}

impl module::Module for Oscillator {

    fn inputs(&mut self) -> Vec<&mut module::Input> {
        vec!(&mut self.freq_in, &mut self.duty_cycle_in)
    }

    fn outputs(&mut self) -> Vec<&mut module::Output> {
        vec!(self)
    }
}


impl Iterator for Oscillator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        Some(self.oscillate())
    }

}

