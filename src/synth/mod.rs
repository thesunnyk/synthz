
use std::f32;

use lv2::midi;
use lv2_raw::midi as raw_midi;

mod filter;
mod module;
pub mod oscillator;
pub mod envelope;

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
pub enum SynthProperty {
    Frame(i64),
    Speed(f32),
    Waveform(f32),
    Envelope(envelope::Envelope),
    FilterFreq(f32),
    FilterOn(bool)
}

pub struct ToneIterator {
    t: i64,
    rate: f32,
    filter_freq: f32,
    filter_on: bool,
    osc: oscillator::Oscillator,
    waveform: f32,
    envelope: envelope::Envelope,
}

impl ToneIterator {
    pub fn new(rate: f32) -> ToneIterator {
        ToneIterator {
            t: 0,
            rate: rate,
            filter_freq: 22050.0,
            filter_on: true,
            osc: oscillator::Oscillator::new(rate),
            waveform: 0.0,
            envelope: envelope::Envelope::new(rate),
        }
    }

    pub fn new_env(&self, a: f32, d: f32, s: f32, r: f32) -> envelope::Envelope {
        // TODO Don't use value from input
        envelope::Envelope::new(self.rate)
    }

    pub fn add_data(&mut self, events: Vec<SynthEvent>) {
        for data in events.as_slice() { match &data.body {
            &SynthEventBody::SynthProperties(ref p) => {
                for prop in p {
                    match prop {
                        &SynthProperty::Frame(f) => {}
                        &SynthProperty::Speed(spd) => {}
                        &SynthProperty::Waveform(wave) => { self.waveform = wave }
                        &SynthProperty::FilterFreq(freq) => { self.filter_freq = freq }
                        &SynthProperty::FilterOn(ison) => { self.filter_on = ison }
                        &SynthProperty::Envelope(ref env) => { self.envelope = env.clone() }
                    }
                }
            },
            &SynthEventBody::MidiData(ref midi_ev) => {
                let t = self.t;
                match midi_ev {
                    &midi::MidiEvent::NoteOn { note_num, velocity } => {
                        let waveform = self.waveform.clone();
                        let envelope = self.envelope.clone();
                        let filter_freq = self.filter_freq;
                        let filter_on = self.filter_on;
                        // TODO Wire up envelope, FM and filter
                        let pitch = (note_num as i32 - 69) as f32 / 12.0;
                        // TODO Velocity as log
                    },
                    &midi::MidiEvent::NoteOff { note_num, velocity } => {
                        // TODO End Note
                    },
                    _ => {
                        println!("MIDI {:?} @{}", midi_ev, data.time_frames);
                    }
                }
            },
        } }

    }

}

