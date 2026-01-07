#![no_std]
#![allow(async_fn_in_trait)]


pub mod interface;
pub mod types;
pub mod characters;


mod instructions;
pub use crate::instructions::{
    EntryDir,
    EntryAds,

    DpState,
    DpCursor,
    DpBlink,

    ShiftType,
    ShiftDir,

    FnsetDataLen,
    FnsetLines,
    FnsetFont
};


#[cfg(not(feature="async"))]
mod blocking;
#[cfg(not(feature="async"))]
pub use crate::blocking::Hd44780Trait;


#[cfg(feature="async")]
mod asyncron;
#[cfg(feature="async")]
pub use crate::asyncron::Hd44780Trait;

mod buffer;


pub struct Hd44780<INTERFACE, DPTYPE>
where 
    INTERFACE: interface::InterfaceTrait,
    DPTYPE: types::DisplayTypeTrait,
{
    interface: INTERFACE,
    dp_type: DPTYPE,
}

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
}


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


#[macro_export]
macro_rules! lcd_write {
    ($dp:expr, $($arg:tt)*) => {
        $dp.print_fmt(format_args!($($arg)*))
    };
}


