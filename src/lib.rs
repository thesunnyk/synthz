
use std::ptr;
use std::mem;
use std::ffi as ffi;
use std::os::raw as raw;

const ControlInput: u32 = 0;
const SynthOutput: u32 = 1;

type LV2_Handle = *mut raw::c_void;
type LV2_URID_Map_Handle = *mut raw::c_void;

type LV2_URID = u32;

#[repr(C)]
pub struct LV2_Descriptor {
    URI: *const raw::c_char,
    instantiate: extern fn (*const LV2_Descriptor, f64, *const raw::c_char, *const *const LV2_Feature) -> LV2_Handle,
    connect_port: extern fn (LV2_Handle, u32, *mut raw::c_void),
    activate: extern fn (LV2_Handle),
    run:extern fn (LV2_Handle, u32),
    deactivate:extern fn (LV2_Handle),
    cleanup:extern fn (LV2_Handle),
    extension_data:extern fn (*const raw::c_char) -> *mut raw::c_void,
}

const LV2_URID_map: *const u8 = b"http://lv2plug.in/ns/ext/urid#map\0" as *const u8;

const LV2_ATOM_Blank: *const u8 = b"http://lv2plug.in/ns/ext/atom#Blank\0" as *const u8;
const LV2_ATOM_Path: *const u8 = b"http://lv2plug.in/ns/ext/atom#Path\0" as *const u8;
const LV2_ATOM_Resource: *const u8 = b"http://lv2plug.in/ns/ext/atom#Resource\0" as *const u8;
const LV2_ATOM_Sequence: *const u8 = b"http://lv2plug.in/ns/ext/atom#Sequence\0" as *const u8;
const LV2_ATOM_URID: *const u8 = b"http://lv2plug.in/ns/ext/atom#URID\0" as *const u8;
const LV2_ATOM_eventTransfer: *const u8 = b"http://lv2plug.in/ns/ext/atom#eventTransfer\0" as *const u8;

const LV2_MIDI_MidiEvent: *const u8 = b"http://lv2plug.in/ns/ext/midi#MidiEvent\0" as *const u8;

const LV2_PATCH_Set: *const u8 = b"http://lv2plug.in/ns/ext/patch#Set\0" as *const u8;
const LV2_PATCH_property: *const u8 = b"http://lv2plug.in/ns/ext/patch#property\0" as *const u8;
const LV2_PATCH_value: *const u8 = b"http://lv2plug.in/ns/ext/patch#value\0" as *const u8;

#[repr(C)]
pub struct LV2_Atom {
    size: u32,
    atom_type: u32,
}

// #[repr(C)]
// pub union LV2_Atom_Event_Time {
//     frames: i64,
//     beats: f64,
// }

#[repr(C)]
pub struct LV2_Atom_Event {
    time_frames: i64, // LV2_Atom_Event_Time,
    body: LV2_Atom,
}

#[repr(C)]
pub struct LV2_Atom_Sequence_Body {
    unit: u32,
    pad: u32,
}

#[repr(C)]
pub struct LV2_Atom_Sequence {
    atom: LV2_Atom,
    body: LV2_Atom_Sequence_Body,
}

fn pad_size(size: u32) -> usize {
    let seven: usize = 7;

    (size as usize + seven) & !seven
}

fn extract_sequence(seq: *const LV2_Atom_Sequence, urid_map: *const SamplerUris) {
    unsafe {
        let start = seq.offset(1) as *const LV2_Atom_Event;

        let mut next = start;

        let total: usize = (seq as usize)
            .checked_add((*seq).atom.size as usize)
            .unwrap()
            .checked_add(mem::size_of::<LV2_Atom>())
            .unwrap();

        while (next as usize) < total {
            let next_size: usize = mem::size_of::<LV2_Atom_Event>() + pad_size((*next).body.size);

            // TODO Grab events
            if (*next).body.atom_type == (*urid_map).midi_Event {
                let msg = next.offset(1) as *const u8;
                println!("{:?}", *msg);
            }

            next = ((next as usize).checked_add(next_size as usize).unwrap()) as *const LV2_Atom_Event;
        }
    }
}

#[repr(C)]
struct LV2_Feature {
    URI: *const raw::c_char,
    data: *mut raw::c_void
}

struct LV2_URID_Map {
    handle: LV2_URID_Map_Handle,
    map: extern fn(LV2_URID_Map_Handle, *const raw::c_char) -> LV2_URID,
}

struct SamplerUris {
    atom_Blank: LV2_URID,
    atom_Path: LV2_URID,
    atom_Resource: LV2_URID,
    atom_Sequence: LV2_URID,
    atom_URID: LV2_URID,
    atom_eventTransfer: LV2_URID,
    midi_Event: LV2_URID,
    patch_Set: LV2_URID,
    patch_property: LV2_URID,
    patch_value: LV2_URID,
}

fn map_sampler_uris(map: *const LV2_URID_Map) -> SamplerUris {
    unsafe {
        SamplerUris {
            atom_Blank: ((*map).map)((*map).handle, LV2_ATOM_Blank as *const raw::c_char),
            atom_Path: ((*map).map)((*map).handle, LV2_ATOM_Path as *const raw::c_char),
            atom_Resource: ((*map).map)((*map).handle, LV2_ATOM_Resource as *const raw::c_char),
            atom_Sequence: ((*map).map)((*map).handle, LV2_ATOM_Sequence as *const raw::c_char),
            atom_URID: ((*map).map)((*map).handle, LV2_ATOM_URID as *const raw::c_char),
            atom_eventTransfer: ((*map).map)((*map).handle, LV2_ATOM_eventTransfer as *const raw::c_char),

            midi_Event: ((*map).map)((*map).handle, LV2_MIDI_MidiEvent as *const raw::c_char),

            patch_Set: ((*map).map)((*map).handle, LV2_PATCH_Set as *const raw::c_char),
            patch_property: ((*map).map)((*map).handle, LV2_PATCH_property as *const raw::c_char),
            patch_value: ((*map).map)((*map).handle, LV2_PATCH_value as *const raw::c_char),
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

extern fn instantiate(descriptor: *const LV2_Descriptor,
                      rate: f64,
                      path: *const raw::c_char,
                      features: *const *const LV2_Feature) -> LV2_Handle {
    println!("SynthZ instantiate");

    unsafe {
        let mut features_iter: *const *const LV2_Feature = features;

        let mut urid_map: Option<*const LV2_URID_Map> = None;

        let urid_map_uri = ffi::CStr::from_ptr(LV2_URID_map as *const raw::c_char);

        while *features_iter as usize > 0 {
            let mut feature: *const LV2_Feature = *features_iter;

            let urid = ffi::CStr::from_ptr((*feature).URI);
            if urid_map_uri == urid {
                urid_map = Some((*feature).data as *const LV2_URID_Map);
            }

            features_iter = features_iter.offset(1);
        }

        let mut amp = Box::new(Amp {
            input: std::ptr::null_mut(),
            output: std::ptr::null_mut(),
            samplerUris: map_sampler_uris(urid_map.unwrap())
        });
        Box::into_raw(amp) as LV2_Handle
    }
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

