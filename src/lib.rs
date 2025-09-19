#![no_std]


pub mod interface;
pub mod types;
pub mod instructions;
pub mod characters;

pub use crate::instructions::*;

use core::fmt::{self, Write, Arguments};

const FMT_BUFFER_SIZE:usize = 64;

#[derive(Debug, Copy, Clone)]
pub enum Hd44780Error {
    InterfaceError(interface::InterfaceError),
    FmtError(core::fmt::Error),
    RowColOutOfRange,
}

impl From<core::fmt::Error> for Hd44780Error
{
    fn from(err: core::fmt::Error) -> Self {
        Hd44780Error::FmtError(err)
    }
}

impl From<&Hd44780Error> for &'static str
{
    fn from(err: &Hd44780Error) -> Self {
        match err {
            Hd44780Error::InterfaceError(_) => "Inteface Error",
            Hd44780Error::FmtError(_) => "Formatting Error",
            Hd44780Error::RowColOutOfRange => "Row or Column out of Range"
        }
    }
}


struct Buffer<'a> {
    buf : &'a mut[u8],
    len : usize,
}

impl<'a> Buffer<'a> {
    pub fn new(buf:&'a mut [u8]) -> Self {
        Self { buf, len:0 }
    }

    pub fn as_str(&self) -> &str {
        core::str::from_utf8( &self.buf[0..self.len]).unwrap_or("")
    }
}

impl<'a> Write for Buffer<'a> {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        let b = s.as_bytes();
        if self.len + b.len() > self.buf.len() {
            return Err(fmt::Error);
        }
        self.buf[self.len..self.len+b.len()].clone_from_slice(b);
        self.len += b.len();
        Ok(())
    }
}


pub struct Hd44780<INTERFACE, DPTYPE>
where 
    INTERFACE: interface::InterfaceTrait,
    DPTYPE: types::DisplayTypeTrait,
{
    interface: INTERFACE,
    dp_type: DPTYPE,
}


#[cfg(not(feature="async"))]
impl<'a, INTERFACE, DPTYPE> Hd44780<INTERFACE, DPTYPE>
where
    INTERFACE: interface::InterfaceTrait,
    DPTYPE: types::DisplayTypeTrait,
{
    pub fn new(interface: INTERFACE, dp_type: DPTYPE) -> Self {
        Self {
            interface,
            dp_type,
        }
    }

    pub fn init(&mut self) -> Result<&mut Self, Hd44780Error> {
        self.interface.init(
            self.dp_type.lines(), 
            self.dp_type.font()
        ).map_err(|e| Hd44780Error::InterfaceError(e))?;

        self.display(DpState::On, DpCursor::Off, DpBlink::Off)?;
        self.clear()?;
        Ok(self)
    }

    pub fn clear(&mut self) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Clear as u8
        ).map_err(|e| Hd44780Error::InterfaceError(e))?;
        self.interface.delay_us(1_520);
        Ok(self)
    }

    pub fn home(&mut self) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Home as u8
        ).map_err(|e| Hd44780Error::InterfaceError(e))?;
        self.interface.delay_us(1_520);
        Ok(self)
    }

    pub fn entry(&mut self, dir:EntryDir, ads:EntryAds) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Entry as u8 | dir as u8 | ads as u8
        ).map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub fn display(&mut self, state:DpState, cursor:DpCursor, blink:DpBlink) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Dp as u8 | state as u8 | cursor as u8 | blink as u8
        ).map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub fn shift(&mut self, dp_type:ShiftType, dir:ShiftDir) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Shift as u8 | dp_type as u8 | dir as u8
        ).map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub fn position(&mut self, row:u8, col:u8) -> Result<&mut Self, Hd44780Error> {
        if row >= self.dp_type.rows() || col >= self.dp_type.cols() {
            return Err(Hd44780Error::RowColOutOfRange);
        }

        let dd: u8 = match self.dp_type.lines() {
            FnsetLines::One => self.dp_type.cols() * row + col,
            FnsetLines::Two => 0x40 * (row % 2) + self.dp_type.cols() * (row / 2) + col,
        };
        self.interface.send_byte::<false>(
            CmdOptions::SetDd as u8 | dd
        ).map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub fn print_string(&mut self, string:&str) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_bytes::<true>(
            string.as_bytes()
        ).map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub fn print_fmt(&mut self, args:Arguments<'_>) -> Result<&mut Self, Hd44780Error> {
        let mut data:[u8;FMT_BUFFER_SIZE] = [0;FMT_BUFFER_SIZE];
        let mut buf = Buffer::new(&mut data);
        match buf.write_fmt(args) {
            Err(e) => Err(Hd44780Error::FmtError(e)),
            Ok(_) => self.print_string(buf.as_str())
        }
    }

    pub fn backlight(&mut self, bl:bool) -> Result<&mut Self, Hd44780Error> {
        self.interface.backlight(bl).map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(self)
    }

    pub fn read_data(&mut self, buffer:&mut [u8]) -> Result<&mut Self, Hd44780Error> {
        self.interface.receive_bytes::<true>(buffer).map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(self)
    }

    pub fn read_address_counter(&mut self) -> Result<u8, Hd44780Error> {
        let mut ac: u8 = 0;
        self.interface.receive_byte::<false>(&mut ac).map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(ac & 0x7f)
    }

    pub fn is_busy(&mut self) -> Result<bool, Hd44780Error> {
        let mut ac: u8 = 0;
        self.interface.receive_byte::<false>(&mut ac).map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok((ac & 0x80) != 0)
    }
}

#[cfg(not(feature="async"))]
impl<INTERFACE> Hd44780<INTERFACE, types::DisplayTypeFont5x8>
where
    INTERFACE: interface::InterfaceTrait,
{
    pub fn create_char(&mut self, charcode: characters::CustomFont5x8, charmap:[u8;8]) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::SetCg as u8 | ((charcode as u8) & 0b0000_0111)
        ).map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        self.interface.send_bytes::<true>(&charmap).map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(self)
    }
}

#[cfg(not(feature="async"))]
impl<'a, INTERFACE> Hd44780<INTERFACE, types::DisplayTypeFont5x10>
where
    INTERFACE: interface::InterfaceTrait,
{
    pub fn create_char(&mut self, charcode:characters::CustomFont5x10, charmap:[u8;10]) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::SetCg as u8 | ((charcode as u8) & 0b0000_0110)
        ).map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        self.interface.send_bytes::<true>(&charmap).map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(self)
    }
}


#[macro_export]
#[cfg(not(feature="async"))]
macro_rules! lcd_write {
    ($dp:expr, $($arg:tt)*) => {
        $dp.print_fmt(format_args!($($arg)*))
    };
}



#[cfg(feature="async")]
impl<INTERFACE, DPTYPE> Hd44780<INTERFACE, DPTYPE>
where
    INTERFACE: interface::InterfaceTrait,
    DPTYPE: types::DisplayTypeTrait,
{
    pub fn new(interface: INTERFACE, dp_type: DPTYPE) -> Self {
        Self {
            interface,
            dp_type,
        }
    }

    pub async fn init(&mut self) -> Result<&mut Self, Hd44780Error> {
        self.interface.init(
            self.dp_type.lines(), 
            self.dp_type.font()
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;

        self.display(DpState::On, DpCursor::Off, DpBlink::Off).await?;
        self.clear().await?;
        Ok(self)
    }

    pub async fn clear(&mut self) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Clear as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        self.interface.delay_us(1_520).await;
        Ok(self)
    }

    pub async fn home(&mut self) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Home as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        self.interface.delay_us(1_520).await;
        Ok(self)
    }

    pub async fn entry(&mut self, dir:EntryDir, ads:EntryAds) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Entry as u8 | dir as u8 | ads as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub async fn display(&mut self, state:DpState, cursor:DpCursor, blink:DpBlink) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Dp as u8 | state as u8 | cursor as u8 | blink as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub async fn shift(&mut self, dp_type:ShiftType, dir:ShiftDir) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::Shift as u8 | dp_type as u8 | dir as u8
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub async fn position(&mut self, row:u8, col:u8) -> Result<&mut Self, Hd44780Error> {
        if row >= self.dp_type.rows() || col >= self.dp_type.cols() {
            return Err(Hd44780Error::RowColOutOfRange);
        }

        let dd: u8 = match self.dp_type.lines() {
            FnsetLines::One => self.dp_type.cols() * row + col,
            FnsetLines::Two => 0x40 * (row % 2) + self.dp_type.cols() * (row / 2) + col,
        };
        self.interface.send_byte::<false>(
            CmdOptions::SetDd as u8 | dd
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    pub async fn print_string(&mut self, string:&str) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_bytes::<true>(
            string.as_bytes()
        ).await.map_err(|e| Hd44780Error::InterfaceError(e))?;
        Ok(self)
    }

    
    pub async fn print_fmt(&mut self, args:Arguments<'_>) -> Result<&mut Self, Hd44780Error> {
        let mut data:[u8;FMT_BUFFER_SIZE] = [0;FMT_BUFFER_SIZE];
        let mut buf = Buffer::new(&mut data);
        match buf.write_fmt(args) {
            Err(e) => Err(Hd44780Error::FmtError(e)),
            Ok(_) => self.print_string(buf.as_str()).await
        }
    }

    pub async fn backlight(&mut self, bl:bool) -> Result<&mut Self, Hd44780Error> {
        self.interface.backlight(bl)
        .await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(self)
    }

    pub async fn read_data(&mut self, buffer:&mut [u8]) -> Result<&mut Self, Hd44780Error> {
        self.interface.receive_bytes::<true>(buffer)
        .await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(self)
    }

    pub async fn read_address_counter(&mut self) -> Result<u8, Hd44780Error> {
        let mut ac: u8 = 0;
        self.interface.receive_byte::<false>(&mut ac)
        .await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(ac & 0x7f)
    }

    pub async fn is_busy(&mut self) -> Result<bool, Hd44780Error> {
        let mut ac: u8 = 0;
        self.interface.receive_byte::<false>(&mut ac)
        .await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok((ac & 0x80) != 0)
    }
}

#[cfg(feature="async")]
impl<INTERFACE> Hd44780<INTERFACE, types::DisplayTypeFont5x8>
where
    INTERFACE: interface::InterfaceTrait,
{
    pub async fn create_char(&mut self, charcode: characters::CustomFont5x8, charmap:[u8;8]) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::SetCg as u8 | ((charcode as u8) & 0b0000_0111)
        ).await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        self.interface.send_bytes::<true>(&charmap).await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(self)
    }
}

#[cfg(feature="async")]
impl<INTERFACE> Hd44780<INTERFACE, types::DisplayTypeFont5x10>
where
    INTERFACE: interface::InterfaceTrait,
{
    pub async fn create_char(&mut self, charcode:characters::CustomFont5x10, charmap:[u8;10]) -> Result<&mut Self, Hd44780Error> {
        self.interface.send_byte::<false>(
            CmdOptions::SetCg as u8 | ((charcode as u8) & 0b0000_0110)
        ).await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        self.interface.send_bytes::<true>(
            &charmap
        ).await.map_err(
            |e| Hd44780Error::InterfaceError(e)
        )?;
        Ok(self)
    }
}


#[macro_export]
#[cfg(feature="async")]
macro_rules! lcd_write {
    ($dp:expr, $($arg:tt)*) => {
        $dp.print_fmt(format_args!($($arg)*))
    };
}

