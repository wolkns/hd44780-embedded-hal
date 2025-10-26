

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

/// [`Increments`] or [`Decrements`] the DDRAM address by 1
/// when a character code is written into or read from DDRAM.
/// 
/// The cursor or blinking moves to the right when incremented
/// and to the left when decremented.
/// 
/// The same applies to writing and reading of [`CGRAM`].
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum EntryDir { // increment or decrement address counter (DDRAM or CGRAM/ROM)
    Dec = 0x00,
    Inc = 0x02,
}

/// [`Accompanies display shift`] shifts the entire display either to right
/// or left, depending on [`EntryDir`].
/// 
/// It will seem as if the cursor does not move but the display does.
/// The display won't shift when reading from [`DDRAM`].
/// Also, writing into or reading out from [`CGRAM`] won't shift the dispaly.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum EntryAds { // Accompanies display shift
    Off = 0x00,
    On  = 0x01,
}


/// Chaneges if characters are showen on the display.
/// If display is off, characters will remain in [`DDRAM`].
/// 
/// This won't affect the backlight.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DpState {
    Off = 0x00,
    On  = 0x04,
}

/// Switches Cursor on and off.
/// The cursor is displayed using 5 dots in the 8th line for [`crate::types::DisplayTypeFont5x8`]
/// and in 11th line for [`crate::types::DisplayTypeFont5x10`].
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DpCursor {
    Off = 0x00,
    On  = 0x02,
}

/// The blinking is displayed as switching between all blank dots
/// and showing the character on the display.
/// The [`DpCursor`] and [`DpBlink`] can be set simultaneously.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum DpBlink {
    Off = 0x00,
    On  = 0x01,
}


/// Shifts the cursor position or display depending on [`ShiftDir`].
/// This action is happening without writing or reading display data.
/// 
/// In a [`FnsetLines::Two`] display, the cursor moves to the second line
/// when it passes the [`40th digit`] of the first line.
/// Note that the first and second line of the displays will shift at the same time.
/// 
/// The address counter (AC) contents won't change
/// if the only action performed is a display shift.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ShiftType {
    Cursor  = 0x00,
    Display = 0x08,
}

/// Direction of cursor position shift or display shift.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ShiftDir {
    Left  = 0x00,
    Right = 0x04,
}


/// Function Set Data Length
/// There are 2 modes available for sending and receiveing data:
///  -> 4-bit mode (DB7 - DB4)
///  -> 8-bit mode (DB7 - DB0)
/// In 4-bit mode you have to send and receive twice for eache byte.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum FnsetDataLen {
    Bit4 = 0x00,
    Bit8 = 0x10,
}

/// Sets the number of display lines.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum FnsetLines {
    One = 0x00,
    Two = 0x08,
}

/// Sets the character font.
#[derive(Copy, Clone)]
#[repr(u8)]
pub enum FnsetFont {
    Dots5x8  = 0x00,
    Dots5x10 = 0x04,
}


