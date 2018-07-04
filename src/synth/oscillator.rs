
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
    primary: module::DataIn,
    freq_in: module::DataIn,
    duty_cycle_in: module::DataIn,
    fm_in: module::DataIn,
    pos: usize
}

pub enum OscillatorInput {
    FreqIn,
    DutyCycleIn,
    FmIn,
    Primary,
}

impl Oscillator {
    fn get_freq(note: f32, rate: f32) -> f32 {
        let pitch = (note * 127.0) - 69.0;
        let freq_hz = (2.0 as f32).powf(pitch/12.0) * 440.0;
        freq_hz / rate
    }

    pub fn new(rate: f32) -> Oscillator {
        let ret = Oscillator {
            t: 0.0,
            rate,
            primary: module::DataIn::new(0.0),
            freq_in: module::DataIn::new(0.0),
            duty_cycle_in: module::DataIn::new(0.5),
            fm_in: module::DataIn::new(0.0),
            pos: 0,
        };
        ret
    }

    pub fn oscillate(&mut self, primary: f32, note: f32, fm_in: f32, duty_cycle_in: f32) -> f32 {
        let freq = Oscillator::get_freq(note, self.rate);
        let wave = Waveform::from_data(primary);
        let res = wave.oscillate(self.t, freq, fm_in, duty_cycle_in);
        self.t = self.t + 1.0;
        res
    }

    pub fn connector_in(&self, input: OscillatorInput) -> module::Connector {
        module::Connector {
            mod_in: self.pos,
            offset: input as usize
        }
    }

    pub fn connector_out(&self) -> module::Connector {
        module::Connector {
            mod_in: self.pos,
            offset: 0
        }
    }
}

impl module::Module for Oscillator {
    fn initialise(&mut self, pos: usize) {
        self.pos = pos;
    }

    fn feed(&mut self, offset: usize, v: Vec<f32>) {
        match offset as OscillatorInput {
            FreqIn => self.freq_in.set(v),
            DutyCycleIn => self.duty_cycle_in.set(v),
            FmIn => self.fm_in.set(v),
            Primary => self.primary.set(v),
            _ => panic!("Invalid input")
        }
    }

    fn extract(&mut self, offset: usize, len: usize) -> Vec<f32> {
        assert_eq!(offset, 0);
        let mut ret = Vec::<f32>::with_capacity(len);
        let v = self.freq_in.get();
        let d = self.duty_cycle_in.get();
        let f = self.fm_in.get();
        let p = self.primary.get();
        let mut it = v.iter().cycle();
        let mut dit = d.iter().cycle();
        let mut fmit = f.iter().cycle();
        let mut pit = p.iter().cycle();

        for i in 0..len {
            let freq: f32 = *it.next().expect("Expected more data");
            let duty_cycle: f32 = *dit.next().expect("Expected more data");
            let fm: f32 = *fmit.next().expect("Expected more data");
            let primary: f32 = *pit.next().expect("Expected more data");
            ret.push(self.oscillate(primary, freq, fm, duty_cycle));
        }
        ret
    }
}


