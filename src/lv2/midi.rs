
use lv2_raw::midi;
use lv2_raw::urid;
use lv2::atom;

pub struct MidiData {
    pub time_frames: i64,
    pub status: u8,
    pub pitch: u8,
    pub velocity: u8,
}

impl MidiData {
    pub fn new(data: *const u8, size: usize, time_frames: i64) -> MidiData {
        assert_eq!(3, size);
        unsafe {
            MidiData {
                time_frames: time_frames,
                status: *data,
                pitch: *data.offset(1),
                velocity: *data.offset(2),
            }
        }
    }
}

