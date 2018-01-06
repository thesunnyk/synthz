
use std::f32;
use std::rc::Rc;
use synth::module::Module;

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
    Envelope(f32, f32, f32, f32),
    FilterFreq(f32),
    FilterOn(bool)
}

pub struct ToneIterator {
    t: i64,
    rate: f32,
    filter_freq: f32,
    filter_on: bool,
    waveform: f32,
    buffer: Rc<module::BufferModule>,
    rack: module::Rack,
}

impl ToneIterator {
    pub fn new(rate: f32) -> ToneIterator {
        let buffer = Rc::new(
            module::BufferModule::new(vec![
                                      module::DataIn::new(0.0), // Envelope Attack
                                      module::DataIn::new(0.0), // Envelope Decay
                                      module::DataIn::new(0.0), // Envelope Sustain
                                      module::DataIn::new(0.0), // Envelope Release
                                      module::DataIn::new(0.0), // Filter Frequency
                                      module::DataIn::new(0.0), // Waveform type
                                      module::DataIn::new(0.0), // Note
                                      module::DataIn::new(0.0), // Trigger
                                      module::DataIn::new(0.0), // Output
            ]));
        let mut ti = ToneIterator {
            t: 0,
            rate: rate,
            filter_freq: 22050.0,
            filter_on: true,
            waveform: 0.0,
            buffer: buffer.clone(),
            rack: module::Rack::new(vec![
                                    buffer,
                                    Rc::new(oscillator::Oscillator::new(rate)),
                                    Rc::new(envelope::Envelope::new(rate))])
        };

        // Connect ADSR to Envelope
        ti.rack.connect(0,0, 2,0);
        ti.rack.connect(0,1, 2,1);
        ti.rack.connect(0,2, 2,2);
        ti.rack.connect(0,3, 2,3);

        // Connect trigger to envelope
        ti.rack.connect(0,7, 2,5);

        // Connect note to oscillator
        ti.rack.connect(0,6, 1,0);

        // Connect oscillator to envelope
        ti.rack.connect(1,0, 2,4);

        // Connect envelope to output
        ti.rack.connect(2,0, 0,8);


        ti
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
                        &SynthProperty::Envelope(a, d, s, r) => {
                            // TODO Update envelope ADSR
                        }
                    }
                }
            },
            &SynthEventBody::MidiData(ref midi_ev) => {
                let t = self.t;
                match midi_ev {
                    &midi::MidiEvent::NoteOn { note_num, velocity } => {
                        // TODO Wire up envelope, FM and filter
                        let note = note_num as f32 / 127.0;
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

    pub fn feed(&mut self, samples: usize) -> Vec<f32> {
        self.rack.feed_all(samples);

        Rc::get_mut(&mut self.buffer).unwrap().extract(8, samples)
    }

}

