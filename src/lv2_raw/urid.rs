
use std::os::raw as raw;

pub type LV2_URID_Map_Handle = *mut raw::c_void;

pub type LV2_URID = u32;

pub const LV2_URID_map: *const u8 = b"http://lv2plug.in/ns/ext/urid#map\0" as *const u8;

// TODO Move to lv2_raw::patch.rs

pub const LV2_PATCH_Set: *const u8 = b"http://lv2plug.in/ns/ext/patch#Set\0" as *const u8;
pub const LV2_PATCH_property: *const u8 = b"http://lv2plug.in/ns/ext/patch#property\0" as *const u8;
pub const LV2_PATCH_value: *const u8 = b"http://lv2plug.in/ns/ext/patch#value\0" as *const u8;


#[repr(C)]
pub struct LV2_URID_Map {
    pub handle: LV2_URID_Map_Handle,
    pub map: extern fn(LV2_URID_Map_Handle, *const raw::c_char) -> LV2_URID,
}

