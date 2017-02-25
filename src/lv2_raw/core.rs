
use std::os::raw as raw;

pub type LV2_Handle = *mut raw::c_void;

#[repr(C)]
pub struct LV2_Feature {
    pub URI: *const raw::c_char,
    pub data: *mut raw::c_void
}

#[repr(C)]
pub struct LV2_Descriptor {
    pub URI: *const raw::c_char,
    pub instantiate: extern fn (*const LV2_Descriptor, f64, *const raw::c_char, *const *const LV2_Feature) -> LV2_Handle,
    pub connect_port: extern fn (LV2_Handle, u32, *mut raw::c_void),
    pub activate: extern fn (LV2_Handle),
    pub run:extern fn (LV2_Handle, u32),
    pub deactivate:extern fn (LV2_Handle),
    pub cleanup:extern fn (LV2_Handle),
    pub extension_data:extern fn (*const raw::c_char) -> *mut raw::c_void,
}

