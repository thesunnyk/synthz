
use std::f32;
use std::rc::Rc;
use synth::module::Module;

use lv2::midi;
use lv2_raw::midi as raw_midi;

mod filter;
mod module;
mod oscillator;
mod envelope;

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
    buffer: Rc<module::BufferModule>,
    rack: module::Rack,
}

enum Modules {
    Buffer,
    Oscillator,
    Envelope,
}

enum DataItems {
    EnvelopeAttack = 0,
    EnvelopeDecay = 1,
    EnvelopeSustain = 2,
    EnvelopeRelease = 3,
    FilterFrequency = 4,
    WaveformType = 5,
    NoteFreq = 6,
    NoteTrigger = 7,
    Output = 8,
    Len = 9
}

impl ToneIterator {
    pub fn new(rate: f32) -> ToneIterator {
        let mut buffer_items = Vec::with_capacity(DataItems::Len as usize);
        for i in 0..(DataItems::Len as usize) {
            buffer_items.push(module::DataIn::new(0.0))
        }
        let buffer = Rc::new(
            module::BufferModule::new(buffer_items));
        let mut ti = ToneIterator {
            t: 0,
            rate: rate,
            buffer: buffer.clone(),
            rack: module::Rack::new(vec![
                                    buffer,
                                    Rc::new(oscillator::Oscillator::new(rate)),
                                    Rc::new(envelope::Envelope::new(rate))])
        };

        ti.rack.connect(Modules::Buffer as usize, DataItems::EnvelopeAttack as usize,
                        Modules::Envelope as usize, 0);
        ti.rack.connect(Modules::Buffer as usize, DataItems::EnvelopeDecay as usize,
                        Modules::Envelope as usize,1);
        ti.rack.connect(Modules::Buffer as usize, DataItems::EnvelopeSustain as usize,
                        Modules::Envelope as usize,2);
        ti.rack.connect(Modules::Buffer as usize, DataItems::EnvelopeRelease as usize,
                        Modules::Envelope as usize,3);

        ti.rack.connect(Modules::Buffer as usize, DataItems::NoteTrigger as usize,
                        Modules::Envelope as usize,5);

        ti.rack.connect(Modules::Buffer as usize, DataItems::NoteFreq as usize,
                        Modules::Oscillator as usize,0);

        ti.rack.connect(Modules::Oscillator as usize,0,
                        Modules::Envelope as usize,4);

        ti.rack.connect(Modules::Envelope as usize,0,
                        Modules::Buffer as usize, DataItems::Output as usize);

        ti
    }

    pub fn add_data(&mut self, events: Vec<SynthEvent>) {
        let buffer = Rc::get_mut(&mut self.buffer).expect("should be available");
        for data in events.as_slice() { match &data.body {
            &SynthEventBody::SynthProperties(ref p) => {
                for prop in p {
                    match prop {
                        &SynthProperty::Frame(f) => {}
                        &SynthProperty::Speed(spd) => {}
                        &SynthProperty::Waveform(wave) => {}
                        &SynthProperty::FilterFreq(freq) => {}
                        &SynthProperty::FilterOn(ison) => {}
                        &SynthProperty::Envelope(a, d, s, r) => {
                            buffer.feed(DataItems::EnvelopeAttack as usize, vec![a]);
                            buffer.feed(DataItems::EnvelopeDecay as usize, vec![d]);
                            buffer.feed(DataItems::EnvelopeSustain as usize, vec![s]);
                            buffer.feed(DataItems::EnvelopeRelease as usize, vec![r]);
                        }
                    }
                }
            },
            &SynthEventBody::MidiData(ref midi_ev) => {
                let t = self.t;
                match midi_ev {
                    &midi::MidiEvent::NoteOn { note_num, velocity } => {
                        let note = note_num as f32 / 127.0;
                        buffer.feed(DataItems::NoteFreq as usize, vec![note]);
                        buffer.feed(DataItems::NoteTrigger as usize, vec![1.0]);
                        // TODO Velocity as log
                    },
                    &midi::MidiEvent::NoteOff { note_num, velocity } => {
                        buffer.feed(DataItems::NoteTrigger as usize, vec![0.0]);
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

        Rc::get_mut(&mut self.buffer).expect("Should exist").extract(DataItems::Output as usize, samples)
    }

}

