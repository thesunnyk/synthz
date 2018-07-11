
extern crate rand;
use self::rand::random;
use std::f32;
use std::iter::Cycle;
use std::slice::Iter;

use synth::module;

// TODO We could try smoothly mixing between the waves.
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

    fn from_data(data: f32) -> Waveform {
        match (data * 5.0) as i32 {
            x if x == Waveform::Sine as i32 => Waveform::Sine,
            x if x == Waveform::Square as i32 => Waveform::Square,
            x if x == Waveform::Sawtooth as i32 => Waveform::Sawtooth,
            x if x == Waveform::Triangle as i32 => Waveform::Triangle,
            x if x == Waveform::Noise as i32 => Waveform::Noise,
            _ => Waveform::Sine
        }
    }

    fn oscillate(&self, t: f32, f: f32, fm: f32, d: f32) -> f32 {
        let ftfm = f * (t + 10.0 * fm);
        match self {
            &Waveform::Sine => {
                let omega = 2.0 * f32::consts::PI;
                f32::sin(ftfm * omega)
            },
            &Waveform::Square => {
                if ftfm % 1.0 < d {
                    0.5
                } else {
                    -0.5
                }
            },
            &Waveform::Sawtooth => {
                2.0 * (ftfm % 1.0) - 1.0
            },
            &Waveform::Triangle => {
                let saw = 2.0 * ((2.0 * ftfm) % 1.0);
                if ftfm % 1.0 < 0.5 {
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
    t: f32,
    rate: f32,
}

impl Oscillator {
    fn get_freq(note: f32, rate: f32) -> f32 {
        let pitch = (note * 127.0) - 69.0;
        let freq_hz = (2.0 as f32).powf(pitch/12.0) * 440.0;
        freq_hz / rate
    }

    pub fn new(rate: f32) -> module::MisoModule<Oscillator> {
        module::MisoModule::new(Oscillator {
            t: 0.0,
            rate,
        })
    }

    pub fn oscillate(&mut self, primary: f32, note: f32, fm_in: f32, duty_cycle_in: f32) -> f32 {
        let freq = Oscillator::get_freq(note, self.rate);
        let wave = Waveform::from_data(primary);
        let res = wave.oscillate(self.t, freq, fm_in, duty_cycle_in);
        self.t = self.t + 1.0;
        res
    }
}

impl module::MisoWorker for Oscillator {
    fn get_data(&self) -> Vec<module::DataIn> {
        vec![
            module::DataIn::new(String::from("primary"), 0.0),
            module::DataIn::new(String::from("freq_in"), 0.0),
            module::DataIn::new(String::from("fm_in"), 0.0),
            module::DataIn::new(String::from("duty_cycle_in"), 0.5),
        ]
    }

    fn extract(&mut self, vals: Vec<f32>) -> f32 {
        self.oscillate(vals[0], vals[1], vals[2], vals[3])
    }
}


