
use std::f32;

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
    MidiEvent(midi::MidiData),
    SynthProperties(Vec<SynthProperty>),
}

#[derive(Debug)]
pub enum SynthProperty {
    Frame(i64),
    Speed(f32),
}

struct Envelope {
    a: f32,
    d: f32,
    s: f32,
    r: f32,
}

impl Envelope {
    fn new(a: i64, d: i64, s: f32, r: i64) -> Envelope {
        Envelope {
            a: a as f32,
            d: d as f32,
            s: s,
            r: r as f32,
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

struct Oscillator {
    freq: f32,
    velocity: f32,
    rate: f32,
    start_t: i64,
    end_t: i64,
    envelope: Envelope,
}

impl Oscillator {
    fn get_freq(note: i32) -> f32 {
        let pitch: f32 = (note as i32 - 69) as f32;
        (2.0 as f32).powf(pitch/12.0) * 440.0
    }

    fn new(rate: f32) -> Oscillator {
        let a = rate as i64 * 10 / 1000;
        let d = rate as i64 * 13 / 1000;
        let s = 0.8;
        let r = rate as i64 * 100 / 1000;
        Oscillator {
            freq: 0.0,
            rate: rate,
            velocity: 0.0,
            start_t: i64::max_value(),
            end_t: i64::max_value(),
            envelope: Envelope::new(a, d, s, r),
        }
    }

    fn config(&mut self, note: i32, velocity: f32, start_t: i64) {
        self.freq = Oscillator::get_freq(note);
        self.start_t = start_t;
        self.velocity = velocity;
        self.end_t = i64::max_value();
    }

    fn end_time(&mut self, end_t: i64) {
        self.end_t = end_t;
    }

    fn oscillate(&self, t: i64) -> f32 {
        if self.start_t < t {
            let env = self.envelope.envelope(self.end_t - self.start_t, t - self.start_t);
            env * self.velocity * f32::sin(t as f32 * self.freq * 2.0 * f32::consts::PI / self.rate)
        } else {
            0.0
        }
    }
}

pub struct ToneIterator {
    t: i64,
    old_t: i64,
    speed: f32,
    rate: f32,
    cur_note: i32,
    osc: Oscillator,
}

impl ToneIterator {
    pub fn new(rate: f32) -> ToneIterator {
        ToneIterator {
            t: 0,
            old_t: 0,
            speed: 1.0,
            rate: rate,
            cur_note: 0,
            osc: Oscillator::new(rate),
        }
    }

    pub fn add_data(&mut self, events: Vec<SynthEvent>) {
        for data in events.as_slice() { match &data.body {
            &SynthEventBody::SynthProperties(ref p) => {
                let mut speed = self.speed;
                let mut t = self.t;
                for i in p {
                    match i {
                        &SynthProperty::Speed(ref s) => { speed = *s },
                        &SynthProperty::Frame(ref f) => { t = *f }
                    }
                }
                if self.old_t != t {
                    self.old_t = t;
                    self.t = t;
                    self.speed = speed;
                }
            },
            &SynthEventBody::MidiEvent(ref midi_data) => {
                match midi_data.status {
                    raw_midi::LV2_MIDI_MSG_NOTE_ON => {
                        if self.cur_note != midi_data.pitch as i32 {
                            println!("MDO {}, {}, {}, {}", midi_data.pitch, data.time_frames, self.t,
                                     self.t + data.time_frames);
                            self.cur_note = midi_data.pitch as i32;
                            // TODO Velocity as log
                            self.osc.config(midi_data.pitch as i32,
                                            midi_data.velocity as f32 / 127.0,
                                            self.t + data.time_frames);
                        }
                    },
                    raw_midi::LV2_MIDI_MSG_NOTE_OFF => {
                        if self.cur_note == midi_data.pitch as i32 {
                            println!("MDF {}, {}", midi_data.pitch, data.time_frames);
                            self.cur_note = -1;
                            self.osc.end_time(self.t + data.time_frames);
                        }
                    },
                    _ => {
                        println!("MIDI({}), {}, @{}", midi_data.status, midi_data.pitch, data.time_frames);
                    }
                }
            },
        } }

    }

}


impl Iterator for ToneIterator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        if self.speed > 0.0 {
            self.t = self.t + 1;
        }

        let mut result = self.osc.oscillate(self.t);

        Some(result)
    }
}

