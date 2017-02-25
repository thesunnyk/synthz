
mod lv2_raw;
mod lv2;

use lv2_raw::core::*;
use lv2_raw::urid::*;
use lv2_raw::atom::*;
use lv2::atom::*;
use lv2::urid::*;
use lv2::core::*;
use std::ptr;
use std::mem;
use std::ffi as ffi;
use std::os::raw as raw;

const ControlInput: u32 = 0;
const SynthOutput: u32 = 1;

pub struct SamplerUris {
    pub atom_Blank: LV2_URID,
    pub atom_Path: LV2_URID,
    pub atom_Resource: LV2_URID,
    pub atom_Sequence: LV2_URID,
    pub atom_URID: LV2_URID,
    pub atom_eventTransfer: LV2_URID,
    pub midi_Event: LV2_URID,
    pub patch_Set: LV2_URID,
    pub patch_property: LV2_URID,
    pub patch_value: LV2_URID,
}

pub fn map_sampler_uris(map: *const LV2_URID_Map) -> SamplerUris {
    SamplerUris {
        atom_Blank: urid_for_const(map, LV2_ATOM_Blank),
        atom_Path: urid_for_const(map, LV2_ATOM_Path),
        atom_Resource: urid_for_const(map, LV2_ATOM_Resource),
        atom_Sequence: urid_for_const(map, LV2_ATOM_Sequence),
        atom_URID: urid_for_const(map, LV2_ATOM_URID),
        atom_eventTransfer: urid_for_const(map, LV2_ATOM_eventTransfer),

        midi_Event: urid_for_const(map, LV2_MIDI_MidiEvent),

        patch_Set: urid_for_const(map, LV2_PATCH_Set),
        patch_property: urid_for_const(map, LV2_PATCH_property),
        patch_value: urid_for_const(map, LV2_PATCH_value),
    }
}

fn extract_sequence(seq: *const LV2_Atom_Sequence, urid_map: &SamplerUris) {

    let iter: AtomSequenceIter = AtomSequenceIter::new(seq);

    for data in iter {
        if data.data_type == urid_map.midi_Event {
            // TODO Extract MIDI Data
            println!("{:?} -> {:?} ({:?})", data.data_type, data.data, data.size);
        }
    }
}

#[repr(C)]
struct Amp {
    input: *const LV2_Atom_Sequence,
    output: *mut f32,
    samplerUris: SamplerUris
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

fn extract_features(features: *const *const LV2_Feature) -> Option<*const LV2_URID_Map> {
    let mut features_iter: *const *const LV2_Feature = features;

    let mut urid_map: Option<*const LV2_URID_Map> = None;

    let iter = LV2_Feature_Iter::new(features);

    unsafe {
        let urid_map_uri = ffi::CStr::from_ptr(LV2_URID_map as *const raw::c_char);

        for feature in iter {
            let urid = ffi::CStr::from_ptr((*feature).URI);
            if urid_map_uri == urid {
                urid_map = Some((*feature).data as *const LV2_URID_Map);
            }
        }
    }

    urid_map
}

extern fn instantiate(descriptor: *const LV2_Descriptor,
                      rate: f64,
                      path: *const raw::c_char,
                      features: *const *const LV2_Feature) -> LV2_Handle {
    println!("SynthZ instantiate");

    let mut urid_map = extract_features(features);

    let mut amp = Box::new(Amp {
        input: std::ptr::null_mut(),
        output: std::ptr::null_mut(),
        samplerUris: map_sampler_uris(urid_map.unwrap())
    });
    Box::into_raw(amp) as LV2_Handle
}

extern fn connect_port(instance: LV2_Handle, port: u32, data: *mut raw::c_void) {
    let mut amp = instance as *mut Amp;

    unsafe {
        match port {
            ControlInput => {
                (*amp).input = data as *const LV2_Atom_Sequence
            },
            SynthOutput => {
                (*amp).output = data as *mut f32
            },
            _ => {println!("SynthZ Connect to unknown port")}
        }
    }
}

extern fn activate(instance: LV2_Handle) {
}

extern fn deactivate(instance: LV2_Handle) {
}

extern fn run(instance: LV2_Handle, n_samples: u32) {
    let mut amp: *mut Amp = instance as *mut Amp;
    unsafe {
        let input = (*amp).input;

        extract_sequence(input, &(*amp).samplerUris);

        let output: &mut [f32] = std::slice::from_raw_parts_mut((*amp).output, n_samples as usize);

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

