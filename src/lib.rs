
mod lv2_raw;
mod lv2;
mod synth;

use std::ptr;
use std::mem;
use std::f32;
use std::ffi;
use std::os::raw;
use std::collections::BTreeMap;

use lv2_raw::core::*;
use lv2_raw::urid::*;
use lv2_raw::atom::*;
use lv2_raw::midi::*;
use lv2::atom::*;
use lv2::urid::*;
use lv2::core::*;
use lv2::midi::*;

const ControlInput: u32 = 0;
const SynthOutput: u32 = 1;
const Waveform: u32 = 2;
const Attack: u32 = 3;
const Decay: u32 = 4;
const Sustain: u32 = 5;
const Release: u32 = 6;
const SecWaveform: u32 = 7;
const SecFreqMul: u32 = 8;
const SecDepth: u32 = 9;
const FilterFreq: u32 = 10;
const FilterOn: u32 = 11;

#[derive(Debug)]
pub struct SamplerUris {
    pub atom_Blank: LV2_URID,
    pub atom_Int: LV2_URID,
    pub atom_Long: LV2_URID,
    pub atom_Float: LV2_URID,
    pub atom_Object: LV2_URID,
    pub atom_Path: LV2_URID,
    pub atom_Property: LV2_URID,
    pub atom_Resource: LV2_URID,
    pub atom_Sequence: LV2_URID,
    pub atom_URID: LV2_URID,
    pub atom_eventTransfer: LV2_URID,
    pub midi_Event: LV2_URID,
    pub patch_Set: LV2_URID,
    pub patch_property: LV2_URID,
    pub patch_value: LV2_URID,
    pub time_frame: LV2_URID,
    pub time_framesPerSecond: LV2_URID,
    pub time_speed: LV2_URID,
    pub time_Position: LV2_URID,
}

pub fn map_sampler_uris(map: *const LV2_URID_Map) -> SamplerUris {
    SamplerUris {
        atom_Blank: urid_for_const(map, LV2_ATOM_Blank),
        atom_Int: urid_for_const(map, LV2_ATOM_Int),
        atom_Long: urid_for_const(map, LV2_ATOM_Long),
        atom_Float: urid_for_const(map, LV2_ATOM_Float),
        atom_Object: urid_for_const(map, LV2_ATOM_Object),
        atom_Path: urid_for_const(map, LV2_ATOM_Path),
        atom_Property: urid_for_const(map, LV2_ATOM_Property),
        atom_Resource: urid_for_const(map, LV2_ATOM_Resource),
        atom_Sequence: urid_for_const(map, LV2_ATOM_Sequence),
        atom_URID: urid_for_const(map, LV2_ATOM_URID),
        atom_eventTransfer: urid_for_const(map, LV2_ATOM_eventTransfer),

        midi_Event: urid_for_const(map, LV2_MIDI_MidiEvent),

        patch_Set: urid_for_const(map, LV2_PATCH_Set),
        patch_property: urid_for_const(map, LV2_PATCH_property),
        patch_value: urid_for_const(map, LV2_PATCH_value),

        time_frame: urid_for_const(map, LV2_TIME_frame),
        time_framesPerSecond: urid_for_const(map, LV2_TIME_framesPerSecond),
        time_speed: urid_for_const(map, LV2_TIME_speed),
        time_Position: urid_for_const(map, LV2_TIME_Position),
    }
}

#[repr(C)]
struct Amp {
    input: *const LV2_Atom,
    output: *mut f32,
    waveform: *mut f32,
    attack: *mut f32,
    decay: *mut f32,
    sustain: *mut f32,
    release: *mut f32,
    sec_waveform: *mut f32,
    sec_freq_mul: *mut f32,
    sec_depth: *mut f32,
    filter_freq: *mut f32,
    filter_on: *mut f32,
    synth: synth::ToneIterator,
    samplerUris: SamplerUris,
}

const AMP_URI: *const u8 = b"http://quaddmg.com/plugins/synthz\0" as *const u8;

const Lv2Descriptor: LV2_Descriptor = LV2_Descriptor {
    URI: AMP_URI as *const raw::c_char,
    instantiate: instantiate,
    connect_port: connect_port,
    activate: activate,
    run: run,
    deactivate: deactivate,
    cleanup: cleanup,
    extension_data: extension_data
};

struct URID_Extractor<'a> {
    urid_uri: &'a ffi::CStr,
    urid_map: Option<*const LV2_URID_Map>
}

impl <'a> URID_Extractor<'a> {
    fn new() -> URID_Extractor<'a> {
        unsafe {
            URID_Extractor {
                urid_uri: ffi::CStr::from_ptr(LV2_URID_map as *const raw::c_char),
                urid_map: None
            }
        }
    }
}

impl <'a> FeatureExtractor for URID_Extractor<'a> {
    fn matches(&self, item: &ffi::CStr) -> bool {
        *item == *self.urid_uri
    }

    fn store(&mut self, data: *const raw::c_void) {
        unsafe {
            self.urid_map = Some(data as *const LV2_URID_Map);
        }
    }
}

extern fn instantiate(descriptor: *const LV2_Descriptor,
                      rate: f64,
                      path: *const raw::c_char,
                      features: *const *const LV2_Feature) -> LV2_Handle {
    println!("SynthZ instantiate");

    let mut urid_extractor = URID_Extractor::new();
    extract_features(features, vec!(&mut urid_extractor));

    let mut urid_map = urid_extractor.urid_map.unwrap();

    let mut amp = Box::new(Amp {
        input: std::ptr::null_mut(),
        output: std::ptr::null_mut(),
        waveform: std::ptr::null_mut(),
        sec_waveform: std::ptr::null_mut(),
        sec_freq_mul: std::ptr::null_mut(),
        sec_depth: std::ptr::null_mut(),
        attack: std::ptr::null_mut(),
        decay: std::ptr::null_mut(),
        sustain: std::ptr::null_mut(),
        release: std::ptr::null_mut(),
        filter_freq: std::ptr::null_mut(),
        filter_on: std::ptr::null_mut(),
        synth: synth::ToneIterator::new(rate as f32),
        samplerUris: map_sampler_uris(urid_map),
    });

    println!("{:?}", amp.samplerUris);
    Box::into_raw(amp) as LV2_Handle
}

extern fn connect_port(instance: LV2_Handle, port: u32, data: *mut raw::c_void) {
    let mut pamp = instance as *mut Amp;

    unsafe {
        let amp = &mut *pamp;
        match port {
            ControlInput => {
                amp.input = data as *const LV2_Atom
            },
            SynthOutput => {
                amp.output = data as *mut f32
            },
            Waveform => {
                amp.waveform = data as *mut f32
            },
            Attack => {
                amp.attack = data as *mut f32
            },
            Decay => {
                amp.decay = data as *mut f32
            },
            Sustain => {
                amp.sustain = data as *mut f32
            },
            Release => {
                amp.release = data as *mut f32
            },
            SecWaveform => {
                amp.sec_waveform = data as *mut f32
            },
            SecFreqMul => {
                amp.sec_freq_mul = data as *mut f32
            },
            SecDepth => {
                amp.sec_depth = data as *mut f32
            }
            FilterFreq => {
                amp.filter_freq = data as *mut f32
            }
            FilterOn => {
                amp.filter_on = data as *mut f32
            }
            _ => {println!("SynthZ Connect to unknown port")}
        }
    }
}

extern fn activate(instance: LV2_Handle) {
}

extern fn deactivate(instance: LV2_Handle) {
}

fn extract_sequence(seq: *const LV2_Atom_Sequence, s: &SamplerUris) -> Vec<synth::SynthEvent> {
    let mut ret = Vec::new();

    let iter: AtomSequenceIter = AtomSequenceIter::new(seq);

    for event in iter {
        if event.data_type == s.midi_Event {
            ret.push(synth::SynthEvent::new(event.time_frames,
                                     synth::SynthEventBody::MidiData(MidiEvent::new(event.data, event.size))));
        } else if event.data_type == s.atom_Object || event.data_type == s.atom_Blank {
            let properties = synth::SynthEventBody::SynthProperties(
                extract_object(event.data as *const LV2_Atom_Object_Body, event.size, s));
            ret.push(synth::SynthEvent::new(event.time_frames, properties));
        }
    }

    ret
}

fn extract_object(obj: *const LV2_Atom_Object_Body,
                  size: usize,
                  uris: &SamplerUris) -> Vec<synth::SynthProperty> {
    unsafe {
        let oType = (*obj).otype;
        let mut processed: usize = mem::size_of::<LV2_Atom_Object_Body>();
        let mut items: Vec<synth::SynthProperty> = Vec::new();

        while processed < size {
            let pboffset = (obj as usize).checked_add(processed).unwrap();
            let pbody: *const LV2_Atom_Property_Body = pboffset as *const LV2_Atom_Property_Body;
            let body = &*pbody;

            // TODO Get BPM, Bar (?), and BarBeat (?), BeatsPerBar (?)
            if body.key == uris.time_frame {
                assert_eq!(body.value.size as usize, mem::size_of::<i64>());
                assert_eq!(body.value.atom_type, uris.atom_Long);
                let value = pbody.offset(1) as *const i64;
                items.push(synth::SynthProperty::Frame(*value));
            } else if body.key == uris.time_speed {
                assert_eq!(body.value.size as usize, mem::size_of::<f32>());
                assert_eq!(body.value.atom_type, uris.atom_Float);
                let value = pbody.offset(1) as *const f32;
                items.push(synth::SynthProperty::Speed(*value));
            }
            processed = processed + pad_size(body.value.size) as usize + mem::size_of::<LV2_Atom_Property_Body>();
        }
        items
    }
}

fn get_waveform(wave: f32) -> synth::Waveform {
    let waveint = wave as i32;
    match waveint {
        0 => { synth::Waveform::Sine }
        1 => { synth::Waveform::Square }
        2 => { synth::Waveform::Sawtooth }
        3 => { synth::Waveform::Triangle }
        4 => { synth::Waveform::Noise }
        _ => { panic!("Unexpected waveform"); }
    }
}

extern fn run(instance: LV2_Handle, n_samples: u32) {
    let mut pamp: *mut Amp = instance as *mut Amp;
    unsafe {
        let amp = &mut *pamp;
        let pinput = amp.input;

        let input = &*pinput;

        let uris = &amp.samplerUris;

        let synth = &mut amp.synth;

        let waveform = get_waveform(*amp.waveform);

        let envelope = synth.new_env(*amp.attack, *amp.decay, *amp.sustain, *amp.release);

        let sec_waveform = get_waveform(*amp.sec_waveform);

        let sec = synth::WaveType::from_waveform(sec_waveform, *amp.sec_freq_mul, *amp.sec_depth);

        let filter_freq = *amp.filter_freq;

        let filter_on = *amp.filter_on > 0.5;

        let control = vec!(
                synth::SynthProperty::Waveform(waveform),
                synth::SynthProperty::Envelope(envelope),
                synth::SynthProperty::SecWave(sec),
                synth::SynthProperty::FilterFreq(filter_freq),
                synth::SynthProperty::FilterOn(filter_on)
            );
        let evs = vec!(synth::SynthEvent::new(0, synth::SynthEventBody::SynthProperties(control)));
        synth.add_data(evs);

        if input.atom_type == uris.atom_Sequence {
            let midi_data = extract_sequence(pinput as *const LV2_Atom_Sequence, uris);
            synth.add_data(midi_data);
        }

        let output: &mut [f32] = std::slice::from_raw_parts_mut(amp.output, n_samples as usize);

        for i in 0..output.len() {
            output[i as usize] = synth.next().unwrap();
        }
    }
}

extern fn cleanup(instance: LV2_Handle) {
    println!("SynthZ cleanup");
    unsafe {
        let mut amp: Box<Amp> = Box::from_raw(instance as *mut Amp);
        drop(amp);
    }
}

extern fn extension_data(uri: *const raw::c_char) -> *mut raw::c_void {
    println!("SynthZ extension_data");
    return std::ptr::null_mut();
}

#[no_mangle]
pub extern fn lv2_descriptor(index: u32) -> *const LV2_Descriptor {
    println!("SynthZ lv2_descriptor");
    match index {
        0 => return &Lv2Descriptor,
        _ => return std::ptr::null_mut()
    }
}

