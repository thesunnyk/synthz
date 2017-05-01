
extern crate rand;

use std::collections::VecDeque;
use std::f32;
use self::rand::random;

use lv2::midi;
use lv2_raw::midi as raw_midi;

pub struct SynthEvent {
    time_frames: i64,
    body: SynthEventBody,
}

impl SynthEvent {
    pub fn new(time_frames: i64, body: SynthEventBody) -> SynthEvent {
        SynthEvent {
            time_frames: time_frames,
            body: body,
        }
    }
}

pub enum SynthEventBody {
    MidiData(midi::MidiEvent),
    SynthProperties(Vec<SynthProperty>),
}

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
pub enum SynthProperty {
    Frame(i64),
    Speed(f32),
    Waveform(Waveform),
    SecWave(WaveType),
    Envelope(Envelope)
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Envelope {
    a: f32,
    d: f32,
    s: f32,
    r: f32,
}

impl Envelope {
    fn new(a: f32, d: f32, s: f32, r: f32, rate: f32) -> Envelope {
        Envelope {
            a: rate * a,
            d: rate * d,
            s: s,
            r: rate * r,
        }
    }

    fn envelope(&self, e_t: i64, r_t: i64) -> f32 {
        let rt = r_t as f32;
        let et = e_t as f32;

        let ad = self.a + self.d;
        let er = et + self.r;

        if rt < self.a {
            rt / self.a
        } else if rt < ad {
            (1.0 - self.s) * ((ad - rt) / self.d) + self.s
        } else if rt < et {
            self.s
        } else if rt < er {
            self.s * (er - rt) / self.r
        } else {
            0.0
        }
    }

}

#[derive(Debug)]
#[derive(Clone)]
pub enum WaveType {
    Sine(f32, f32),
    Square(f32, f32, f32),
    Sawtooth(f32, f32),
    Triangle(f32, f32),
    Noise(f32),
}

impl WaveType {

    pub fn from_waveform(form: Waveform, freq: f32, velocity: f32) -> WaveType {
        match form {
            Waveform::Triangle => {
                WaveType::Triangle(freq, velocity)
            },
            Waveform::Square => {
                WaveType::Square(freq, velocity, 0.5)
            },
            Waveform::Sine => {
                WaveType::Sine(freq, velocity)
            },
            Waveform::Sawtooth => {
                WaveType::Sawtooth(freq, velocity)
            },
            Waveform::Noise => {
                WaveType::Noise(velocity)
            },
        }
    }
    fn oscillate(&self, t: f32, sec: f32) -> f32 {
        match self {
            &WaveType::Sine(f, v) => {
                let omega = 2.0 * f32::consts::PI;
                v * f32::sin(f * t * omega + sec)
            },
            &WaveType::Square(f, v, d) => {
                if (f * t + sec) % 1.0 < d {
                    v / 2.0
                } else {
                    -v / 2.0
                }
            },
            &WaveType::Sawtooth(f, v) => {
                v * (2.0 * ((f * t + sec) % 1.0) - 1.0)
            },
            &WaveType::Triangle(f, v) => {
                let out = f * t + sec;
                let saw = 2.0 * ((2.0 * out) % 1.0);
                if out % 1.0 < 0.5 {
                    v * (saw - 1.0)
                } else {
                    v * (1.0 - saw)
                }
            },
            &WaveType::Noise(v) => {
                v * (random::<f32>() * 2.0 - 1.0)
            }
        }
    }

    fn secondary(&self, freq: f32) -> WaveType {
        match self {
            &WaveType::Triangle(mul, depth) => {
                WaveType::Triangle(mul * freq, depth)
            },
            &WaveType::Square(mul, depth, delta) => {
                WaveType::Square(mul * freq, depth, delta)
            },
            &WaveType::Sine(mul, depth) => {
                WaveType::Sine(mul * freq, depth)
            },
            &WaveType::Sawtooth(mul, depth) => {
                WaveType::Sawtooth(mul * freq, depth)
            },
            &WaveType::Noise(depth) => {
                WaveType::Noise(depth)
            },
        }
    }

}

struct Oscillator {
    note: i32,
    rate: f32,
    start_t: i64,
    end_t: i64,
    primary: WaveType,
    secondary: Option<WaveType>,
    envelope: Envelope,
    filter: Filter
}

impl Oscillator {
    fn get_freq(note: i32, rate: f32) -> f32 {
        let pitch: f32 = (note as i32 - 69) as f32;
        let freq_hz = (2.0 as f32).powf(pitch/12.0) * 440.0;
        freq_hz / rate
    }

    fn free_for(&self, t: i64, note: i32) -> bool {
        if self.note == note {
            true
        } else { // TODO Think about release phase
            t > self.end_t
        }
    }

    fn new(rate: f32) -> Oscillator {
        Oscillator {
            note: 0,
            rate: rate,
            start_t: i64::max_value(),
            end_t: 0,
            envelope: Envelope::new(0.01, 0.013, 0.6, 0.1, rate),
            filter: Filter::fromCfg(lpf(450.0, rate)),
            primary: WaveType::Sine(0.0, 0.0),
            secondary: None
        }
    }

    fn config(&mut self, form: Waveform, note: i32, velocity: f32,
              secondary: &Option<WaveType>, env: Envelope, start_t: i64) {
        if start_t > self.end_t {
            self.note = note;
            self.start_t = start_t;
            self.end_t = i64::max_value();
            self.envelope = env.clone();

            let freq = Oscillator::get_freq(note, self.rate);
            let velocity = velocity;
            self.primary = WaveType::from_waveform(form, freq, velocity);
            self.secondary = secondary.as_ref().map(|x| x.secondary(freq));
        }
    }

    fn is_note(&self, note: i32) -> bool {
        note == self.note
    }

    fn end_note(&mut self, end_t: i64) {
        if end_t < self.end_t {
            self.end_t = end_t;
        }
    }

    fn oscillate(&mut self, t: i64) -> f32 {
        if self.start_t < t {
            let env = self.envelope.envelope(self.end_t - self.start_t, t - self.start_t);
            let secwave = self.secondary.as_ref().map_or(0.0, |s| s.oscillate(t as f32, 0.0));

            self.filter.filter(env * self.primary.oscillate(t as f32, secwave))
        } else {
            0.0
        }
    }
}

pub struct ToneIterator {
    t: i64,
    old_t: i64,
    rate: f32,
    osc: Vec<Oscillator>,
    waveform: Waveform,
    envelope: Envelope,
    secondary: Option<WaveType>
}

pub struct FilterConfig {
    x_coeff: Vec<f32>,
    y_coeff: Vec<f32>
}

fn lpf(freq: f32, rate: f32) -> FilterConfig {
    FilterConfig {
        x_coeff: vec![1.0],
        y_coeff: vec![f32::exp(-(freq * 2.0 * f32::consts::PI)/rate)]
    }
}

pub struct Filter {
    config: FilterConfig,
    x_val: VecDeque<f32>,
    y_val: VecDeque<f32>
}

impl Filter {
    pub fn fromCfg(f: FilterConfig) -> Filter {
        let mut xv = VecDeque::with_capacity(f.x_coeff.len());
        for i in 0..f.x_coeff.len() - 1 {
            xv.push_back(0.0);
        }
        let mut yv = VecDeque::with_capacity(f.y_coeff.len());
        for i in 0..f.y_coeff.len() {
            yv.push_back(0.0);
        }
        Filter {
            config: f,
            x_val: xv,
            y_val: yv
        }
    }

    pub fn filter(&mut self, x: f32) -> f32 {
        let mut y = 0;
        self.x_val.push_back(x);
        let y = self.x_val.iter().zip(&self.config.x_coeff).map(|(xi, xc)| xi * xc).fold(0.0, |a, b| a + b)
            + self.y_val.iter().zip(&self.config.y_coeff).map(|(xi, xc)| xi * xc).fold(0.0, |a, b| a + b);

        self.x_val.pop_front();
        self.y_val.push_back(y);
        self.y_val.pop_front();
        y
    }
}

// TODO Add Filter and Filter ADSR
// start at cutoff frequency fC,
// A => go to fC + Filter Depth dp
// D => go to fC + dp*Sustain
// S => Stay
// R => back to fC

// TODO Also make the filter do Formants

// TODO Also add reverb
impl ToneIterator {
    pub fn new(rate: f32) -> ToneIterator {
        let mut vec = Vec::new();
        for i in 0..4 {
            vec.push(Oscillator::new(rate));
        }

        ToneIterator {
            t: 0,
            old_t: 0,
            rate: rate,
            osc: vec,
            waveform: Waveform::Sine,
            envelope: Envelope::new(0.01, 0.013, 0.6, 0.1, rate),
            secondary: Some(WaveType::Sine(2.0, 0.6)),
        }
    }

    pub fn new_env(&self, a: f32, d: f32, s: f32, r: f32) -> Envelope {
        Envelope::new(a, d, s, r, self.rate)
    }

    pub fn add_data(&mut self, events: Vec<SynthEvent>) {
        for data in events.as_slice() { match &data.body {
            &SynthEventBody::SynthProperties(ref p) => {
                for prop in p {
                    match prop {
                        &SynthProperty::Frame(f) => {}
                        &SynthProperty::Speed(spd) => {}
                        &SynthProperty::Waveform(ref wave) => { self.waveform = wave.clone() }
                        &SynthProperty::SecWave(ref wave) => { self.secondary = Some(wave.clone()) }
                        &SynthProperty::Envelope(ref env) => { self.envelope = env.clone() }
                    }
                }
            },
            &SynthEventBody::MidiData(ref midi_ev) => {
                let t = self.t;
                match midi_ev {
                    &midi::MidiEvent::NoteOn { note_num, velocity } => {
                        let secondary = &self.secondary;
                        let waveform = self.waveform.clone();
                        let envelope = self.envelope.clone();
                        // TODO Velocity as log
                        self.osc.iter_mut().find(|x| x.free_for(t, note_num as i32))
                            .map(|mut x| x.config(waveform,
                                                  note_num as i32,
                                                  velocity as f32 / 127.0,
                                                  secondary,
                                                  envelope,
                                                  t + data.time_frames));
                    },
                    &midi::MidiEvent::NoteOff { note_num, velocity } => {
                        self.osc.iter_mut().find(|x| x.is_note(note_num as i32))
                            .map(|mut x| x.end_note(t + data.time_frames));
                    },
                    _ => {
                        println!("MIDI {:?} @{}", midi_ev, data.time_frames);
                    }
                }
            },
        } }

    }

}


impl Iterator for ToneIterator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        self.t = self.t + 1;

        let mut result = 0.0;
        for osc in self.osc.iter_mut() {
            result = result + osc.oscillate(self.t);
        }

        Some(result)
    }
}

