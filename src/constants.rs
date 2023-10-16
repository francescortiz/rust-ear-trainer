pub mod midi_messages {

    pub const NOTE_OFF_MSG: u8 = 0x80;
    pub const NOTE_ON_MSG: u8 = 0x90;
    pub const AFTERTOUCH_MSG: u8 = 0xA0;
    pub const CONTINUOUS_CONTROLLER_MSG: u8 = 0xB0;
    pub const PATCH_CHANGE_MSG: u8 = 0xC0;
    pub const CHANNEL_PRESSURE_MSG: u8 = 0xD0;
    pub const PITCH_BEND_MSG: u8 = 0xE0;

    // command	meaning	# param
    // 0xF0	start of system exclusive message	variable
    // 0xF1	MIDI Time Code Quarter Frame (Sys Common)
    // 0xF2	Song Position Pointer (Sys Common)
    // 0xF3	Song Select (Sys Common)
    // 0xF4	???
    // 0xF5	???
    // 0xF6	Tune Request (Sys Common)
    // 0xF7	end of system exclusive message	0
    // 0xF8	Timing Clock (Sys Realtime)
    // 0xFA	Start (Sys Realtime)
    // 0xFB	Continue (Sys Realtime)
    // 0xFC	Stop (Sys Realtime)
    // 0xFD	???
    // 0xFE	Active Sensing (Sys Realtime)
    // 0xFF	System Reset (Sys Realtime)
}

pub mod notes {
    pub const C_MINUS_1: u8 = 0;
    pub const C_SHARP_MINUS_1: u8 = 1;
    pub const D_MINUS_1: u8 = 2;
    pub const D_SHARP_MINUS_1: u8 = 3;
    pub const E_MINUS_1: u8 = 4;
    pub const F_MINUS_1: u8 = 5;
    pub const F_SHARP_MINUS_1: u8 = 6;
    pub const G_MINUS_1: u8 = 7;
    pub const G_SHARP_MINUS_1: u8 = 8;
    pub const A_MINUS_1: u8 = 9;
    pub const A_SHARP_MINUS_1: u8 = 10;
    pub const B_MINUS_1: u8 = 11;

    pub const C0: u8 = 12;
    pub const C_SHARP0: u8 = 13;
    pub const D0: u8 = 14;
    pub const D_SHARP0: u8 = 15;
    pub const E0: u8 = 16;
    pub const F0: u8 = 17;
    pub const F_SHARP0: u8 = 18;
    pub const G0: u8 = 19;
    pub const G_SHARP0: u8 = 20;
    pub const A0: u8 = 21;
    pub const A_SHARP0: u8 = 22;
    pub const B0: u8 = 23;

    pub const C1: u8 = 24;
    pub const C_SHARP1: u8 = 25;
    pub const D1: u8 = 26;
    pub const D_SHARP1: u8 = 27;
    pub const E1: u8 = 28;
    pub const F1: u8 = 29;
    pub const F_SHARP1: u8 = 30;
    pub const G1: u8 = 31;
    pub const G_SHARP1: u8 = 32;
    pub const A1: u8 = 33;
    pub const A_SHARP1: u8 = 34;
    pub const B1: u8 = 35;

    pub const MIDI_NOTE_NAMES: [(u8, &str); 128] = [
        (0, "C-1"),
        (1, "C#-1"),
        (2, "D-1"),
        (3, "D#-1"),
        (4, "E-1"),
        (5, "F-1"),
        (6, "F#-1"),
        (7, "G-1"),
        (8, "G#-1"),
        (9, "A-1"),
        (10, "A#-1"),
        (11, "B-1"),
        (12, "C0"),
        (13, "C#0"),
        (14, "D0"),
        (15, "D#0"),
        (16, "E0"),
        (17, "F0"),
        (18, "F#0"),
        (19, "G0"),
        (20, "G#0"),
        (21, "A0"),
        (22, "A#0"),
        (23, "B0"),
        (24, "C1"),
        (25, "C#1"),
        (26, "D1"),
        (27, "D#1"),
        (28, "E1"),
        (29, "F1"),
        (30, "F#1"),
        (31, "G1"),
        (32, "G#1"),
        (33, "A1"),
        (34, "A#1"),
        (35, "B1"),
        (36, "C2"),
        (37, "C#2"),
        (38, "D2"),
        (39, "D#2"),
        (40, "E2"),
        (41, "F2"),
        (42, "F#2"),
        (43, "G2"),
        (44, "G#2"),
        (45, "A2"),
        (46, "A#2"),
        (47, "B2"),
        (48, "C3"),
        (49, "C#3"),
        (50, "D3"),
        (51, "D#3"),
        (52, "E3"),
        (53, "F3"),
        (54, "F#3"),
        (55, "G3"),
        (56, "G#3"),
        (57, "A3"),
        (58, "A#3"),
        (59, "B3"),
        (60, "C4"),
        (61, "C#4"),
        (62, "D4"),
        (63, "D#4"),
        (64, "E4"),
        (65, "F4"),
        (66, "F#4"),
        (67, "G4"),
        (68, "G#4"),
        (69, "A4"),
        (70, "A#4"),
        (71, "B4"),
        (72, "C5"),
        (73, "C#5"),
        (74, "D5"),
        (75, "D#5"),
        (76, "E5"),
        (77, "F5"),
        (78, "F#5"),
        (79, "G5"),
        (80, "G#5"),
        (81, "A5"),
        (82, "A#5"),
        (83, "B5"),
        (84, "C6"),
        (85, "C#6"),
        (86, "D6"),
        (87, "D#6"),
        (88, "E6"),
        (89, "F6"),
        (90, "F#6"),
        (91, "G6"),
        (92, "G#6"),
        (93, "A6"),
        (94, "A#6"),
        (95, "B6"),
        (96, "C7"),
        (97, "C#7"),
        (98, "D7"),
        (99, "D#7"),
        (100, "E7"),
        (101, "F7"),
        (102, "F#7"),
        (103, "G7"),
        (104, "G#7"),
        (105, "A7"),
        (106, "A#7"),
        (107, "B7"),
        (108, "C8"),
        (109, "C#8"),
        (110, "D8"),
        (111, "D#8"),
        (112, "E8"),
        (113, "F8"),
        (114, "F#8"),
        (115, "G8"),
        (116, "G#8"),
        (117, "A8"),
        (118, "A#8"),
        (119, "B8"),
        (120, "C9"),
        (121, "C#9"),
        (122, "D9"),
        (123, "D#9"),
        (124, "E9"),
        (125, "F9"),
        (126, "F#9"),
        (127, "G9"),
    ];
}
