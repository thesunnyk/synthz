
use lv2_raw::midi;
use lv2_raw::urid;
use lv2::atom;

pub struct MidiData {
    pub time_frames: i64,
    pub status: u8,
    pub pitch: u8,
    pub velocity: u8,
}

pub struct MidiEventExtractor {
    midi_event_urid: urid::LV2_URID,
    pub midi_data: Vec<MidiData>
}

impl MidiEventExtractor {
    pub fn new(midiEvent_urid: urid::LV2_URID) -> MidiEventExtractor {
        MidiEventExtractor {
            midi_event_urid: midiEvent_urid,
            midi_data: Vec::new(),
        }
    }
}

impl atom::EventExtractor for MidiEventExtractor {
    fn matches(&self, current: urid::LV2_URID) -> bool {
        self.midi_event_urid == current
    }

    fn store(&mut self, data: *const u8, size: usize, time_frames: i64) {
        assert_eq!(3, size);
        unsafe {
            self.midi_data.push(MidiData {
                time_frames: time_frames,
                status: *data,
                pitch: *data.offset(1),
                velocity: *data.offset(2),
            });
        }
    }
}

