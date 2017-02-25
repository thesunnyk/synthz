
type LV2_Midi_Message_Type = u8;

const LV2_MIDI_MSG_INVALID: LV2_Midi_Message_Type          = 0;     // Invalid Message
const LV2_MIDI_MSG_NOTE_OFF: LV2_Midi_Message_Type         = 0x80;  // Note Off
const LV2_MIDI_MSG_NOTE_ON: LV2_Midi_Message_Type          = 0x90;  // Note On
const LV2_MIDI_MSG_NOTE_PRESSURE: LV2_Midi_Message_Type    = 0xA0;  // Note Pressure
const LV2_MIDI_MSG_CONTROLLER: LV2_Midi_Message_Type       = 0xB0;  // Controller
const LV2_MIDI_MSG_PGM_CHANGE: LV2_Midi_Message_Type       = 0xC0;  // Program Change
const LV2_MIDI_MSG_CHANNEL_PRESSURE: LV2_Midi_Message_Type = 0xD0;  // Channel Pressure
const LV2_MIDI_MSG_BENDER: LV2_Midi_Message_Type           = 0xE0;  // Pitch Bender
const LV2_MIDI_MSG_SYSTEM_EXCLUSIVE: LV2_Midi_Message_Type = 0xF0;  // System Exclusive Begin
const LV2_MIDI_MSG_MTC_QUARTER: LV2_Midi_Message_Type      = 0xF1;  // MTC Quarter Frame
const LV2_MIDI_MSG_SONG_POS: LV2_Midi_Message_Type         = 0xF2;  // Song Position
const LV2_MIDI_MSG_SONG_SELECT: LV2_Midi_Message_Type      = 0xF3;  // Song Select
const LV2_MIDI_MSG_TUNE_REQUEST: LV2_Midi_Message_Type     = 0xF6;  // Tune Request
const LV2_MIDI_MSG_CLOCK: LV2_Midi_Message_Type            = 0xF8;  // Clock
const LV2_MIDI_MSG_START: LV2_Midi_Message_Type            = 0xFA;  // Start
const LV2_MIDI_MSG_CONTINUE: LV2_Midi_Message_Type         = 0xFB;  // Continue
const LV2_MIDI_MSG_STOP: LV2_Midi_Message_Type             = 0xFC;  // Stop
const LV2_MIDI_MSG_ACTIVE_SENSE: LV2_Midi_Message_Type     = 0xFE;  // Active Sensing
const LV2_MIDI_MSG_RESET: LV2_Midi_Message_Type            = 0xFF;  // Reset

type LV2_Midi_Controller = u8;

const LV2_MIDI_CTL_MSB_BANK: LV2_Midi_Controller             = 0x00;  // Bank Selection
const LV2_MIDI_CTL_MSB_MODWHEEL: LV2_Midi_Controller         = 0x01;  // Modulation
const LV2_MIDI_CTL_MSB_BREATH: LV2_Midi_Controller           = 0x02;  // Breath
const LV2_MIDI_CTL_MSB_FOOT: LV2_Midi_Controller             = 0x04;  // Foot
const LV2_MIDI_CTL_MSB_PORTAMENTO_TIME: LV2_Midi_Controller  = 0x05;  // Portamento Time
const LV2_MIDI_CTL_MSB_DATA_ENTRY: LV2_Midi_Controller       = 0x06;  // Data Entry
const LV2_MIDI_CTL_MSB_MAIN_VOLUME: LV2_Midi_Controller      = 0x07;  // Main Volume
const LV2_MIDI_CTL_MSB_BALANCE: LV2_Midi_Controller          = 0x08;  // Balance
const LV2_MIDI_CTL_MSB_PAN: LV2_Midi_Controller              = 0x0A;  // Panpot
const LV2_MIDI_CTL_MSB_EXPRESSION: LV2_Midi_Controller       = 0x0B;  // Expression
const LV2_MIDI_CTL_MSB_EFFECT1: LV2_Midi_Controller          = 0x0C;  // Effect1
const LV2_MIDI_CTL_MSB_EFFECT2: LV2_Midi_Controller          = 0x0D;  // Effect2
const LV2_MIDI_CTL_MSB_GENERAL_PURPOSE1: LV2_Midi_Controller = 0x10;  // General Purpose 1
const LV2_MIDI_CTL_MSB_GENERAL_PURPOSE2: LV2_Midi_Controller = 0x11;  // General Purpose 2
const LV2_MIDI_CTL_MSB_GENERAL_PURPOSE3: LV2_Midi_Controller = 0x12;  // General Purpose 3
const LV2_MIDI_CTL_MSB_GENERAL_PURPOSE4: LV2_Midi_Controller = 0x13;  // General Purpose 4
const LV2_MIDI_CTL_LSB_BANK: LV2_Midi_Controller             = 0x20;  // Bank Selection
const LV2_MIDI_CTL_LSB_MODWHEEL: LV2_Midi_Controller         = 0x21;  // Modulation
const LV2_MIDI_CTL_LSB_BREATH: LV2_Midi_Controller           = 0x22;  // Breath
const LV2_MIDI_CTL_LSB_FOOT: LV2_Midi_Controller             = 0x24;  // Foot
const LV2_MIDI_CTL_LSB_PORTAMENTO_TIME: LV2_Midi_Controller  = 0x25;  // Portamento Time
const LV2_MIDI_CTL_LSB_DATA_ENTRY: LV2_Midi_Controller       = 0x26;  // Data Entry
const LV2_MIDI_CTL_LSB_MAIN_VOLUME: LV2_Midi_Controller      = 0x27;  // Main Volume
const LV2_MIDI_CTL_LSB_BALANCE: LV2_Midi_Controller          = 0x28;  // Balance
const LV2_MIDI_CTL_LSB_PAN: LV2_Midi_Controller              = 0x2A;  // Panpot
const LV2_MIDI_CTL_LSB_EXPRESSION: LV2_Midi_Controller       = 0x2B;  // Expression
const LV2_MIDI_CTL_LSB_EFFECT1: LV2_Midi_Controller          = 0x2C;  // Effect1
const LV2_MIDI_CTL_LSB_EFFECT2: LV2_Midi_Controller          = 0x2D;  // Effect2
const LV2_MIDI_CTL_LSB_GENERAL_PURPOSE1: LV2_Midi_Controller = 0x30;  // General Purpose 1
const LV2_MIDI_CTL_LSB_GENERAL_PURPOSE2: LV2_Midi_Controller = 0x31;  // General Purpose 2
const LV2_MIDI_CTL_LSB_GENERAL_PURPOSE3: LV2_Midi_Controller = 0x32;  // General Purpose 3
const LV2_MIDI_CTL_LSB_GENERAL_PURPOSE4: LV2_Midi_Controller = 0x33;  // General Purpose 4
const LV2_MIDI_CTL_SUSTAIN: LV2_Midi_Controller              = 0x40;  // Sustain Pedal
const LV2_MIDI_CTL_PORTAMENTO: LV2_Midi_Controller           = 0x41;  // Portamento
const LV2_MIDI_CTL_SOSTENUTO: LV2_Midi_Controller            = 0x42;  // Sostenuto
const LV2_MIDI_CTL_SOFT_PEDAL: LV2_Midi_Controller           = 0x43;  // Soft Pedal
const LV2_MIDI_CTL_LEGATO_FOOTSWITCH: LV2_Midi_Controller    = 0x44;  // Legato Foot Switch
const LV2_MIDI_CTL_HOLD2: LV2_Midi_Controller                = 0x45;  // Hold2
const LV2_MIDI_CTL_SC1_SOUND_VARIATION: LV2_Midi_Controller  = 0x46;  // SC1 Sound Variation
const LV2_MIDI_CTL_SC2_TIMBRE: LV2_Midi_Controller           = 0x47;  // SC2 Timbre
const LV2_MIDI_CTL_SC3_RELEASE_TIME: LV2_Midi_Controller     = 0x48;  // SC3 Release Time
const LV2_MIDI_CTL_SC4_ATTACK_TIME: LV2_Midi_Controller      = 0x49;  // SC4 Attack Time
const LV2_MIDI_CTL_SC5_BRIGHTNESS: LV2_Midi_Controller       = 0x4A;  // SC5 Brightness
const LV2_MIDI_CTL_SC6: LV2_Midi_Controller                  = 0x4B;  // SC6
const LV2_MIDI_CTL_SC7: LV2_Midi_Controller                  = 0x4C;  // SC7
const LV2_MIDI_CTL_SC8: LV2_Midi_Controller                  = 0x4D;  // SC8
const LV2_MIDI_CTL_SC9: LV2_Midi_Controller                  = 0x4E;  // SC9
const LV2_MIDI_CTL_SC10: LV2_Midi_Controller                 = 0x4F;  // SC10
const LV2_MIDI_CTL_GENERAL_PURPOSE5: LV2_Midi_Controller     = 0x50;  // General Purpose 5
const LV2_MIDI_CTL_GENERAL_PURPOSE6: LV2_Midi_Controller     = 0x51;  // General Purpose 6
const LV2_MIDI_CTL_GENERAL_PURPOSE7: LV2_Midi_Controller     = 0x52;  // General Purpose 7
const LV2_MIDI_CTL_GENERAL_PURPOSE8: LV2_Midi_Controller     = 0x53;  // General Purpose 8
const LV2_MIDI_CTL_PORTAMENTO_CONTROL: LV2_Midi_Controller   = 0x54;  // Portamento Control
const LV2_MIDI_CTL_E1_REVERB_DEPTH: LV2_Midi_Controller      = 0x5B;  // E1 Reverb Depth
const LV2_MIDI_CTL_E2_TREMOLO_DEPTH: LV2_Midi_Controller     = 0x5C;  // E2 Tremolo Depth
const LV2_MIDI_CTL_E3_CHORUS_DEPTH: LV2_Midi_Controller      = 0x5D;  // E3 Chorus Depth
const LV2_MIDI_CTL_E4_DETUNE_DEPTH: LV2_Midi_Controller      = 0x5E;  // E4 Detune Depth
const LV2_MIDI_CTL_E5_PHASER_DEPTH: LV2_Midi_Controller      = 0x5F;  // E5 Phaser Depth
const LV2_MIDI_CTL_DATA_INCREMENT: LV2_Midi_Controller       = 0x60;  // Data Increment
const LV2_MIDI_CTL_DATA_DECREMENT: LV2_Midi_Controller       = 0x61;  // Data Decrement
const LV2_MIDI_CTL_NRPN_LSB: LV2_Midi_Controller             = 0x62;  // Non-registered Parameter Number
const LV2_MIDI_CTL_NRPN_MSB: LV2_Midi_Controller             = 0x63;  // Non-registered Parameter Number
const LV2_MIDI_CTL_RPN_LSB: LV2_Midi_Controller              = 0x64;  // Registered Parameter Number
const LV2_MIDI_CTL_RPN_MSB: LV2_Midi_Controller              = 0x65;  // Registered Parameter Number
const LV2_MIDI_CTL_ALL_SOUNDS_OFF: LV2_Midi_Controller       = 0x78;  // All Sounds Off
const LV2_MIDI_CTL_RESET_CONTROLLERS: LV2_Midi_Controller    = 0x79;  // Reset Controllers
const LV2_MIDI_CTL_LOCAL_CONTROL_SWITCH: LV2_Midi_Controller = 0x7A;  // Local Control Switch
const LV2_MIDI_CTL_ALL_NOTES_OFF: LV2_Midi_Controller        = 0x7B;  // All Notes Off
const LV2_MIDI_CTL_OMNI_OFF: LV2_Midi_Controller             = 0x7C;  // Omni Off
const LV2_MIDI_CTL_OMNI_ON: LV2_Midi_Controller              = 0x7D;  // Omni On
const LV2_MIDI_CTL_MONO1: LV2_Midi_Controller                = 0x7E;  // Mono1
const LV2_MIDI_CTL_MONO2: LV2_Midi_Controller                = 0x7F;  // Mono2


pub fn midi_is_voice_message(msg: *const u8) -> bool {
    unsafe {
        *msg >= 0x80 && *msg < 0xF0
    }
}

pub fn midi_is_system_message(msg: *const u8) -> bool {
    unsafe {
        match *msg {
            0xF4 => false,
            0xF5 => false,
            0xF7 => false,
            0xF9 => false,
            0xFD => false,
            _ => (*msg & 0xF0) == 0xF0,
        }
    }
}

