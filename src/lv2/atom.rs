
use std::mem;
use lv2_raw::atom::*;
use lv2_raw::urid::LV2_URID as LV2_URID;

pub struct AtomSequenceIter {
    pub seq: *const LV2_Atom_Sequence,
    pub next: *const LV2_Atom_Event,
    pub total: usize,
}

pub struct SequenceData {
    pub data_type: LV2_URID,
    pub time_frames: i64, // LV2_Atom_Event_Time,
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
                    time_frames: (*self.next).time_frames,
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

pub trait EventExtractor {
    fn matches(&self, urid: LV2_URID) -> bool;
    fn store(&mut self, data: *const u8, size: usize, time_frames: i64);
}

// TODO Use BTreeMap
pub fn extract_sequence(seq: *const LV2_Atom_Sequence, mut extractors: Vec<&mut EventExtractor>) {

    let iter: AtomSequenceIter = AtomSequenceIter::new(seq);

    for event in iter {
        for extractor in &mut extractors {
            if extractor.matches(event.data_type) {
                extractor.store(event.data, event.size, event.time_frames);
            }
        }
    }
}

