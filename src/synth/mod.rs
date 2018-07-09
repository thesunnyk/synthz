
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
        let mut buffer_items = vec![
            module::DataIn::new(String::from("envelope_attack"), 0.0),
            module::DataIn::new(String::from("envelope_decay"), 0.0),
            module::DataIn::new(String::from("envelope_sustain"), 0.0),
            module::DataIn::new(String::from("envelope_release"), 0.0),
            module::DataIn::new(String::from("filter_frequency"), 0.0),
            module::DataIn::new(String::from("waveform_type"), 0.0),
            module::DataIn::new(String::from("sec_waveform_type"), 0.0),
            module::DataIn::new(String::from("sec_waveform_depth"), 0.0),
            module::DataIn::new(String::from("sec_waveform_freq"), 0.0),
            module::DataIn::new(String::from("note_freq"), 0.0),
            module::DataIn::new(String::from("note_velocity"), 0.0),
            module::DataIn::new(String::from("note_trigger"), 0.0),
            module::DataIn::new(String::from("output"), 0.0)
        ];
        let mut modules = vec![
            module::ModuleInfo::new("buffer", Box::new(module::BufferModule::new(buffer_items))),
            module::ModuleInfo::new("depth_attenuverter", Box::new(module::Attenuverter::new())),
            module::ModuleInfo::new("velocity_attenuverter", Box::new(module::Attenuverter::new())),
            module::ModuleInfo::new("primary_osc", Box::new(oscillator::Oscillator::new(rate))),
            module::ModuleInfo::new("secondary_osc", Box::new(oscillator::Oscillator::new(rate))),
            module::ModuleInfo::new("envelope", Box::new(envelope::Envelope::new(rate))),
        ];
        let mut connections = vec![];
        let mut ti = ToneIterator {
            rate,
            rack: module::Rack::new(modules, connections)
        };

//        ti.rack.connect( buffer.connector(DataItems::EnvelopeAttack as usize),
//                        envelope.connect_in(EnvelopeInput::Attack));
//        ti.rack.connect(buffer.connector(DataItems::EnvelopeDecay as usize),
//                        envelope.connect_in(EnvelopeInput::Decay));
//        ti.rack.connect(buffer.connector(DataItems::EnvelopeSustain as usize),
//                        envelope.connect_in(EnvelopeInput::Sustain));
//        ti.rack.connect(buffer.connector(DataItems::EnvelopeRelease as usize),
//                        envelope.connect_in(EnvelopeInput::Release));
//
//        ti.rack.connect(buffer.connector(DataItems::NoteTrigger as usize),
//                        envelope.connect_in(EnvelopeInput::Trigger));
//
//        // TODO Attach the filter
//
//        ti.rack.connect(buffer.connector(DataItems::WaveformType as usize),
//                        oscillator.connector_in(OscillatorInput::Primary));
//        ti.rack.connect(buffer.connector(DataItems::NoteFreq as usize),
//                        oscillator.connector_in(OscillatorInput::FreqIn));
//
//        ti.rack.connect(buffer.connector(DataItems::SecWaveformDepth as usize),
//                        depth_attenuverter.connector_in(AttenuverterInput::ATTENUATION));
//        ti.rack.connect(buffer.connector(DataItems::SecWaveformFreq as usize),
//                        fm_oscillator.connector_in(OscillatorInput::FreqIn));
//        ti.rack.connect(buffer.connector(DataItems::SecWaveformType as usize),
//                        fm_oscillator.connector_in(OscillatorInput::Primary));
//        ti.rack.connect(fm_oscillator.connector_out(),
//                        depth_attenuverter.connector_in(AttenuverterInput::SIGNAL));
//
//        ti.rack.connect(depth_attenuverter.connector_out(),
//                        oscillator.connector_in(OscillatorInput::FmIn));
//
//        ti.rack.connect(buffer.connector(DataItems::NoteVelocity as usize),
//                        velocity_attenuverter.conector_in(AttenuverterInput::ATTENUATION));
//        ti.rack.connect(oscillator.connector_out(),
//                        velocity_attenuverter.connector_in(AttenuverterInput::SIGNAL));
//
//        ti.rack.connect(velocity_attenuverter.connector_out(),
//                        envelope.connect_in(EnvelopeInput::Signal));
//
//        ti.rack.connect(envelope.connector_out(),
//                        buffer.connector(DataItems::Output as usize));

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

