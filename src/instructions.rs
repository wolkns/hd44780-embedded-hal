

#[derive(Copy, Clone)]
#[repr(u8)]
pub(crate) enum CmdOptions {
    Clear = 0x01,
    Home  = 0x02,
    Entry = 0x04,
    Dp    = 0x08,
    Shift = 0x10,
    Fnset = 0x20,
    SetCg = 0x40,
    SetDd = 0x80,
}

// Entry parameters
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum EntryDir { // increment or decrement address counter (DDRAM or CGRAM/ROM)
    Dec = 0x00,
    Inc = 0x02,
}
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum EntryAds { // Accompanies display shift
    Off = 0x00,
    On  = 0x01,
}

// Display parameters
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DpState {
    Off = 0x00,
    On  = 0x04,
}
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DpCursor {
    Off = 0x00,
    On  = 0x02,
}
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DpBlink {
    Off = 0x00,
    On  = 0x01,
}

// Shift Parameters
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ShiftType {
    Cursor  = 0x00,
    Display = 0x08,
}
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ShiftDir {
    Left  = 0x00,
    Right = 0x04,
}

// Fnset Parameters
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum FnsetDataLen {
    Bit4 = 0x00,
    Bit8 = 0x10,
}
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum FnsetLines {
    One = 0x00,
    Two = 0x08,
}
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum FnsetFont {
    Dots5x8  = 0x00,
    Dots5x10 = 0x04,
}


