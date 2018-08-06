
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
        let buffer_env_attack = module::ConnectorInfo::new("buffer", "envelope_attack");
        let buffer_env_decay = module::ConnectorInfo::new("buffer", "envelope_decay");
        let buffer_env_sustain = module::ConnectorInfo::new("buffer", "envelope_sustain");
        let buffer_env_release = module::ConnectorInfo::new("buffer", "envelope_release");
        let buffer_filter_freq = module::ConnectorInfo::new("buffer", "filter_frequency");
        let buffer_waveform_type = module::ConnectorInfo::new("buffer", "waveform_type");
        let buffer_sec_waveform_type = module::ConnectorInfo::new("buffer", "sec_waveform_type");
        let buffer_sec_waveform_depth = module::ConnectorInfo::new("buffer", "sec_waveform_depth");
        let buffer_sec_waveform_freq = module::ConnectorInfo::new("buffer", "sec_waveform_freq");
        let buffer_note_freq = module::ConnectorInfo::new("buffer", "note_freq");
        let buffer_note_velocity = module::ConnectorInfo::new("buffer", "note_velocity");
        let buffer_note_trigger = module::ConnectorInfo::new("buffer", "note_trigger");
        let buffer_output = module::ConnectorInfo::new("buffer", "output");

        let depth_attenuverter_attenuation = module::ConnectorInfo::new("depth_attenuverter", "attenuation");
        let depth_attenuverter_signal = module::ConnectorInfo::new("depth_attenuverter", "signal");
        let depth_attenuverter_out = module::ConnectorInfo::new("depth_attenuverter", "output");

        let velocity_attenuverter_attenuation = module::ConnectorInfo::new("velocity_attenuverter", "attenuation");
        let velocity_attenuverter_signal = module::ConnectorInfo::new("velocity_attenuverter", "signal");
        let velocity_attenuverter_out = module::ConnectorInfo::new("velocity_attenuverter", "output");

        let osc_freq_in = module::ConnectorInfo::new("primary_osc", "freq_in");
        let osc_duty_cycle_in = module::ConnectorInfo::new("primary_osc", "duty_cycle_in");
        let osc_fm_in = module::ConnectorInfo::new("primary_osc", "fm_in");
        let osc_primary = module::ConnectorInfo::new("primary_osc", "primary");
        let osc_out = module::ConnectorInfo::new("primary_osc", "output");

        let fm_osc_freq_in = module::ConnectorInfo::new("secondary_osc", "freq_in");
        let fm_osc_duty_cycle_in = module::ConnectorInfo::new("secondary_osc", "duty_cycle_in");
        let fm_osc_fm_in = module::ConnectorInfo::new("secondary_osc", "fm_in");
        let fm_osc_primary = module::ConnectorInfo::new("secondary_osc", "primary");
        let fm_osc_out = module::ConnectorInfo::new("secondary_osc", "output");

        let envelope_attack = module::ConnectorInfo::new("envelope", "attack");
        let envelope_decay = module::ConnectorInfo::new("envelope", "decay");
        let envelope_sustain = module::ConnectorInfo::new("envelope", "sustain");
        let envelope_release = module::ConnectorInfo::new("envelope", "release");
        let envelope_signal = module::ConnectorInfo::new("envelope", "signal");
        let envelope_trigger = module::ConnectorInfo::new("envelope", "trigger");
        let envelope_out = module::ConnectorInfo::new("envelope", "output");

        let mut connections = vec![
            module::ConnectionInfo::new(buffer_env_attack, envelope_attack),
            module::ConnectionInfo::new(buffer_env_decay, envelope_decay),
            module::ConnectionInfo::new(buffer_env_sustain, envelope_sustain),
            module::ConnectionInfo::new(buffer_env_release, envelope_release),

            module::ConnectionInfo::new(buffer_note_trigger, envelope_trigger),
            // TODO Attach the filter
            module::ConnectionInfo::new(buffer_waveform_type, osc_primary),
            module::ConnectionInfo::new(buffer_note_freq, osc_freq_in),

            module::ConnectionInfo::new(buffer_sec_waveform_depth, depth_attenuverter_attenuation),
            module::ConnectionInfo::new(buffer_sec_waveform_freq, fm_osc_freq_in),
            module::ConnectionInfo::new(buffer_sec_waveform_type, fm_osc_primary),
            module::ConnectionInfo::new(fm_osc_out, depth_attenuverter_signal),

            module::ConnectionInfo::new(depth_attenuverter_out, osc_fm_in),

            module::ConnectionInfo::new(buffer_note_velocity, velocity_attenuverter_attenuation),
            module::ConnectionInfo::new(osc_out, velocity_attenuverter_signal),

            module::ConnectionInfo::new(velocity_attenuverter_out, envelope_signal),

            module::ConnectionInfo::new(envelope_out, buffer_output)
        ];
        let mut ti = ToneIterator {
            rate,
            rack: module::Rack::new(modules, connections)
        };

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
                        // TODO Create vectors with offset of old values, then the new values until
                        // the next note.
                        let offset = data.time_frames;
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

