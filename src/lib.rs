
use std::ptr;
use std::ffi as ffi;
use std::os::raw as raw;

const AmpGain: u32 = 1;
const AmpInput: u32 = 2;
const AmpOutput: u32 = 3;

#[repr(C)]
struct Amp {
    gain: *const f32,
    input: *const f32,
    output: *mut f32,
}

type LV2_Handle = *mut raw::c_void;

#[repr(C)]
struct LV2_Descriptor {
    URI: *const raw::c_char,
    instantiate: extern fn (*const LV2_Descriptor, f64, *const raw::c_char, *const *const LV2_Feature) -> LV2_Handle,
    connect_port: extern fn (LV2_Handle, u32, *mut raw::c_void),
    activate: extern fn (LV2_Handle),
    run:extern fn (LV2_Handle, u32),
    deactivate:extern fn (LV2_Handle),
    cleanup:extern fn (LV2_Handle),
    extension_data:extern fn (*const raw::c_char) -> *mut raw::c_void,
}

#[repr(C)]
struct LV2_Feature {
    URI: *const raw::c_char,
    data: *mut raw::c_void
}

const AMP_URI: *const u8 = b"http://quaddmg.com/plugins/eg-amp\0" as *const u8;

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
    println!("OMG OMG ");
    let mut amp = Box::new(Amp {
        gain: std::ptr::null_mut(),
        input: std::ptr::null_mut(),
        output: std::ptr::null_mut(),
    });
    Box::into_raw(amp) as LV2_Handle
}

extern fn connect_port(instance: LV2_Handle, port: u32, data: *mut raw::c_void) {
    let mut amp = instance as *mut Amp;

    unsafe {
        match port {
            AmpGain => (*amp).gain = data as *const f32,
            AmpInput => (*amp).input = data as *const f32,
            AmpOutput => (*amp).output = data as *mut f32,
            _ => {}
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
        let gain: f32 = *(*amp).gain;
        let input: &[f32] = std::slice::from_raw_parts((*amp).input, n_samples as usize);
        let output: &mut [f32] = std::slice::from_raw_parts_mut((*amp).output, n_samples as usize);

        do_gain(gain, input, output);
    }
}

fn do_gain(gain: f32, input: &[f32], output: &mut [f32]) {
    let coef = if gain > (-90.0) { (10.0 as f32).powf(gain * 0.05) } else { 0.0 };

    for pos in 0..input.len() {
        output[pos as usize] = input[pos as usize] * coef;
    }
}

extern fn cleanup(instance: LV2_Handle) {
    unsafe {
        let mut amp: Box<Amp> = Box::from_raw(instance as *mut Amp);
    }
}

extern fn extension_data(uri: *const raw::c_char) -> *mut raw::c_void {
    return std::ptr::null_mut();
}

#[no_mangle]
extern fn lv2_descriptor(index: u32) -> *const LV2_Descriptor {
    match index {
        0 => return &Lv2Descriptor,
        _ => return std::ptr::null_mut()
    }
}

