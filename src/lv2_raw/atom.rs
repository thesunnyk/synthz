
pub const LV2_ATOM_Blank: *const u8 = b"http://lv2plug.in/ns/ext/atom#Blank\0" as *const u8;
pub const LV2_ATOM_Float: *const u8 = b"http://lv2plug.in/ns/ext/atom#Float\0" as *const u8;
pub const LV2_ATOM_Literal: *const u8 = b"http://lv2plug.in/ns/ext/atom#Literal\0" as *const u8;
pub const LV2_ATOM_Int: *const u8 = b"http://lv2plug.in/ns/ext/atom#Int\0" as *const u8;
pub const LV2_ATOM_Long: *const u8 = b"http://lv2plug.in/ns/ext/atom#Long\0" as *const u8;
pub const LV2_ATOM_Object: *const u8 = b"http://lv2plug.in/ns/ext/atom#Object\0" as *const u8;
pub const LV2_ATOM_Path: *const u8 = b"http://lv2plug.in/ns/ext/atom#Path\0" as *const u8;
pub const LV2_ATOM_Property: *const u8 = b"http://lv2plug.in/ns/ext/atom#Property\0" as *const u8;
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

#[repr(C)]
pub struct LV2_Atom_Property_Body {
    pub key: u32,
    pub context: u32,
    pub value: LV2_Atom,
}

#[repr(C)]
pub struct LV2_Atom_Property {
    pub atom: LV2_Atom,
    pub body: LV2_Atom_Property_Body,
}

#[repr(C)]
pub struct LV2_Atom_Object_Body {
    pub id: u32,
    pub otype: u32,
}

#[repr(C)]
pub struct LV2_Atom_Object {
    pub atom: LV2_Atom,
    pub body: LV2_Atom_Object_Body,
}

