
use std::os::raw as raw;
use lv2_raw::atom::*;
use lv2_raw::urid::*;

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

