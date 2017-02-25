
use lv2_raw::urid::LV2_URID as LV2_URID;

use std::mem;


pub const LV2_ATOM_Blank: *const u8 = b"http://lv2plug.in/ns/ext/atom#Blank\0" as *const u8;
pub const LV2_ATOM_Path: *const u8 = b"http://lv2plug.in/ns/ext/atom#Path\0" as *const u8;
pub const LV2_ATOM_Resource: *const u8 = b"http://lv2plug.in/ns/ext/atom#Resource\0" as *const u8;
pub const LV2_ATOM_Sequence: *const u8 = b"http://lv2plug.in/ns/ext/atom#Sequence\0" as *const u8;
pub const LV2_ATOM_URID: *const u8 = b"http://lv2plug.in/ns/ext/atom#URID\0" as *const u8;
pub const LV2_ATOM_eventTransfer: *const u8 = b"http://lv2plug.in/ns/ext/atom#eventTransfer\0" as *const u8;

#[repr(C)]
pub struct LV2_Atom {
    pub size: u32,
    pub atom_type: u32,
}

// #[repr(C)]
// pub union LV2_Atom_Event_Time {
//     frames: i64,
//     beats: f64,
// }

#[repr(C)]
pub struct LV2_Atom_Event {
    pub time_frames: i64, // LV2_Atom_Event_Time,
    pub body: LV2_Atom,
}

#[repr(C)]
pub struct LV2_Atom_Sequence_Body {
    pub unit: u32,
    pub pad: u32,
}

#[repr(C)]
pub struct LV2_Atom_Sequence {
    pub atom: LV2_Atom,
    pub body: LV2_Atom_Sequence_Body,
}

pub struct AtomSequenceIter {
    pub seq: *const LV2_Atom_Sequence,
    pub next: *const LV2_Atom_Event,
    pub total: usize,
}

pub struct SequenceData {
    pub data_type: LV2_URID,
    pub data: *const u8,
    pub size: usize
}

impl AtomSequenceIter {
    pub fn new(seq: *const LV2_Atom_Sequence) -> AtomSequenceIter {
        unsafe {
            AtomSequenceIter {
                seq: seq,
                next: seq.offset(1) as *const LV2_Atom_Event,
                total: (seq as usize)
                .checked_add((*seq).atom.size as usize)
                .unwrap()
                .checked_add(mem::size_of::<LV2_Atom>())
                .unwrap(),
            }
        }
    }
}

fn pad_size(size: u32) -> usize {
    let seven: usize = 7;

    (size as usize + seven) & !seven
}

impl Iterator for AtomSequenceIter {
    type Item = SequenceData;

    fn next(&mut self) -> Option<SequenceData> {
        if self.next as usize > self.total {
            None
        } else {
            unsafe {
                let seqData = SequenceData {
                    data_type: (*self.next).body.atom_type,
                    data: self.next.offset(1) as *const u8,
                    size: (*self.next).body.size as usize,
                };
                let next_offset: usize = mem::size_of::<LV2_Atom_Event>() + pad_size((*self.next).body.size);
                self.next = ((self.next as usize).checked_add(next_offset as usize).unwrap()) as *const LV2_Atom_Event;
                Some(seqData)
            }
        }
    }
}

