
use std::f32;
use synth::module::Module;

use lv2::midi;
use lv2_raw::midi as raw_midi;
use synth::module::AttenuverterInput;
use synth::oscillator::OscillatorInput;

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
            time_frames,
            body,
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
    DepthAttenuverter,
    VelocityAttenuverter,
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
    NoteVelocity = 10,
    NoteTrigger = 11,
    Output = 12,
    Len = 13
}

impl ToneIterator {
    pub fn new(rate: f32) -> ToneIterator {
        let mut buffer_items = Vec::with_capacity(DataItems::Len as usize);
        for i in 0..(DataItems::Len as usize) {
            buffer_items.push(module::DataIn::new(0.0))
        }
        let mut ti = ToneIterator {
            rate,
            rack: module::Rack::new(vec![
                                    Box::new(module::BufferModule::new(buffer_items)),
                                    Box::new(module::Attenuverter::new()),
                                    Box::new(module::Attenuverter::new()),
                                    Box::new(oscillator::Oscillator::new(rate)),
                                    Box::new(oscillator::Oscillator::new(rate)),
                                    Box::new(envelope::Envelope::new(rate))])
        };

        let buffer = ti.rack.get(0);
        let depth_attenuverter = ti.rack.get(1);
        let velocity_attenuverter = ti.rack.get(2);
        let oscillator = ti.rack.get(3);
        let fm_oscillator = ti.rack.get(4);
        let envelope = ti.rack.get(5);

        ti.rack.connect( buffer.connector(DataItems::EnvelopeAttack as usize),
                        envelope.connect_in(EnvelopeInput::Attack));
        ti.rack.connect(buffer.connector(DataItems::EnvelopeDecay as usize),
                        envelope.connect_in(EnvelopeInput::Decay));
        ti.rack.connect(buffer.connector(DataItems::EnvelopeSustain as usize),
                        envelope.connect_in(EnvelopeInput::Sustain));
        ti.rack.connect(buffer.connector(DataItems::EnvelopeRelease as usize),
                        envelope.connect_in(EnvelopeInput::Release));

        ti.rack.connect(buffer.connector(DataItems::NoteTrigger as usize),
                        envelope.connect_in(EnvelopeInput::Trigger));

        // TODO Attach the filter

        ti.rack.connect(buffer.connector(DataItems::WaveformType as usize),
                        oscillator.connector_in(OscillatorInput::Primary));
        ti.rack.connect(buffer.connector(DataItems::NoteFreq as usize),
                        oscillator.connector_in(OscillatorInput::FreqIn));

        ti.rack.connect(buffer.connector(DataItems::SecWaveformDepth as usize),
                        depth_attenuverter.connector_in(AttenuverterInput::ATTENUATION));
        ti.rack.connect(buffer.connector(DataItems::SecWaveformFreq as usize),
                        fm_oscillator.connector_in(OscillatorInput::FreqIn));
        ti.rack.connect(buffer.connector(DataItems::SecWaveformType as usize),
                        fm_oscillator.connector_in(OscillatorInput::Primary));
        ti.rack.connect(fm_oscillator.connector_out(),
                        depth_attenuverter.connector_in(AttenuverterInput::SIGNAL));

        ti.rack.connect(depth_attenuverter.connector_out(),
                        oscillator.connector_in(OscillatorInput::FmIn));

        ti.rack.connect(buffer.connector(DataItems::NoteVelocity as usize),
                        velocity_attenuverter.conector_in(AttenuverterInput::ATTENUATION));
        ti.rack.connect(oscillator.connector_out(),
                        velocity_attenuverter.connector_in(AttenuverterInput::SIGNAL));

        ti.rack.connect(velocity_attenuverter.connector_out(),
                        envelope.connect_in(EnvelopeInput::Signal));

        ti.rack.connect(envelope.connector_out(),
                        buffer.connector(DataItems::Output as usize));

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
                        &SynthProperty::Waveform(wave) => {
                            buffer.feed(DataItems::WaveformType as usize, vec![wave]);
                        }
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
                        buffer.feed(DataItems::NoteVelocity as usize, vec![(velocity as f32) / 255.0 + 0.5]);
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

