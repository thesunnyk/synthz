
use lv2_raw::midi;
use lv2_raw::urid;
use lv2::atom;

#[derive(Debug)]
pub enum MidiEvent {
    AfterTouch { note_num: u8, pressure: u8 },
    Bender { value: u16 },
    ChannelPressure { pressure: u8 },
    Controller { controller_num: u8, controller_val: u8 },
    NoteOn { note_num: u8, velocity: u8 },
    NoteOff { note_num: u8, velocity: u8 },
    ProgramChange { num: u8 },
    Unknown
}

impl MidiEvent {
    pub fn new(data: *const u8, size: usize) -> MidiEvent {
        unsafe {
            match *data {
                midi::LV2_MIDI_MSG_NOTE_PRESSURE => {
                    assert_eq!(size, 3);
                    MidiEvent::AfterTouch {
                        note_num: *data.offset(1),
                        pressure: *data.offset(2),
                    }
                },
                midi::LV2_MIDI_MSG_BENDER => {
                    assert_eq!(size, 3);
                    MidiEvent::Bender {
                        value: *data.offset(1) as u16,
                    }
                },
                midi::LV2_MIDI_MSG_CHANNEL_PRESSURE => {
                    assert_eq!(size, 3);
                    MidiEvent::ChannelPressure {
                        pressure: *data.offset(1),
                    }
                },
                midi::LV2_MIDI_MSG_CONTROLLER => {
                    assert_eq!(size, 3);
                    MidiEvent::Controller {
                        controller_num: *data.offset(1),
                        controller_val: *data.offset(2),
                    }
                },
                midi::LV2_MIDI_MSG_NOTE_ON => {
                    assert_eq!(size, 3);
                    MidiEvent::NoteOn {
                        note_num: *data.offset(1),
                        velocity: *data.offset(2),
                    }
                },
                midi::LV2_MIDI_MSG_NOTE_OFF => {
                    assert_eq!(size, 3);
                    MidiEvent::NoteOff {
                        note_num: *data.offset(1),
                        velocity: *data.offset(2),
                    }
                },
                midi::LV2_MIDI_MSG_PGM_CHANGE => {
                    assert_eq!(size, 2);
                    MidiEvent::ProgramChange {
                        num: *data.offset(1),
                    }
                },
                _ => {
                    MidiEvent::Unknown
                }
            }

        }
    }
}

