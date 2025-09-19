
use crate::instructions::{FnsetLines, FnsetFont};



pub trait DisplayTypeTrait
{
    fn rows(&self) -> u8;
    fn cols(&self) -> u8;
    fn lines(&self) -> FnsetLines;
    fn font(&self) -> FnsetFont;
}


pub struct DisplayTypeFont5x8
{
    rows: u8,
    cols: u8,
    lines: FnsetLines,
}
impl DisplayTypeFont5x8
{
    pub fn new(rows:u8, cols:u8, lines:FnsetLines) -> Self
    {
        Self {rows, cols, lines}
    }
}
impl DisplayTypeTrait for DisplayTypeFont5x8
{
    fn rows(&self) -> u8 {
        self.rows
    }
    fn cols(&self) -> u8 {
        self.cols
    }
    fn lines(&self) -> FnsetLines {
        self.lines
    }
    fn font(&self) -> FnsetFont {
        FnsetFont::Dots5x8
    }
}


pub struct DisplayTypeFont5x10
{
    rows: u8,
    cols: u8,
}
impl DisplayTypeFont5x10
{
    pub fn new(rows:u8, cols:u8) -> Self
    {
        Self { rows, cols }
    }
}
impl DisplayTypeTrait for DisplayTypeFont5x10
{
    fn rows(&self) -> u8 {
        self.rows
    }
    fn cols(&self) -> u8 {
        self.cols
    }
    fn lines(&self) -> FnsetLines {
        FnsetLines::One
    }
    fn font(&self) -> FnsetFont {
        FnsetFont::Dots5x10
    }
}

