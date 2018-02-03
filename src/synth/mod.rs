
use std::f32;
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
    Secondary(f32, f32, f32),
    Envelope(f32, f32, f32, f32),
    FilterFreq(f32),
    FilterOn(bool)
}

pub struct ToneIterator {
    rate: f32,
    rack: module::Rack,
}

enum Modules {
    Buffer,
    Attenuverter,
    Oscillator,
    FmOscillator,
    Envelope,
}

enum DataItems {
    EnvelopeAttack = 0,
    EnvelopeDecay = 1,
    EnvelopeSustain = 2,
    EnvelopeRelease = 3,
    FilterFrequency = 4,
    WaveformType = 5,
    SecWaveformType = 6,
    SecWaveformDepth = 7,
    SecWaveformFreq = 8,
    NoteFreq = 9,
    NoteTrigger = 10,
    Output = 11,
    Len = 12
}

impl ToneIterator {
    pub fn new(rate: f32) -> ToneIterator {
        let mut buffer_items = Vec::with_capacity(DataItems::Len as usize);
        for i in 0..(DataItems::Len as usize) {
            buffer_items.push(module::DataIn::new(0.0))
        }
        let mut ti = ToneIterator {
            rate: rate,
            rack: module::Rack::new(vec![
                                    Box::new(module::BufferModule::new(buffer_items)),
                                    Box::new(module::Attenuverter::new()),
                                    Box::new(oscillator::Oscillator::new(rate)),
                                    Box::new(oscillator::Oscillator::new(rate)),
                                    Box::new(envelope::Envelope::new(rate))])
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

        // TODO Attach primary and secondary waveforms
        ti.rack.connect(Modules::Buffer as usize, DataItems::NoteFreq as usize,
                        Modules::Oscillator as usize,0);

        ti.rack.connect(Modules::Buffer as usize, DataItems::SecWaveformDepth as usize,
                        Modules::Attenuverter as usize,0);
        ti.rack.connect(Modules::Buffer as usize, DataItems::SecWaveformFreq as usize,
                        Modules::FmOscillator as usize,0);
        ti.rack.connect(Modules::FmOscillator as usize,0,
                        Modules::Attenuverter as usize,1);

        ti.rack.connect(Modules::Attenuverter as usize, 0,
                        Modules::Oscillator as usize, 2);

        ti.rack.connect(Modules::Oscillator as usize,0,
                        Modules::Envelope as usize,4);

        ti.rack.connect(Modules::Envelope as usize,0,
                        Modules::Buffer as usize, DataItems::Output as usize);

        ti
    }

    fn get_buffer<'a>(&'a mut self) -> &'a mut module::Module {
        self.rack.get(0)
    }

    pub fn add_data(&mut self, events: Vec<SynthEvent>) {
        let buffer = self.get_buffer();
        for data in events.as_slice() { match &data.body {
            &SynthEventBody::SynthProperties(ref p) => {
                for prop in p {
                    match prop {
                        &SynthProperty::Frame(f) => {}
                        &SynthProperty::Speed(spd) => {}
                        &SynthProperty::Waveform(wave) => {}
                        &SynthProperty::FilterFreq(freq) => {}
                        &SynthProperty::FilterOn(ison) => {}
                        &SynthProperty::Secondary(wave, depth, multiplier) => {
                            buffer.feed(DataItems::SecWaveformType as usize, vec![wave]);
                            buffer.feed(DataItems::SecWaveformDepth as usize, vec![depth]);
                            buffer.feed(DataItems::SecWaveformFreq as usize, vec![multiplier]);
                        }
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

        let buffer = self.get_buffer();

        buffer.extract(DataItems::Output as usize, samples)
    }

}

