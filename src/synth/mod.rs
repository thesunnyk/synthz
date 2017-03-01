
use std::f32;

use lv2::midi;
use lv2_raw::midi as raw_midi;

pub enum SynthEvent {
    MidiEvent(midi::MidiData)
}

pub struct ToneIterator {
    t: u64,
    rate: f32,
    data: Vec<SynthEvent>
}

impl ToneIterator {
    pub fn new(rate: f32) -> ToneIterator {
        ToneIterator {
            t: 0,
            rate: rate,
            data: Vec::new(),
        }
    }

    pub fn add_data(&mut self, mut events: Vec<SynthEvent>) {
        self.data.append(&mut events);
    }

    pub fn clear_data(&mut self) {
        self.data.clear();
    }
}

impl Iterator for ToneIterator {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let t: f32 = self.t as f32;
        self.t = self.t + 1;

        let mut vol = 0.0;
        let mut freq = 0.0;
        for data in &self.data { match data {
            &SynthEvent::MidiEvent(ref midi_data) => 
                if midi_data.status == raw_midi::LV2_MIDI_MSG_NOTE_ON {
                    vol = 0.6;
                    let pitch: f32 = (midi_data.pitch as i32 - 69) as f32;
                    freq = (2.0 as f32).powf(pitch/12.0) * 440.0;
                }
        } }

        //
        // f(n) = 2^((n-69)/12)*440 // where n = midi note

        Some(vol * f32::sin(t * freq * 2.0 * f32::consts::PI / self.rate))
    }
}

