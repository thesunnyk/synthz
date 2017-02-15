
use std::ptr;
use std::os::raw::c_void;

enum PortIndex {
    AmpGain = 1,
    AmpInput = 2,
    AmpOutput = 3
}

#[repr(C)]
struct Amp {
    gain: *const f32,
    input: *const f32,
    output: *mut f32,
}

type LV2_Handle = raw::c_void;

#[repr(C)]
struct LV2_Descriptor {
    URI: *const u8,
    instantiate: extern fn (*const LV2_Descriptor, f64, *const u8, *const *const LV2_Feature) -> LV2_Handle,
    connect_port: extern fn (LV2_Handle, u32, raw::c_void) -> raw::c_void,
    activate: extern fn (LV2_Handle) -> raw::c_void,
    run:extern fn (LV2_Handle, u32) -> raw::c_void,
    deactivate:extern fn (LV2_Handle) -> raw::c_void,
    cleanup:extern fn (LV2_Handle) -> raw::c_void,
    extension_data:extern fn (*const u8) -> raw::c_void,
}

#[repr(C)]
struct LV2_Feature {
    URI: *const u8,
    data: raw::c_void
}

const AMP_URI: *const u8 = "http://quaddmg.com/plugins/eg-amp";

const descriptor: LV2_Descriptor = LV2_Descriptor {
    URI: AMP_URI,
    instantiate: instantiate,
    connect_port: connect_port,
    activate: activate,
    run: run,
    deactivate: deactivate,
    cleanup: cleanup,
    extension_data: extension_data
};


#[link(name = "extlib")]
extern "C" {


    fn instantiate(descriptor: *const LV2_Descriptor,
                   rate: *const f64,
                   path: *const u8,
                   features: *const *const LV2_Feature) -> LV2_Handle {
        println!("OMG OMG ");
        let mut amp = Box::new(Amp {});
        amp
    }

    fn connect_port(instance: LV2_Handle, port: u32, data: raw::c_void) -> raw::c_void {
        let mut amp = instance;

        match port {
            AmpGain => instance.gain = data,
            AmpInput => instance.input = data,
            AmpOutput => instance.output = data,

        }
    }

    fn activate(instance: LV2_Handle) -> raw::c_void {
    }

    fn deactivate(instance: LV2_Handle) -> raw::c_void {
    }

    fn run(instance: LV2_Handle, n_samples: u32) -> raw::c_void {
    }

    fn cleanup(instance: LV2_Handle) -> raw::c_void {
        // free instance?
    }

    fn extension_data(uri: *const u8) -> raw::c_void {
        return std::ptr::null_mut();
    }

    fn lv2_descriptor(index: u32) -> LV2_Descriptor {
        match index {
            0 => return &descriptor,
            _ => return std::ptr::null_mut()
        }
    }
}

