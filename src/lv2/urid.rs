
use std::ffi as ffi;
use std::os::raw as raw;
use lv2_raw::urid::*;

pub fn urid_for_const(map: *const LV2_URID_Map, item: *const u8) -> LV2_URID {
    urid_for_cchar(map, item as *const raw::c_char)
}

pub fn urid_for_cchar(map: *const LV2_URID_Map, item: *const raw::c_char) -> LV2_URID {
    unsafe {
        ((*map).map)((*map).handle, item)
    }
}

pub fn urid_for(map: *const LV2_URID_Map, item: &ffi::CStr) -> LV2_URID {
    urid_for_cchar(map, item.as_ptr())
}

